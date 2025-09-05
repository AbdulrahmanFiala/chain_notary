use ic_cdk::query;
use crate::types::{Document, Institution};
use crate::storage::{DOCUMENTS, INSTITUTIONS};

// ============================================================================
// SEARCH FUNCTIONS
// ============================================================================

/// Search documents by name (case-insensitive partial match)
#[query]
pub fn search_documents_by_name(search_term: String) -> Vec<Document> {
    let search_term_lower = search_term.to_lowercase();
    DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .map(|(_, storable_doc)| storable_doc.0)
            .filter(|doc| doc.name.to_lowercase().contains(&search_term_lower))
            .collect()
    })
}


/// Search institutions by name (case-insensitive partial match)
#[query]
pub fn search_institutions_by_name(search_term: String) -> Vec<Institution> {
    let search_term_lower = search_term.to_lowercase();
    INSTITUTIONS.with(|storage| {
        storage.borrow().iter()
            .map(|(_, storable_inst)| storable_inst.0)
            .filter(|institution| institution.name.to_lowercase().contains(&search_term_lower))
            .collect()
    })
}
