use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, 
    storable::Bound, 
    Storable, Memory as MemoryTrait,
};
use std::cell::RefCell;
use candid::Principal;
use crate::types::{Document, Institution, UserProfile, StorageStats, CleanupResult};
use std::borrow::Cow;
use crate::logging::{get_logger, get_severity_for_event_type};

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Constants for storage limits
const MAX_STRING_KEY_SIZE: usize = 1000;
const MAX_PRINCIPAL_SIZE: usize = 29;

// Helper function for logging serialization errors
fn log_serialization_error(type_name: &str, error: &impl std::fmt::Display) {
    let logger = get_logger("storage");
    let severity = get_severity_for_event_type("SERIALIZATION_ERROR");
    logger.log(severity, "SERIALIZATION_ERROR", &format!("Failed to serialize {}: {}", type_name, error), Some(error.to_string()));
}

// Helper function for logging deserialization errors
fn log_deserialization_error(type_name: &str, error: &impl std::fmt::Display, data_preview: &str) {
    let logger = get_logger("storage");
    let severity = get_severity_for_event_type("DESERIALIZATION_ERROR");
    logger.log(severity, "DESERIALIZATION_ERROR", &format!("Failed to deserialize {}: {}", type_name, error), Some(error.to_string()));
    
    let severity = get_severity_for_event_type("CORRUPTED_DATA");
    logger.log(severity, "CORRUPTED_DATA", &format!("Corrupted {} data (first 100 bytes): {}", type_name, data_preview), None);
}

// Macro to implement Storable with consistent error handling
macro_rules! impl_storable_with_logging {
    ($type:ty, $wrapper:ty, $constructor:expr, $default_constructor:expr) => {
        impl Storable for $wrapper {
            fn to_bytes(&self) -> Cow<[u8]> {
                match bincode::serialize(&self.0) {
                    Ok(bytes) => Cow::Owned(bytes),
                    Err(e) => {
                        log_serialization_error(stringify!($type), &e);
                        Cow::Owned(Vec::new())
                    }
                }
            }

            fn from_bytes(bytes: Cow<[u8]>) -> Self {
                if bytes.is_empty() {
                    let logger = get_logger("storage");
                    let severity = get_severity_for_event_type("CORRUPTED_DATA");
                    logger.log(severity, "CORRUPTED_DATA", &format!("Attempted to deserialize empty bytes - returning default {}", stringify!($type)), None);
                    return $default_constructor;
                }
                
                match bincode::deserialize::<$type>(&bytes) {
                    Ok(data) => $constructor(data),
                    Err(e) => {
                        let data_preview = format!("{:?}", &bytes[..std::cmp::min(100, bytes.len())]);
                        log_deserialization_error(stringify!($type), &e, &data_preview);
                        $default_constructor
                    }
                }
            }

            const BOUND: Bound = Bound::Unbounded;
        }
    };
}

// Wrapper types that implement Storable for stable storage
#[derive(Clone)]
pub struct StorableDocument(pub Document);

#[derive(Clone)]
pub struct StorableInstitution(pub Institution);

#[derive(Clone)]
pub struct StorableUserProfile(pub UserProfile);

// Implement Storable for Document wrapper using macro
impl_storable_with_logging!(Document, StorableDocument, StorableDocument, StorableDocument(crate::types::Document::default()));

// Implement Storable for Institution wrapper using macro
impl_storable_with_logging!(Institution, StorableInstitution, StorableInstitution, StorableInstitution(crate::types::Institution::default()));

// Implement Storable for UserProfile wrapper using macro
impl_storable_with_logging!(
    UserProfile, 
    StorableUserProfile, 
    StorableUserProfile,
    StorableUserProfile(UserProfile {
        internet_identity: Principal::anonymous(),
        name: String::new(),
        email: String::new(),
        role: crate::types::UserRole::RegularUser,
        assigned_institution_id: String::new(),
        created_at: 0,
        last_login: 0,
    })
);

// Wrapper type for String keys
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StorableString(pub String);

impl Storable for StorableString {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Borrowed(self.0.as_bytes())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        match String::from_utf8(bytes.to_vec()) {
            Ok(string) => StorableString(string),
            Err(e) => {
                let logger = get_logger("storage");
                let severity = get_severity_for_event_type("DESERIALIZATION_ERROR");
                logger.log(severity, "DESERIALIZATION_ERROR", &format!("Failed to deserialize string: {}", e), Some(e.to_string()));
                // Return empty string instead of trapping to avoid init mode issues
                StorableString(String::new())
            }
        }
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_STRING_KEY_SIZE as u32,
        is_fixed_size: false,
    };
}

