use ic_cdk::query;
use candid::Principal;
use crate::types::{Document, CollectionCategory};
use crate::storage::{DOCUMENTS, OWNER_TOKENS, COLLECTIONS, bytes_to_document, bytes_to_collection, principal_to_bytes, bytes_to_tokens};

// ============================================================================
// DOCUMENT QUERY FUNCTIONS
// ============================================================================

/// Get document metadata by document ID (fast query, no file data)
#[query]
pub fn get_document_metadata(document_id: String) -> Option<Document> {
    DOCUMENTS.with(|storage| {
        storage.borrow().get(&document_id).and_then(|bytes| bytes_to_document(&bytes))
    })
}

/// Get document file data by document ID (loads file data)
#[query]
pub fn get_document_file(document_id: String) -> Option<Vec<u8>> {
    DOCUMENTS.with(|storage| {
        storage.borrow().get(&document_id).and_then(|bytes| {
            bytes_to_document(&bytes).map(|document| document.file_data.unwrap_or_default())
        })
    })
}

/// Get complete document (metadata + file data) by document ID
#[query]
pub fn get_complete_document(document_id: String) -> Option<(Document, Vec<u8>)> {
    let metadata = get_document_metadata(document_id.clone())?;
    let file_data = get_document_file(document_id)?;
    Some((metadata, file_data))
}

/// List all document IDs (fast query)
#[query]
pub fn list_all_documents() -> Vec<String> {
    DOCUMENTS.with(|storage| {
        storage.borrow().iter().map(|(k, _)| k.clone()).collect()
    })
}

/// Get documents owned by a specific principal (fast query)
#[query]
pub fn get_documents_by_owner(owner: Principal) -> Vec<String> {
    OWNER_TOKENS.with(|owner_tokens| {
        let owner_bytes = principal_to_bytes(&owner);
        owner_tokens.borrow().get(&owner_bytes).map(|bytes| bytes_to_tokens(&bytes)).unwrap_or_default()
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
    DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_document(&bytes))
            .filter(|metadata| metadata.collection_id == Some(collection_id.clone()))
            .collect()
    })
}

/// Get documents by category
#[query]
pub fn get_documents_by_collection_category(category: CollectionCategory) -> Vec<Document> {
    // Get all collections with this category
    let collections = COLLECTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_collection(&bytes))
            .filter(|collection| collection.category.as_ref() == Some(&category))
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

/// Get documents by recipient email
#[query]
pub fn get_documents_by_recipient_email(recipient_email: String) -> Vec<Document> {
    DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_document(&bytes))
            .filter(|doc| {
                doc.recipient.as_ref()
                    .and_then(|r| r.email.as_ref())
                    .map(|email| email == &recipient_email)
                    .unwrap_or(false)
            })
            .collect()
    })
}

/// Get documents by recipient ID
#[query]
pub fn get_documents_by_recipient_id(recipient_id: String) -> Vec<Document> {
    DOCUMENTS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_document(&bytes))
            .filter(|doc| {
                doc.recipient.as_ref()
                    .and_then(|r| r.id.as_ref())
                    .map(|id| id == &recipient_id)
                    .unwrap_or(false)
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
            if let Some(document) = get_document_metadata(document_id.clone()) {
                documents.push(document);
            }
        }
    }
    
    documents
}
