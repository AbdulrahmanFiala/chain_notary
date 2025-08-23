use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use std::cell::RefCell;
use candid::Principal;
use crate::types::{Document, CollectionMetadata, Institution};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Store complete documents (metadata + file data) in a single storage area
    pub static DOCUMENTS: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    // Store collections separately from documents
    pub static COLLECTIONS: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    // Store institutions separately from collections
    pub static INSTITUTIONS: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

    // Store owner mappings as JSON strings
    pub static OWNER_TOKENS: RefCell<StableBTreeMap<Vec<u8>, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
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
        storage.borrow().get(&document_id.to_string())
            .and_then(|bytes| bytes_to_document(&bytes).ok())
    })
}

// Helper function to safely get and deserialize a collection
pub fn get_collection_safe(collection_id: &str) -> Option<CollectionMetadata> {
    COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id.to_string())
            .and_then(|bytes| bytes_to_collection(&bytes).ok())
    })
}

// Helper function to safely get and deserialize an institution
pub fn get_institution_safe(institution_id: &str) -> Option<Institution> {
    INSTITUTIONS.with(|storage| {
        storage.borrow().get(&institution_id.to_string())
            .and_then(|bytes| bytes_to_institution(&bytes).ok())
    })
}

// Helper function to safely update a document
pub fn update_document_safe(document_id: &str, document: &Document) -> Result<(), String> {
    let document_bytes = document_to_bytes(document)
        .map_err(|e| format!("Failed to serialize document: {}", e))?;
    DOCUMENTS.with(|storage| {
        storage.borrow_mut().insert(document_id.to_string(), document_bytes);
    });
    Ok(())
}

// Helper function to safely update a collection
pub fn update_collection_safe(collection_id: &str, collection: &CollectionMetadata) -> Result<(), String> {
    let collection_bytes = collection_to_bytes(collection)
        .map_err(|e| format!("Failed to serialize collection: {}", e))?;
    COLLECTIONS.with(|storage| {
        storage.borrow_mut().insert(collection_id.to_string(), collection_bytes);
    });
    Ok(())
}

// Helper function to safely update an institution
pub fn update_institution_safe(institution_id: &str, institution: &Institution) -> Result<(), String> {
    let institution_bytes = institution_to_bytes(institution)
        .map_err(|e| format!("Failed to serialize institution: {}", e))?;
    INSTITUTIONS.with(|storage| {
        storage.borrow_mut().insert(institution_id.to_string(), institution_bytes);
    });
    Ok(())
}

pub fn document_to_bytes(document: &Document) -> Result<Vec<u8>, String> {
    serde_json::to_vec(document).map_err(|e| format!("Failed to serialize document: {}", e))
}

pub fn bytes_to_document(bytes: &[u8]) -> Result<Document, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("Failed to deserialize document: {}", e))
}

pub fn collection_to_bytes(collection: &CollectionMetadata) -> Result<Vec<u8>, String> {
    serde_json::to_vec(collection).map_err(|e| format!("Failed to serialize collection: {}", e))
}

pub fn bytes_to_collection(bytes: &[u8]) -> Result<CollectionMetadata, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("Failed to deserialize collection: {}", e))
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