// Implement Storable for Principal wrapper
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StorablePrincipal(pub Principal);

impl Storable for StorablePrincipal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Borrowed(self.0.as_slice())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        match Principal::try_from_slice(&bytes) {
            Ok(principal) => StorablePrincipal(principal),
            Err(e) => {
                let logger = get_logger("storage");
                let severity = get_severity_for_event_type("DESERIALIZATION_ERROR");
                logger.log(severity, "DESERIALIZATION_ERROR", &format!("Failed to deserialize principal: {:?}", e), Some(format!("{:?}", e)));
                // Return anonymous principal instead of trapping to avoid init mode issues
                StorablePrincipal(Principal::anonymous())
            }
        }
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_PRINCIPAL_SIZE as u32,
        is_fixed_size: false,
    };
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Store complete documents using proper Storable types
    pub static DOCUMENTS: RefCell<StableBTreeMap<StorableString, StorableDocument, Memory>> = RefCell::new(
        init_stable_map(MemoryId::new(0))
    );

    // Store institutions using proper Storable types
    pub static INSTITUTIONS: RefCell<StableBTreeMap<StorableString, StorableInstitution, Memory>> = RefCell::new(
        init_stable_map(MemoryId::new(1))
    );

    // Store user profiles using proper Storable types
    pub static USER_PROFILES: RefCell<StableBTreeMap<StorablePrincipal, StorableUserProfile, Memory>> = RefCell::new(
        init_stable_map(MemoryId::new(2))
    );
}

// Helper function to safely initialize stable maps
fn init_stable_map<K, V>(memory_id: MemoryId) -> StableBTreeMap<K, V, Memory> 
where
    K: Storable + Clone + Ord,
    V: Storable + Clone,
{
    let memory = MEMORY_MANAGER.with(|m| m.borrow().get(memory_id));
    
    // Check if memory has been initialized by checking if it has any data
    // If memory is empty/uninitialized, create new map
    // If memory has data, load existing map
    if memory.size() == 0 {
        StableBTreeMap::new(memory)
    } else {
        StableBTreeMap::load(memory)
    }
}

// Helper function to safely get and deserialize a document
pub fn get_document_safe(document_id: &str) -> Option<Document> {
    DOCUMENTS.with(|storage| {
        storage.borrow().get(&StorableString(document_id.to_string()))
            .map(|storable_doc| storable_doc.0)
    })
}

// Helper function to safely get and deserialize an institution
pub fn get_institution_safe(institution_id: &str) -> Option<Institution> {
    INSTITUTIONS.with(|storage| {
        storage.borrow().get(&StorableString(institution_id.to_string()))
            .map(|storable_inst| storable_inst.0)
    })
}

// Helper function to safely update an institution
pub fn update_institution_safe(institution_id: &str, institution: &Institution) -> Result<(), String> {
    INSTITUTIONS.with(|storage| {
        storage.borrow_mut().insert(StorableString(institution_id.to_string()), StorableInstitution(institution.clone()));
    });
    Ok(())
}

// Helper function to safely store a document
pub fn store_document_safe(document_id: &str, document: &Document) -> Result<(), String> {
    // Validate document before storing
    if document_id.is_empty() {
        return Err("Document ID cannot be empty".to_string());
    }
    
    if document.document_id != document_id {
        return Err("Document ID mismatch".to_string());
    }
    
    // Log storage operation for memory wipe tracking
    let before_count = DOCUMENTS.with(|storage| storage.borrow().len());
    
    DOCUMENTS.with(|storage| {
        storage.borrow_mut().insert(StorableString(document_id.to_string()), StorableDocument(document.clone()));
    });
    
    let after_count = DOCUMENTS.with(|storage| storage.borrow().len());
    
    // Check for unexpected storage behavior
    if after_count < before_count {
        let logger = get_logger("storage");
        let severity = get_severity_for_event_type("STORAGE_ANOMALY");
        logger.log(severity, "STORAGE_ANOMALY", &format!("Document count decreased during store operation! Before: {}, After: {}", before_count, after_count), None);
    }
    
    Ok(())
}

