use ic_cdk::query;
use candid::Principal;
use crate::types::CollectionMetadata;
use crate::storage::{COLLECTIONS, bytes_to_collection};

// ============================================================================
// COLLECTION QUERY FUNCTIONS
// ============================================================================

/// Get collection metadata by collection ID
#[query]
pub fn get_collection_metadata(collection_id: String) -> Option<CollectionMetadata> {
    COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id)
            .and_then(|bytes| bytes_to_collection(&bytes))
    })
}

/// Get all collection IDs
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
            .filter_map(|(_, bytes)| bytes_to_collection(&bytes))
            .collect()
    })
}

/// Get collections by owner
#[query]
pub fn get_collections_by_owner(owner: Principal) -> Vec<CollectionMetadata> {
    COLLECTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_collection(&bytes))
            .filter(|collection| collection.owner == owner)
            .collect()
    })
}

/// Get collections by institution
#[query]
pub fn get_collections_by_institution(institution_id: String) -> Vec<CollectionMetadata> {
    COLLECTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_collection(&bytes))
            .filter(|collection| collection.institution_id == institution_id)
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
