use ic_cdk::query;
use candid::Principal;
use crate::types::Institution;
use crate::storage::INSTITUTIONS;

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
            .map(|(_, storable_inst)| storable_inst.0)
            .collect()
    })
}

/// Get institutions by owner
#[query]
pub fn get_institutions_by_owner(owner: Principal) -> Vec<Institution> {
    INSTITUTIONS.with(|storage| {
        storage.borrow().iter()
            .map(|(_, storable_inst)| storable_inst.0)
            .filter(|institution| institution.owner == owner)
            .collect()
    })
}

