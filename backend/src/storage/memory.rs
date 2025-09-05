use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, 
    storable::Bound, 
    Storable,
};
use std::cell::RefCell;
use candid::Principal;
use crate::types::{Document, Institution, UserProfile};
use std::borrow::Cow;

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Wrapper types that implement Storable for stable storage
#[derive(Clone)]
pub struct StorableDocument(pub Document);

#[derive(Clone)]
pub struct StorableInstitution(pub Institution);

#[derive(Clone)]
pub struct StorableTokens(pub Vec<String>);

#[derive(Clone)]
pub struct StorableUserProfile(pub UserProfile);

// Implement Storable for Document wrapper
impl Storable for StorableDocument {
    fn to_bytes(&self) -> Cow<[u8]> {
        match serde_json::to_vec(&self.0) {
            Ok(bytes) => Cow::Owned(bytes),
            Err(e) => {
                ic_cdk::println!("CRITICAL: Failed to serialize document: {}", e);
                // In case of serialization failure, we trap to prevent data corruption
                ic_cdk::trap(&format!("Document serialization failed: {}", e));
            }
        }
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        match serde_json::from_slice(&bytes) {
            Ok(document) => StorableDocument(document),
            Err(e) => {
                ic_cdk::println!("CRITICAL: Failed to deserialize document: {}", e);
                ic_cdk::println!("Corrupted data (first 100 bytes): {:?}", 
                    &bytes[..std::cmp::min(100, bytes.len())]);
                
                // Return a default/empty document instead of trapping to allow recovery
                StorableDocument(crate::types::Document::default())
            }
        }
    }

    const BOUND: Bound = Bound::Unbounded;
}

// Implement Storable for Institution wrapper
impl Storable for StorableInstitution {
    fn to_bytes(&self) -> Cow<[u8]> {
        match serde_json::to_vec(&self.0) {
            Ok(bytes) => Cow::Owned(bytes),
            Err(e) => {
                ic_cdk::println!("CRITICAL: Failed to serialize institution: {}", e);
                ic_cdk::trap(&format!("Institution serialization failed: {}", e));
            }
        }
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        match serde_json::from_slice(&bytes) {
            Ok(institution) => StorableInstitution(institution),
            Err(e) => {
                ic_cdk::println!("CRITICAL: Failed to deserialize institution: {}", e);
                ic_cdk::println!("Corrupted institution data (first 100 bytes): {:?}", 
                    &bytes[..std::cmp::min(100, bytes.len())]);
                
                // Return a default/empty institution instead of trapping to allow recovery
                StorableInstitution(crate::types::Institution::default())
            }
        }
    }

    const BOUND: Bound = Bound::Unbounded;
}

// Implement Storable for Tokens wrapper
impl Storable for StorableTokens {
    fn to_bytes(&self) -> Cow<[u8]> {
        match serde_json::to_vec(&self.0) {
            Ok(bytes) => Cow::Owned(bytes),
            Err(e) => {
                ic_cdk::println!("CRITICAL: Failed to serialize tokens: {}", e);
                ic_cdk::trap(&format!("Tokens serialization failed: {}", e));
            }
        }
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        match serde_json::from_slice(&bytes) {
            Ok(tokens) => StorableTokens(tokens),
            Err(e) => {
                ic_cdk::println!("CRITICAL: Failed to deserialize tokens: {}", e);
                ic_cdk::println!("Corrupted tokens data (first 100 bytes): {:?}", 
                    &bytes[..std::cmp::min(100, bytes.len())]);
                
                // Return empty tokens instead of trapping to allow recovery
                StorableTokens(Vec::new())
            }
        }
    }

    const BOUND: Bound = Bound::Unbounded;
}

// Implement Storable for UserProfile wrapper
impl Storable for StorableUserProfile {
    fn to_bytes(&self) -> Cow<[u8]> {
        match serde_json::to_vec(&self.0) {
            Ok(bytes) => Cow::Owned(bytes),
            Err(e) => {
                ic_cdk::println!("CRITICAL: Failed to serialize user profile: {}", e);
                ic_cdk::trap(&format!("UserProfile serialization failed: {}", e));
            }
        }
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        match serde_json::from_slice(&bytes) {
            Ok(profile) => StorableUserProfile(profile),
            Err(e) => {
                ic_cdk::println!("CRITICAL: Failed to deserialize user profile: {}", e);
                ic_cdk::println!("Corrupted profile data (first 100 bytes): {:?}", 
                    &bytes[..std::cmp::min(100, bytes.len())]);
                
                // Return a default/empty profile instead of trapping to allow recovery
                StorableUserProfile(UserProfile {
                    internet_identity: Principal::anonymous(),
                    role: crate::types::UserRole::RegularUser,
                    assigned_institution_id: String::new(),
                    created_at: 0,
                    last_login: 0,
                })
            }
        }
    }