// User profile helper functions
pub fn get_user_profile_safe(user_identity: &Principal) -> Option<UserProfile> {
    USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&StorablePrincipal(*user_identity))
            .map(|storable_profile| storable_profile.0)
    })
}

pub fn update_user_profile_safe(user_identity: &Principal, profile: &UserProfile) -> Result<(), String> {
    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(
            StorablePrincipal(*user_identity),
            StorableUserProfile(profile.clone())
        );
    });
    Ok(())
}

// Function to validate all storage integrity
pub fn validate_all_storage() -> Result<(), String> {
    // Clear corrupted entries during validation
    cleanup_corrupted_entries();
    
    // Validate documents - only check structural integrity, not content issues handled by cleanup
    let document_issues = DOCUMENTS.with(|storage| {
        let mut issues = Vec::new();
        for (key, value) in storage.borrow().iter() {
            if key.0.is_empty() {
                issues.push("Empty document key found".to_string());
            }
            
            if value.0.document_id.is_empty() {
                issues.push(format!("Document with key '{}' has empty document_id", key.0));
            }
            
            if !value.0.document_id.is_empty() && value.0.document_id != key.0 {
                issues.push(format!("Document ID mismatch: key='{}', document_id='{}'", key.0, value.0.document_id));
            }
        }
        issues
    });
    
    // Log document issues as warnings but don't fail validation
    if !document_issues.is_empty() {
        let logger = get_logger("storage");
        let severity = get_severity_for_event_type("VALIDATION_WARNINGS");
        logger.log(severity, "VALIDATION_WARNINGS", &format!("Document validation warnings: {}", document_issues.join("; ")), Some(document_issues.join("; ")));
    }
    
    // Validate institutions - only check structural integrity
    let institution_issues = INSTITUTIONS.with(|storage| {
        let mut issues = Vec::new();
        for (key, value) in storage.borrow().iter() {
            if key.0.is_empty() {
                issues.push("Empty institution key found".to_string());
            }
            
            if value.0.institution_id.is_empty() {
                issues.push(format!("Institution with key '{}' has empty institution_id", key.0));
            }
            
            if !value.0.institution_id.is_empty() && value.0.institution_id != key.0 {
                issues.push(format!("Institution ID mismatch: key='{}', institution_id='{}'", key.0, value.0.institution_id));
            }
            
            if value.0.name.is_empty() {
                issues.push(format!("Institution '{}' has empty name", key.0));
            }
        }
        issues
    });
    
    // Log institution issues as warnings but don't fail validation
    if !institution_issues.is_empty() {
        let logger = get_logger("storage");
        let severity = get_severity_for_event_type("VALIDATION_WARNINGS");
        logger.log(severity, "VALIDATION_WARNINGS", &format!("Institution validation warnings: {}", institution_issues.join("; ")), Some(institution_issues.join("; ")));
    }
    
    // Validate user profiles - only check structural integrity, not content issues handled by cleanup
    let user_profile_issues = USER_PROFILES.with(|storage| {
        let mut issues = Vec::new();
        for (key, value) in storage.borrow().iter() {
            if value.0.internet_identity != key.0 {
                issues.push(format!("User identity mismatch: key='{}', internet_identity='{}'", key.0, value.0.internet_identity));
            }
            
            if value.0.name.is_empty() {
                issues.push(format!("User profile '{}' has empty name", key.0));
            }
            
            if value.0.email.is_empty() {
                issues.push(format!("User profile '{}' has empty email", key.0));
            }
            
            // Validate email format (basic check)
            if !value.0.email.is_empty() && !value.0.email.contains('@') {
                issues.push(format!("User profile '{}' has invalid email format: {}", key.0, value.0.email));
            }
        }
        issues
    });
    
    // Log user profile issues as warnings but don't fail validation
    if !user_profile_issues.is_empty() {
        let logger = get_logger("storage");
        let severity = get_severity_for_event_type("VALIDATION_WARNINGS");
        logger.log(severity, "VALIDATION_WARNINGS", &format!("User profile validation warnings: {}", user_profile_issues.join("; ")), Some(user_profile_issues.join("; ")));
    }
    
    Ok(())
}

