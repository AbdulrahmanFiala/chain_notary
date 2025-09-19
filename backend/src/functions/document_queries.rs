use ic_cdk::query;
use candid::Principal;
use crate::types::{Document, DocumentType, DocumentSummary};
use crate::storage::{DOCUMENTS, StorableString};

// ============================================================================
// DOCUMENT QUERY FUNCTIONS
// ============================================================================

/// Get document metadata by document ID (fast query, no file data)
#[query]
pub fn get_document_metadata(document_id: String) -> Option<Document> {
    crate::storage::get_document_safe(&document_id)
}

/// Get document file data by document ID (loads file data)
#[query]
pub fn get_document_file(document_id: String) -> Option<Vec<u8>> {
    DOCUMENTS.with(|storage| {
        storage.borrow().get(&StorableString(document_id))
            .map(|storable_doc| storable_doc.0.file_data)
    })
}

/// Get complete document (metadata + file data) by document ID
#[query]
pub fn get_complete_document(document_id: String) -> Option<(Document, Vec<u8>)> {
    let metadata = get_document_metadata(document_id.clone())?;
    let file_data = get_document_file(document_id)?;
    Some((metadata, file_data))
}

/// Get all document IDs (fast query)
#[query]
pub fn get_all_document_ids() -> Vec<String> {
    DOCUMENTS.with(|storage| {
        storage.borrow().iter().map(|(k, _)| k.0.clone()).collect()
    })
}

/// Get documents owned by a specific principal (direct query - no owner token mapping needed)
#[query]
pub fn get_documents_by_owner(owner: Principal) -> Vec<DocumentSummary> {
    DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .filter(|(_, doc)| doc.0.owner == owner)
            .map(|(_, doc)| {
                let document = &doc.0;
                DocumentSummary {
                    id: document.document_id.clone(),
                    document_name: document.name.clone(),
                    file_type: document.file_type.clone(),
                    publication_date: document.publication_date,
                }
            })
            .collect()
    })
}

/// Get documents by document type
#[query]
pub fn get_documents_by_type(document_type: String) -> Vec<Document> {
    DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .map(|(_, storable_doc)| storable_doc.0)
            .filter(|doc| {
                match &doc.document_data {
                    DocumentType::EarningRelease(_) => document_type == "EarningRelease",
                }
            })
            .collect()
    })
}

/// Get documents by earning release quarter and year
#[query]
pub fn get_documents_by_quarter_year(quarter: u8, year: u16) -> Vec<Document> {
    DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .map(|(_, storable_doc)| storable_doc.0)
            .filter(|doc| {
                match &doc.document_data {
                    DocumentType::EarningRelease(data) => data.quarter == quarter && data.year == year,
                }
            })
            .collect()
    })
}

/// Get documents by institution
#[query]
pub fn get_documents_by_institution(institution_id: String) -> Vec<Document> {
    // Normalize the institution_id by trimming whitespace
    let normalized_institution_id = institution_id.trim();
    
    DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .map(|(_, storable_doc)| storable_doc.0)
            .filter(|document| {
                if normalized_institution_id.is_empty() {
                    // If institution_id is empty or whitespace-only, return documents with no institution
                    document.institution_id.trim().is_empty()
                } else {
                    // Otherwise, return documents that match the institution_id (after trimming)
                    document.institution_id.trim() == normalized_institution_id
                }
            })
            .collect()
    })
}

/// Check if a user owns a specific document (direct query)
#[query]
pub fn is_document_owned_by(document_id: String, owner: Principal) -> bool {
    DOCUMENTS.with(|storage| {
        storage.borrow().get(&crate::storage::memory::StorableString(document_id))
            .map(|doc| doc.0.owner == owner)
            .unwrap_or(false)
    })
}
