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

pub fn bytes_to_principal(bytes: &[u8]) -> Principal {
    Principal::from_slice(bytes)
}

pub fn document_to_bytes(document: &Document) -> Vec<u8> {
    serde_json::to_vec(document).unwrap_or_default()
}

pub fn bytes_to_document(bytes: &[u8]) -> Option<Document> {
    serde_json::from_slice(bytes).ok()
}

pub fn collection_to_bytes(collection: &CollectionMetadata) -> Vec<u8> {
    serde_json::to_vec(collection).unwrap_or_default()
}

pub fn bytes_to_collection(bytes: &[u8]) -> Option<CollectionMetadata> {
    serde_json::from_slice(bytes).ok()
}

pub fn tokens_to_bytes(tokens: &[String]) -> Vec<u8> {
    serde_json::to_vec(tokens).unwrap_or_default()
}

pub fn bytes_to_tokens(bytes: &[u8]) -> Vec<String> {
    serde_json::from_slice(bytes).unwrap_or_default()
}

pub fn institution_to_bytes(institution: &Institution) -> Vec<u8> {
    serde_json::to_vec(institution).unwrap_or_default()
}

pub fn bytes_to_institution(bytes: &[u8]) -> Option<Institution> {
    serde_json::from_slice(bytes).ok()
} 