// Cleanup function - removes documents with empty files and anonymous users
pub fn cleanup_corrupted_entries() -> CleanupResult {
    let mut total_cleaned = 0;
    let mut cleaned_document_ids = Vec::new();
    let mut cleaned_user_profile_identities = Vec::new();
    
    // Remove documents with empty file data
    let corrupted_doc_keys: Vec<String> = DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(key, value)| {
                if value.0.file_data.is_empty() {
                    Some(key.0.clone())
                } else {
                    None
                }
            })
            .collect()
    });
    
    for key in corrupted_doc_keys {
        let logger = get_logger("storage");
        let severity = get_severity_for_event_type("CLEANUP");
        logger.log(severity, "CLEANUP", &format!("Removing document with empty file data: {}", key), Some(key.clone()));
        DOCUMENTS.with(|storage| {
            storage.borrow_mut().remove(&StorableString(key.clone()));
        });
        cleaned_document_ids.push(key);
        total_cleaned += 1;
    }
    
    // Remove anonymous user profiles
    let anonymous_user_keys: Vec<Principal> = USER_PROFILES.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(key, value)| {
                if value.0.internet_identity == Principal::anonymous() {
                    Some(key.0)
                } else {
                    None
                }
            })
            .collect()
    });
    
    for principal in anonymous_user_keys {
        let logger = get_logger("storage");
        let severity = get_severity_for_event_type("CLEANUP");
        logger.log(severity, "CLEANUP", &format!("Removing anonymous user profile: {}", principal), Some(principal.to_string()));
        USER_PROFILES.with(|storage| {
            storage.borrow_mut().remove(&StorablePrincipal(principal));
        });
        cleaned_user_profile_identities.push(principal.to_string());
        total_cleaned += 1;
    }
    
    CleanupResult {
        total_cleaned,
        cleaned_document_ids,
        cleaned_user_profile_identities,
    }
}

// Function to get storage statistics for monitoring
pub fn get_storage_stats() -> StorageStats {
    let institution_count = INSTITUTIONS.with(|storage| storage.borrow().len());
    let user_profile_count = USER_PROFILES.with(|storage| storage.borrow().len());
    
    // Calculate document count and total file size in a single pass
    let (document_count, total_file_size) = DOCUMENTS.with(|storage| {
        let mut count = 0;
        let mut total_size = 0;
        for (_, doc) in storage.borrow().iter() {
            count += 1;
            total_size += doc.0.file_data.len() as u64;
        }
        (count, total_size)
    });
    
    StorageStats {
        document_count: document_count as u64,
        institution_count: institution_count as u64,
        user_profile_count: user_profile_count as u64,
        total_file_size_bytes: total_file_size,
    }
}

// Function to detect potential memory wipes by checking for sudden drops
pub fn detect_memory_anomalies() -> Vec<String> {
    let mut anomalies = Vec::new();
    let stats = get_storage_stats();
    
    // Check if all storage is empty (potential complete wipe)
    if stats.document_count == 0 && stats.institution_count == 0 && stats.user_profile_count == 0 {
        anomalies.push("CRITICAL: All storage appears to be empty - potential memory wipe detected".to_string());
    }
    
    // Check for suspiciously low counts (might indicate partial wipe)
    if stats.document_count == 0 && stats.institution_count > 0 {
        anomalies.push("WARNING: Documents are empty but institutions exist - potential partial wipe".to_string());
    }
    
    if stats.institution_count == 0 && stats.document_count > 0 {
        anomalies.push("WARNING: Institutions are empty but documents exist - potential partial wipe".to_string());
    }
    
    if stats.user_profile_count == 0 && (stats.document_count > 0 || stats.institution_count > 0) {
        anomalies.push("WARNING: User profiles are empty but other data exists - potential partial wipe".to_string());
    }
    
    // Check for orphaned data
    let orphaned_docs = DOCUMENTS.with(|docs| {
        docs.borrow().iter()
            .filter(|(_, doc)| {
                // Check if document's institution still exists
                !INSTITUTIONS.with(|insts| 
                    insts.borrow().contains_key(&crate::storage::StorableString(doc.0.institution_id.clone()))
                )
            })
            .count()
    });
    
    if orphaned_docs > 0 {
        anomalies.push(format!("WARNING: {} documents have orphaned institution references", orphaned_docs));
    }
    
    // Log any anomalies found
    for anomaly in &anomalies {
        let logger = get_logger("storage");
        let severity = get_severity_for_event_type("MEMORY_ANOMALY");
        logger.log(severity, "MEMORY_ANOMALY", anomaly, None);
    }
    
    anomalies
}


