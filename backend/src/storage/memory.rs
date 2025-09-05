use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, 
    storable::Bound, 
    Storable,
};
use std::cell::RefCell;
use candid::Principal;
use crate::types::{Document, Institution};
use std::borrow::Cow;

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Wrapper types that implement Storable for stable storage
#[derive(Clone)]
pub struct StorableDocument(pub Document);

#[derive(Clone)]
pub struct StorableInstitution(pub Institution);

#[derive(Clone)]
pub struct StorableTokens(pub Vec<String>);

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
                // Log the error and attempt recovery or trap to prevent corruption
                ic_cdk::trap(&format!("Document deserialization failed: {}", e));
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
                ic_cdk::trap(&format!("Institution deserialization failed: {}", e));
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
                ic_cdk::trap(&format!("Tokens deserialization failed: {}", e));
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
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Store complete documents using proper Storable types
    pub static DOCUMENTS: RefCell<StableBTreeMap<StorableString, StorableDocument, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    // Store institutions using proper Storable types
    pub static INSTITUTIONS: RefCell<StableBTreeMap<StorableString, StorableInstitution, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    // Store owner mappings using proper Storable types
    pub static OWNER_TOKENS: RefCell<StableBTreeMap<StorablePrincipal, StorableTokens, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
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
    // Validate documents
    let document_issues = DOCUMENTS.with(|storage| {
        let mut issues = Vec::new();
        for (key, value) in storage.borrow().iter() {
            if key.0.is_empty() {
                issues.push("Empty document key found".to_string());
            }
            
            if value.0.document_id.is_empty() {
                issues.push(format!("Document with key '{}' has empty document_id", key.0));
            }
            
            if value.0.document_id != key.0 {
                issues.push(format!("Document ID mismatch: key='{}', document_id='{}'", key.0, value.0.document_id));
            }
            
            if value.0.file_data.is_empty() && value.0.file_type != "text/plain" {
                issues.push(format!("Document '{}' has empty file data", key.0));
            }
        }
        issues
    });
    
    if !document_issues.is_empty() {
        return Err(format!("Document validation failed: {}", document_issues.join("; ")));
    }
    
    // Validate institutions
    let institution_issues = INSTITUTIONS.with(|storage| {
        let mut issues = Vec::new();
        for (key, value) in storage.borrow().iter() {
            if key.0.is_empty() {
                issues.push("Empty institution key found".to_string());
            }
            
            if value.0.institution_id.is_empty() {
                issues.push(format!("Institution with key '{}' has empty institution_id", key.0));
            }
            
            if value.0.institution_id != key.0 {
                issues.push(format!("Institution ID mismatch: key='{}', institution_id='{}'", key.0, value.0.institution_id));
            }
            
            if value.0.name.is_empty() {
                issues.push(format!("Institution '{}' has empty name", key.0));
            }
        }
        issues
    });
    
    if !institution_issues.is_empty() {
        return Err(format!("Institution validation failed: {}", institution_issues.join("; ")));
    }
    
    // Validate owner tokens consistency
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
    
    if !token_issues.is_empty() {
        return Err(format!("Owner tokens validation failed: {}", token_issues.join("; ")));
    }
    
    Ok(())
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

