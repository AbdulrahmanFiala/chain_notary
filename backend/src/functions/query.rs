use ic_cdk::query;
use candid::Principal;
use crate::types::{CollectionMetadata, Document, CollectionCategory, Institution};
use crate::storage::{NFT_METADATA, DOCUMENT_STORAGE, OWNER_TOKENS, bytes_to_nft_info, bytes_to_document, principal_to_bytes, bytes_to_tokens};

/// Get collection metadata
#[query]
pub fn icrc37_metadata() -> CollectionMetadata {
    CollectionMetadata {
        institution_id: "chain_notary".to_string(),
        collection_id: "default".to_string(),
        owner: Principal::anonymous(),
        name: "Chain Notary Documents".to_string(),
        description: Some("Documents created on Chain Notary platform".to_string()),
        image_url: None,
        external_url: None,
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
        category: Some(CollectionCategory::UniversityGraduationCertificate),
        documents: Vec::new(),
    }
}

/// Get document metadata by document ID (fast query)
#[query]
pub fn get_nft_metadata(document_id: String) -> Option<Document> {
    NFT_METADATA.with(|storage| {
        storage.borrow().get(&document_id).and_then(|bytes| bytes_to_nft_info(&bytes))
    })
}

/// Get document file data by document ID (loads file data)
#[query]
pub fn get_nft_file(document_id: String) -> Option<Vec<u8>> {
    DOCUMENT_STORAGE.with(|storage| {
        storage.borrow().get(&document_id).and_then(|bytes| {
            bytes_to_document(&bytes).map(|document| document.file_data.unwrap_or_default())
        })
    })
}

/// Get complete document (metadata + file data) by document ID
#[query]
pub fn get_complete_document(document_id: String) -> Option<(Document, Vec<u8>)> {
    let metadata = get_nft_metadata(document_id.clone())?;
    let file_data = get_nft_file(document_id)?;
    Some((metadata, file_data))
}

/// Get document object by document ID (includes file data)
#[query]
pub fn get_document(document_id: String) -> Option<Document> {
    DOCUMENT_STORAGE.with(|storage| {
        storage.borrow().get(&document_id).and_then(|bytes| bytes_to_document(&bytes))
    })
}

/// List all document IDs (fast query)
#[query]
pub fn list_all_nfts() -> Vec<String> {
    NFT_METADATA.with(|storage| {
        storage.borrow().iter().map(|(k, _)| k.clone()).collect()
    })
}

/// Get documents owned by a specific principal (fast query)
#[query]
pub fn get_nfts_by_owner(owner: Principal) -> Vec<String> {
    OWNER_TOKENS.with(|owner_tokens| {
        let owner_bytes = principal_to_bytes(&owner);
        owner_tokens.borrow().get(&owner_bytes).map(|bytes| bytes_to_tokens(&bytes)).unwrap_or_default()
    })
}

/// Get total number of documents (fast query)
#[query]
pub fn get_nft_count() -> u64 {
    NFT_METADATA.with(|storage| {
        storage.borrow().len() as u64
    })
}

/// Get document file size by document ID (fast query)
#[query]
pub fn get_document_file_size(document_id: String) -> Option<u64> {
    NFT_METADATA.with(|storage| {
        storage.borrow().get(&document_id).and_then(|bytes| {
            bytes_to_nft_info(&bytes).map(|metadata| metadata.file_size)
        })
    })
}

/// Get document file type by document ID (fast query)
#[query]
pub fn get_document_file_type(document_id: String) -> Option<String> {
    NFT_METADATA.with(|storage| {
        storage.borrow().get(&document_id).and_then(|bytes| {
            bytes_to_nft_info(&bytes).map(|metadata| metadata.file_type)
        })
    })
}

/// Get documents by collection ID (fast query)
#[query]
pub fn get_documents_by_collection(collection_id: String) -> Vec<Document> {
    NFT_METADATA.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_nft_info(&bytes))
            .filter(|metadata| metadata.collection_id == collection_id)
            .collect()
    })
}

/// Get total supply (same as count for documents)
#[query]
pub fn get_total_supply() -> u64 {
    get_nft_count()
}

/// Get documents by category
#[query]
pub fn get_documents_by_category(category: CollectionCategory) -> Vec<Document> {
    // For now, return all documents since we don't have category filtering implemented
    // This would need to be implemented based on collection categories
    NFT_METADATA.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_nft_info(&bytes))
            .collect()
    })
}

/// Get all documents with their metadata (for collection building)
#[query]
pub fn get_all_documents_with_metadata() -> Vec<Document> {
    NFT_METADATA.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_nft_info(&bytes))
            .collect()
    })
}

/// Get documents by recipient name
#[query]
pub fn get_documents_by_recipient(recipient_name: String) -> Vec<Document> {
    NFT_METADATA.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_nft_info(&bytes))
            .filter(|doc| {
                doc.recipient.as_ref()
                    .map(|r| r.name == recipient_name)
                    .unwrap_or(false)
            })
            .collect()
    })
}

/// Get documents by recipient email
#[query]
pub fn get_documents_by_recipient_email(recipient_email: String) -> Vec<Document> {
    NFT_METADATA.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_nft_info(&bytes))
            .filter(|doc| {
                doc.recipient.as_ref()
                    .and_then(|r| r.email.as_ref())
                    .map(|email| email == &recipient_email)
                    .unwrap_or(false)
            })
            .collect()
    })
}

/// Get collection metadata by collection ID
#[query]
pub fn get_collection_metadata(collection_id: String) -> Option<CollectionMetadata> {
    // Get all documents in the collection
    let documents = get_documents_by_collection(collection_id.clone());
    
    if documents.is_empty() {
        return None;
    }
    
    // Build collection metadata from documents
    let first_doc = documents.first()?;
    Some(CollectionMetadata {
        institution_id: first_doc.collection_id.clone(),
        collection_id,
        owner: first_doc.owner,
        name: format!("Collection: {}", first_doc.collection_id),
        description: Some(format!("Collection containing {} documents", documents.len())),
        image_url: None,
        external_url: None,
        created_at: documents.iter().map(|d| d.minted_at).min().unwrap_or(0),
        updated_at: documents.iter().map(|d| d.minted_at).max().unwrap_or(0),
        category: Some(CollectionCategory::UniversityGraduationCertificate), // Default category
        documents: documents.into_iter().map(|d| d.document_id).collect(),
    })
}

/// Get all collection IDs
#[query]
pub fn get_all_collection_ids() -> Vec<String> {
    let mut collection_ids = std::collections::HashSet::new();
    
    NFT_METADATA.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_nft_info(&bytes))
            .for_each(|doc| {
                if !doc.collection_id.is_empty() {
                    collection_ids.insert(doc.collection_id);
                }
            });
    });
    
    collection_ids.into_iter().collect()
}

/// Get documents by file type
#[query]
pub fn get_documents_by_file_type(file_type: String) -> Vec<Document> {
    NFT_METADATA.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_nft_info(&bytes))
            .filter(|doc| doc.file_type == file_type)
            .collect()
    })
}

/// Get documents by file size range
#[query]
pub fn get_documents_by_file_size_range(min_size: u64, max_size: u64) -> Vec<Document> {
    NFT_METADATA.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_nft_info(&bytes))
            .filter(|doc| doc.file_size >= min_size && doc.file_size <= max_size)
            .collect()
    })
} 