use ic_cdk::query;
use candid::Principal;
use crate::types::{CollectionMetadata, DocumentMetadata, Document, CollectionCategory, Certificate};
use crate::storage::{NFT_METADATA, DOCUMENT_STORAGE, OWNER_TOKENS, CERTIFICATES, bytes_to_nft_info, bytes_to_document, principal_to_bytes, bytes_to_tokens, bytes_to_certificate};

/// Get collection metadata
#[query]
pub fn icrc37_metadata() -> CollectionMetadata {
    CollectionMetadata {
        institution_id: "".to_string(),
        collection_id: "default".to_string(),
        owner: Principal::anonymous(),
        name: "Chain Notary Documents".to_string(),
        description: Some("Documents created on Chain Notary platform".to_string()),
        image_url: None,
        external_url: None,
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
        category: CollectionCategory::UniversityGraduationCertificate,
        documents: Vec::new(),
    }
}

/// Get document metadata by document ID (fast query)
#[query]
pub fn get_nft_metadata(document_id: String) -> Option<DocumentMetadata> {
    NFT_METADATA.with(|storage| {
        storage.borrow().get(&document_id).and_then(|bytes| bytes_to_nft_info(&bytes))
    })
}

/// Get document file data by document ID (loads file data)
#[query]
pub fn get_nft_file(document_id: String) -> Option<Vec<u8>> {
    DOCUMENT_STORAGE.with(|storage| {
        storage.borrow().get(&document_id).and_then(|bytes| {
            bytes_to_document(&bytes).map(|document| document.file_data)
        })
    })
}

/// Get complete document (metadata + file data) by document ID
#[query]
pub fn get_complete_document(document_id: String) -> Option<(DocumentMetadata, Vec<u8>)> {
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
pub fn get_documents_by_collection(collection_id: String) -> Vec<DocumentMetadata> {
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

/// Get certificate data for a document if it exists
#[query]
pub fn get_certificate_data(document_id: String) -> Option<Certificate> {
    CERTIFICATES.with(|storage| {
        storage.borrow().get(&document_id).and_then(|bytes| bytes_to_certificate(&bytes))
    })
}

/// Get all certificates (documents with certificate metadata)
#[query]
pub fn get_all_certificates() -> Vec<Certificate> {
    CERTIFICATES.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_certificate(&bytes))
            .collect()
    })
}

/// Get certificates by recipient name
#[query]
pub fn get_certificates_by_recipient(recipient_name: String) -> Vec<Certificate> {
    get_all_certificates()
        .into_iter()
        .filter(|cert| {
            cert.recipient_info
                .as_ref()
                .map(|info| info.name == recipient_name)
                .unwrap_or(false)
        })
        .collect()
}

/// Get documents by category
#[query]
pub fn get_documents_by_category(category: CollectionCategory) -> Vec<DocumentMetadata> {
    // For now, return all documents since we don't have category filtering implemented
    // This would need to be implemented based on collection categories
    NFT_METADATA.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_nft_info(&bytes))
            .collect()
    })
}

/// Get certificate metadata for a document
#[query]
pub fn get_certificate_metadata(document_id: String) -> Option<Certificate> {
    get_certificate_data(document_id)
} 