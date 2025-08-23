use ic_cdk::query;
use crate::types::{Document, CollectionMetadata, Institution, DocumentType};
use crate::storage::{DOCUMENTS, COLLECTIONS, INSTITUTIONS, bytes_to_document, bytes_to_collection, bytes_to_institution};

// ============================================================================
// SEARCH FUNCTIONS
// ============================================================================

/// Search documents by name (case-insensitive partial match)
#[query]
pub fn search_documents_by_name(search_term: String) -> Vec<Document> {
    let search_term_lower = search_term.to_lowercase();
    DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_document(&bytes).ok())
            .filter(|doc| doc.document_base_data.name.to_lowercase().contains(&search_term_lower))
            .collect()
    })
}

/// Search collections by name (case-insensitive partial match)
#[query]
pub fn search_collections_by_name(search_term: String) -> Vec<CollectionMetadata> {
    let search_term_lower = search_term.to_lowercase();
    COLLECTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_collection(&bytes).ok())
            .filter(|collection| collection.name.to_lowercase().contains(&search_term_lower))
            .collect()
    })
}

/// Search institutions by name (case-insensitive partial match)
#[query]
pub fn search_institutions_by_name(search_term: String) -> Vec<Institution> {
    let search_term_lower = search_term.to_lowercase();
    INSTITUTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_institution(&bytes).ok())
            .filter(|institution| institution.name.to_lowercase().contains(&search_term_lower))
            .collect()
    })
}
