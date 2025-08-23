use ic_cdk::query;
use candid::Principal;
use crate::types::{Document, CollectionCategory, DocumentType};
use crate::storage::{DOCUMENTS, OWNER_TOKENS, COLLECTIONS, bytes_to_document, bytes_to_collection, principal_to_bytes, bytes_to_tokens};

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
        storage.borrow().get(&document_id)
            .and_then(|bytes| bytes_to_document(&bytes).ok())
            .map(|document| document.file_data)
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
        storage.borrow().iter().map(|(k, _)| k.clone()).collect()
    })
}

/// Get documents owned by a specific principal (fast query)
#[query]
pub fn get_documents_by_owner(owner: Principal) -> Vec<String> {
    OWNER_TOKENS.with(|owner_tokens| {
        let owner_bytes = principal_to_bytes(&owner);
        owner_tokens.borrow().get(&owner_bytes)
            .and_then(|bytes| bytes_to_tokens(&bytes).ok())
            .unwrap_or_default()
    })
}

/// Get total number of documents (fast query)
#[query]
pub fn get_document_count() -> u64 {
    DOCUMENTS.with(|storage| {
        storage.borrow().len() as u64
    })
}

/// Get documents by collection ID (fast query)
#[query]
pub fn get_documents_by_collection(collection_id: String) -> Vec<Document> {
    // Normalize the collection_id by trimming whitespace
    let normalized_collection_id = collection_id.trim();
    
    DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_document(&bytes).ok())
            .filter(|metadata| {
                if normalized_collection_id.is_empty() {
                    // If collection_id is empty or whitespace-only, return documents with no collection
                    metadata.document_base_data.collection_id.trim().is_empty()
                } else {
                    // Otherwise, return documents that match the collection_id (after trimming)
                    metadata.document_base_data.collection_id.trim() == normalized_collection_id
                }
            })
            .collect()
    })
}

/// Get documents by category
#[query]
pub fn get_documents_by_collection_category(category: CollectionCategory) -> Vec<Document> {
    // Get all collections with this category
    let collections = COLLECTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_collection(&bytes).ok())
            .filter(|collection| collection.category == category)
            .collect::<Vec<_>>()
    });
    
    // Get all documents from these collections
    let mut documents = Vec::new();
    for collection in collections {
        for document_id in &collection.documents {
            if let Some(document) = get_document_metadata(document_id.clone()) {
                documents.push(document);
            }
        }
    }
    
    documents
}

/// Get documents by document type
#[query]
pub fn get_documents_by_type(document_type: String) -> Vec<Document> {
    DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_document(&bytes).ok())
            .filter(|doc| {
                match &doc.document_base_data.document_data {
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
            .filter_map(|(_, bytes)| bytes_to_document(&bytes).ok())
            .filter(|doc| {
                match &doc.document_base_data.document_data {
                    DocumentType::EarningRelease(data) => data.quarter == quarter && data.year == year,
                }
            })
            .collect()
    })
}

/// Get documents by institution (through collections)
#[query]
pub fn get_documents_by_institution(institution_id: String) -> Vec<Document> {
    // Get all collections for this institution
    let collections = crate::functions::collection_queries::get_collections_by_institution(institution_id);
    
    // Get all documents from these collections
    let mut documents = Vec::new();
    for collection in collections {
        for document_id in &collection.documents {
            if let Some(document) = crate::storage::get_document_safe(document_id) {
                documents.push(document);
            }
        }
    }
    
    documents
}
