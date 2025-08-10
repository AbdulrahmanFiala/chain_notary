use ic_cdk::update;
use crate::types::{NFTResponse, Document};
use crate::storage::{DOCUMENT_STORAGE, NFT_METADATA, OWNER_TOKENS, nft_info_to_bytes, document_to_bytes, principal_to_bytes, tokens_to_bytes, bytes_to_tokens};
use crate::utils::{calculate_file_hash, generate_token_id};

/// Custom upload endpoint for creating documents from file uploads
#[update]
pub async fn upload_file_and_create_nft(
    file_data: Vec<u8>,
    file_type: String,
    metadata: Document,
) -> NFTResponse {
    // Validate file size (max 2MB for demo)
    if file_data.len() > 2 * 1024 * 1024 {
        return NFTResponse {
            success: false,
            document_id: None,
            error_message: Some("File size exceeds 2MB limit".to_string()),
            document_hash: None,
        };
    }

    // Validate file type
    let allowed_types = vec!["image/jpeg", "image/png", "image/gif", "image/webp", "application/pdf", "text/plain"];
    if !allowed_types.contains(&file_type.as_str()) {
        return NFTResponse {
            success: false,
            document_id: None,
            error_message: Some("Unsupported file type. Only JPEG, PNG, GIF, WebP, PDF, and TXT are allowed.".to_string()),
            document_hash: None,
        };
    }

    // Generate unique document ID
    let document_id = generate_token_id();
    
    // Calculate file hash for integrity verification
    let document_hash = calculate_file_hash(&file_data);
    
    // Get current timestamp
    let uploaded_at = ic_cdk::api::time();

    // Create document metadata (lightweight, fast queries)
    let mut document = metadata;
    document.document_id = document_id.clone();
    document.document_hash = document_hash.clone();
    document.file_size = file_data.len() as u64;
    document.file_type = file_type.clone();
    document.minted_at = uploaded_at;
    document.file_data = Some(file_data.clone());

    // Create document with file data (separate storage)
    let document_storage = Document {
        document_id: document_id.clone(),
        collection_id: document.collection_id.clone(),
        owner: document.owner,
        name: document.name.clone(),
        description: document.description.clone(),
        image_url: document.image_url.clone(),
        document_hash: document.document_hash.clone(),
        file_size: document.file_size,
        file_type: document.file_type.clone(),
        file_data: Some(file_data),
        minted_at: uploaded_at,
        recipient: document.recipient.clone(),
    };

    // Store both metadata and document atomically
    // Store the document metadata first
    NFT_METADATA.with(|storage| {
        storage.borrow_mut().insert(document_id.clone(), nft_info_to_bytes(&document));
    });

    // Store the document file data separately
    DOCUMENT_STORAGE.with(|storage| {
        storage.borrow_mut().insert(document_id.clone(), document_to_bytes(&document_storage));
    });

    // Update owner's token list
    OWNER_TOKENS.with(|owner_tokens| {
        let mut tokens = owner_tokens.borrow_mut();
        let owner_bytes = principal_to_bytes(&document.owner);
        let current_tokens_bytes = tokens.get(&owner_bytes).unwrap_or_default();
        let mut current_tokens = bytes_to_tokens(&current_tokens_bytes);
        current_tokens.push(document_id.clone());
        tokens.insert(owner_bytes, tokens_to_bytes(&current_tokens));
    });

    NFTResponse {
        success: true,
        document_id: Some(document_id),
        error_message: None,
        document_hash: Some(document_hash),
    }
} 