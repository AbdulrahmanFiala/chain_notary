use ic_cdk::query;
use candid::Principal;
use crate::types::CollectionMetadata;
use crate::storage::{COLLECTIONS, bytes_to_collection};

// ============================================================================
// COLLECTION QUERY FUNCTIONS
// ============================================================================

#[query]
pub fn get_collection_metadata(collection_id: String) -> Option<CollectionMetadata> {
    crate::storage::get_collection_safe(&collection_id)
}

#[query]
pub fn get_all_collection_ids() -> Vec<String> {
    COLLECTIONS.with(|storage| {
        storage.borrow().iter().map(|(k, _)| k.clone()).collect()
    })
}

/// Get all collections with full metadata
#[query]
pub fn get_all_collections() -> Vec<CollectionMetadata> {
    COLLECTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_collection(&bytes).ok())
            .collect()
    })
}

#[query]
pub fn get_collections_by_owner(owner: Principal) -> Vec<CollectionMetadata> {
    COLLECTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_collection(&bytes).ok())
            .filter(|collection| collection.owner == owner)
            .collect()
    })
}

#[query]
pub fn get_collections_by_institution(institution_id: String) -> Vec<CollectionMetadata> {
    // Normalize the institution_id by trimming whitespace
    let normalized_institution_id = institution_id.trim();
    
    COLLECTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_collection(&bytes).ok())
            .filter(|collection| {
                if normalized_institution_id.is_empty() {
                    // If institution_id is empty or whitespace-only, return collections with no institution
                    collection.institution_id.trim().is_empty()
                } else {
                    // Otherwise, return collections that match the institution_id (after trimming)
                    collection.institution_id.trim() == normalized_institution_id
                }
            })
            .collect()
    })
}

/// Get collection count
#[query]
pub fn get_collection_count() -> u64 {
    COLLECTIONS.with(|storage| {
        storage.borrow().len() as u64
    })
}
