use ic_cdk::query;
use candid::Principal;
use crate::types::Institution;
use crate::storage::{INSTITUTIONS, bytes_to_institution};

// ============================================================================
// INSTITUTION QUERY FUNCTIONS
// ============================================================================

/// Get institution metadata by institution ID
#[query]
pub fn get_institution_metadata(institution_id: String) -> Option<Institution> {
    crate::storage::get_institution_safe(&institution_id)
}

/// Get all institutions with full metadata
#[query]
pub fn get_all_institutions() -> Vec<Institution> {
    INSTITUTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_institution(&bytes).ok())
            .collect()
    })
}

/// Get institutions by owner
#[query]
pub fn get_institutions_by_owner(owner: Principal) -> Vec<Institution> {
    INSTITUTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_institution(&bytes).ok())
            .filter(|institution| institution.owner == owner)
            .collect()
    })
}

/// Get institution count
#[query]
pub fn get_institution_count() -> u64 {
    INSTITUTIONS.with(|storage| {
        storage.borrow().len() as u64
    })
}