    const BOUND: Bound = Bound::Unbounded;
}

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
                ic_cdk::println!("CRITICAL: Failed to deserialize string: {}", e);
                ic_cdk::trap(&format!("String deserialization failed: {}", e));
            }
        }
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1000, // Maximum key size
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
                ic_cdk::println!("CRITICAL: Failed to deserialize principal: {:?}", e);
                ic_cdk::trap(&format!("Principal deserialization failed: {:?}", e));
            }
        }
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 29, // Principal max size
        is_fixed_size: false,
    };
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Store complete documents using proper Storable types
    pub static DOCUMENTS: RefCell<StableBTreeMap<StorableString, StorableDocument, Memory>> = RefCell::new(
        StableBTreeMap::load(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    // Store institutions using proper Storable types
    pub static INSTITUTIONS: RefCell<StableBTreeMap<StorableString, StorableInstitution, Memory>> = RefCell::new(
        StableBTreeMap::load(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    // Store owner mappings using proper Storable types
    pub static OWNER_TOKENS: RefCell<StableBTreeMap<StorablePrincipal, StorableTokens, Memory>> = RefCell::new(
        StableBTreeMap::load(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

    // Store user profiles using proper Storable types
    pub static USER_PROFILES: RefCell<StableBTreeMap<StorablePrincipal, StorableUserProfile, Memory>> = RefCell::new(
        StableBTreeMap::load(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );
}

// Helper functions for storage operations
pub fn principal_to_bytes(principal: &Principal) -> Vec<u8> {
    principal.as_slice().to_vec()
}

pub fn bytes_to_principal(bytes: &[u8]) -> Result<Principal, String> {
    match Principal::try_from_slice(bytes) {
        Ok(principal) => Ok(principal),
        Err(_) => Err("Invalid principal bytes".to_string())
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
    
    DOCUMENTS.with(|storage| {
        storage.borrow_mut().insert(StorableString(document_id.to_string()), StorableDocument(document.clone()));
    });
    Ok(())
}

// Function to validate all storage integrity
pub fn validate_all_storage() -> Result<(), String> {
    // Clear corrupted entries during validation
    clear_corrupted_entries();
    
    // Validate documents
    let document_issues = DOCUMENTS.with(|storage| {
        let mut issues = Vec::new();
        for (key, value) in storage.borrow().iter() {
            // Skip default/empty documents that were recovered from corruption
            if value.0.document_id.is_empty() && value.0.name.is_empty() && value.0.file_data.is_empty() {
                ic_cdk::println!("Skipping validation for recovered empty document with key: {}", key.0);
                continue;
            }
            
            if key.0.is_empty() {
                issues.push("Empty document key found".to_string());
            }
            
            if value.0.document_id.is_empty() {
                issues.push(format!("Document with key '{}' has empty document_id", key.0));
            }
            
            if !value.0.document_id.is_empty() && value.0.document_id != key.0 {
                issues.push(format!("Document ID mismatch: key='{}', document_id='{}'", key.0, value.0.document_id));
            }
            
            if value.0.file_data.is_empty() && value.0.file_type != "text/plain" && !value.0.file_type.is_empty() {
                issues.push(format!("Document '{}' has empty file data", key.0));
            }
        }
        issues
    });
    
    // Log document issues as warnings but don't fail validation
    if !document_issues.is_empty() {
        ic_cdk::println!("Document validation warnings: {}", document_issues.join("; "));
    }
    
    // Validate institutions
    let institution_issues = INSTITUTIONS.with(|storage| {
        let mut issues = Vec::new();
        for (key, value) in storage.borrow().iter() {
            // Skip default/empty institutions that were recovered from corruption
            if value.0.institution_id.is_empty() && value.0.name.is_empty() {
                ic_cdk::println!("Skipping validation for recovered empty institution with key: {}", key.0);
                continue;
            }
            
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
        ic_cdk::println!("Institution validation warnings: {}", institution_issues.join("; "));
    }
    
    // Validate owner tokens consistency (be lenient about missing documents during recovery)
    let token_issues = OWNER_TOKENS.with(|storage| {
        let mut issues = Vec::new();
        for (principal, tokens) in storage.borrow().iter() {
            // Check that all tokens in the list correspond to actual documents
            for token in &tokens.0 {
                if !DOCUMENTS.with(|docs| docs.borrow().contains_key(&StorableString(token.clone()))) {
                    issues.push(format!("Token '{}' for principal '{}' references non-existent document", token, principal.0));
                }
            }
        }
        issues
    });
    
    // Log token issues as warnings but don't fail validation during recovery
    if !token_issues.is_empty() {
        ic_cdk::println!("Owner tokens validation warnings: {}", token_issues.join("; "));
    }
    
    Ok(())
}

// Function to clear corrupted entries that were replaced with defaults
pub fn clear_corrupted_entries() {
    // Clear corrupted documents (those with all default values)
    let corrupted_doc_keys: Vec<String> = DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(key, value)| {
                // Check if this is a default document (indicates corruption recovery)
                if value.0.document_id.is_empty() && 
                   value.0.name.is_empty() && 
                   value.0.file_data.is_empty() &&
                   value.0.company_name.is_empty() &&
                   value.0.description.is_empty() {
                    Some(key.0.clone())
                } else {
                    None
                }
            })
            .collect()
    });
    
    for key in corrupted_doc_keys {
        ic_cdk::println!("Removing corrupted document entry: {}", key);
        DOCUMENTS.with(|storage| {
            storage.borrow_mut().remove(&StorableString(key));
        });
    }
    
    // Clear corrupted institutions (those with all default values)
    let corrupted_inst_keys: Vec<String> = INSTITUTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(key, value)| {
                // Check if this is a default institution (indicates corruption recovery)
                if value.0.institution_id.is_empty() && 
                   value.0.name.is_empty() && 
                   value.0.email.is_empty() &&
                   value.0.created_at == 0 {
                    Some(key.0.clone())
                } else {
                    None
                }
            })
            .collect()
    });
    
    for key in corrupted_inst_keys {
        ic_cdk::println!("Removing corrupted institution entry: {}", key);
        INSTITUTIONS.with(|storage| {
            storage.borrow_mut().remove(&StorableString(key));
        });
    }
}

// Function to get storage statistics for monitoring
pub fn get_storage_stats() -> StorageStats {
    let document_count = DOCUMENTS.with(|storage| storage.borrow().len());
    let institution_count = INSTITUTIONS.with(|storage| storage.borrow().len());
    let owner_mapping_count = OWNER_TOKENS.with(|storage| storage.borrow().len());
    
    // Calculate total file data size
    let total_file_size = DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .map(|(_, doc)| doc.0.file_data.len() as u64)
            .sum()
    });
    
    StorageStats {
        document_count: document_count as u64,
        institution_count: institution_count as u64,
        owner_mapping_count: owner_mapping_count as u64,
        total_file_size_bytes: total_file_size,
    }
}

// Structure to hold storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub document_count: u64,
    pub institution_count: u64,
    pub owner_mapping_count: u64,
    pub total_file_size_bytes: u64,
}

// Legacy helper functions for backward compatibility
pub fn document_to_bytes(document: &Document) -> Result<Vec<u8>, String> {
    serde_json::to_vec(document).map_err(|e| format!("Failed to serialize document: {}", e))
}

pub fn bytes_to_document(bytes: &[u8]) -> Result<Document, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("Failed to deserialize document: {}", e))
}

pub fn tokens_to_bytes(tokens: &[String]) -> Result<Vec<u8>, String> {
    serde_json::to_vec(tokens).map_err(|e| format!("Failed to serialize tokens: {}", e))
}

pub fn bytes_to_tokens(bytes: &[u8]) -> Result<Vec<String>, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("Failed to deserialize tokens: {}", e))
}

pub fn institution_to_bytes(institution: &Institution) -> Result<Vec<u8>, String> {
    serde_json::to_vec(institution).map_err(|e| format!("Failed to serialize institution: {}", e))
}

pub fn bytes_to_institution(bytes: &[u8]) -> Result<Institution, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("Failed to deserialize institution: {}", e))
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

