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

/// Get all document IDs (fast query)
#[query]
pub fn get_all_document_ids() -> Vec<String> {
    DOCUMENTS.with(|storage| {
        storage.borrow().iter().map(|(k, _)| k.0.clone()).collect()
    })
}

/// Unified document query function with comprehensive filtering, sorting, and pagination
#[query]
pub fn query_documents(
    // Document ID filter
    doc_id: Option<String>,
    
    // Existing filters
    owner: Option<Principal>,
    institution_id: Option<String>,
    document_type: Option<String>,
    quarter: Option<u8>,
    year: Option<u16>,
    
    // Date range filtering
    start_date: Option<u64>,
    end_date: Option<u64>,
    
    // Pagination
    offset: Option<u64>,
    limit: Option<u64>,
    
    // Sorting
    sort_by: Option<String>, // "date", "name", "institution"
    sort_order: Option<String>, // "asc", "desc"
    
    // Data control
    include_file_data: Option<bool>
) -> (Vec<Document>, u64) { // Returns (documents, total_count)
    let include_file_data = include_file_data.unwrap_or(true);
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(10);
    let sort_by = sort_by.unwrap_or_else(|| "date".to_string());
    let sort_order = sort_order.unwrap_or_else(|| "desc".to_string());
    
    // Get all documents and apply filters
    let mut filtered_docs: Vec<Document> = DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .map(|(_, storable_doc)| storable_doc.0)
            .filter(|doc| {
                // Document ID filter
                if let Some(doc_id_filter) = &doc_id {
                    if doc.document_id != *doc_id_filter {
                        return false;
                    }
                }
                
                // Owner filter
                if let Some(owner_filter) = owner {
                    if doc.owner != owner_filter {
                        return false;
                    }
                }
                
                // Institution filter
                if let Some(institution_filter) = &institution_id {
                    let normalized_institution_id = institution_filter.trim();
                    if normalized_institution_id.is_empty() {
                        if !doc.institution_id.trim().is_empty() {
                            return false;
                        }
                    } else {
                        if doc.institution_id.trim() != normalized_institution_id {
                            return false;
                        }
                    }
                }
                
                // Document type filter
                if let Some(type_filter) = &document_type {
                    match &doc.document_data {
                        DocumentType::EarningRelease(_) => {
                            if type_filter != "EarningRelease" {
                                return false;
                            }
                        }
                    }
                }
                
                // Quarter and year filter
                if let (Some(quarter_filter), Some(year_filter)) = (quarter, year) {
                    match &doc.document_data {
                        DocumentType::EarningRelease(data) => {
                            if data.quarter != quarter_filter || data.year != year_filter {
                                return false;
                            }
                        }
                    }
                }
                
                // Date range filter
                if let Some(start) = start_date {
                    if doc.publication_date < start {
                        return false;
                    }
                }
                if let Some(end) = end_date {
                    if doc.publication_date > end {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    });
    
    // Sort documents
    match sort_by.as_str() {
        "name" => {
            if sort_order == "asc" {
                filtered_docs.sort_by(|a, b| a.name.cmp(&b.name));
            } else {
                filtered_docs.sort_by(|a, b| b.name.cmp(&a.name));
            }
        }
        "institution" => {
            if sort_order == "asc" {
                filtered_docs.sort_by(|a, b| a.institution_id.cmp(&b.institution_id));
            } else {
                filtered_docs.sort_by(|a, b| b.institution_id.cmp(&a.institution_id));
            }
        }
        "date" | _ => {
            if sort_order == "asc" {
                filtered_docs.sort_by(|a, b| a.publication_date.cmp(&b.publication_date));
            } else {
                filtered_docs.sort_by(|a, b| b.publication_date.cmp(&a.publication_date));
            }
        }
    }
    
    let total_count = filtered_docs.len() as u64;
    
    // Apply pagination
    let start_idx = offset as usize;
    let end_idx = std::cmp::min(start_idx + limit as usize, filtered_docs.len());
    
    let paginated_docs = if start_idx >= filtered_docs.len() {
        Vec::new()
    } else {
        filtered_docs[start_idx..end_idx].to_vec()
    };
    
    // Remove file data if requested
    let final_docs = if include_file_data {
        paginated_docs
    } else {
        paginated_docs.into_iter().map(|mut doc| {
            doc.file_data = Vec::new(); // Remove file data
            doc
        }).collect()
    };
    
    (final_docs, total_count)
}

/// Get documents owned by a specific principal (wrapper for backward compatibility)
#[query]
pub fn get_documents_by_owner(owner: Principal) -> Vec<DocumentSummary> {
    let (documents, _) = query_documents(
        None, // doc_id
        Some(owner),
        None, // institution_id
        None, // document_type
        None, // quarter
        None, // year
        None, // start_date
        None, // end_date
        None, // offset
        None, // limit
        None, // sort_by
        None, // sort_order
        Some(false), // include_file_data = false for DocumentSummary
    );
    
    // Convert Document to DocumentSummary
    documents.into_iter().map(|doc| {
        DocumentSummary {
            id: doc.document_id,
            document_name: doc.name,
            file_type: doc.file_type,
            publication_date: Some(doc.publication_date),
        }
    }).collect()
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
