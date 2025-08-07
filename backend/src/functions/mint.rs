use ic_cdk::{update, caller};
use crate::types::{MintArgs, MintResult, MintError, DocumentMetadata, Certificate, RecipientInfo};
use crate::storage::{NFT_METADATA, OWNER_TOKENS, CERTIFICATES, nft_info_to_bytes, principal_to_bytes, tokens_to_bytes, bytes_to_tokens, certificate_to_bytes};
use crate::utils::generate_token_id;
use std::collections::HashMap;

/// ICRC-37 Standard Implementation for minting documents
#[update]
pub fn icrc37_mint(args: MintArgs) -> MintResult {
    // Validate mint arguments
    if args.token_ids.is_empty() {
        return Err(MintError::InvalidTokenId);
    }

    let caller = caller();
    
    for token_id in args.token_ids {
        // Check if token already exists
        if NFT_METADATA.with(|storage| storage.borrow().contains_key(&token_id)) {
            return Err(MintError::TokenExists);
        }

        // Create document metadata with default values
        let document_metadata = DocumentMetadata {
            collection_id: "".to_string(),
            document_id: token_id.clone(),
            owner: caller,
            name: args.metadata.as_ref().map(|m| m.name.clone()).unwrap_or_else(|| "Minted Document".to_string()),
            description: args.metadata.as_ref().and_then(|m| m.description.clone()),
            image_url: args.metadata.as_ref().and_then(|m| m.image_url.clone()),
            document_hash: "".to_string(),
            file_size: 0,  // No file data for minted documents
            file_type: "".to_string(),  // No file type for minted documents
            minted_at: ic_cdk::api::time(),
        };

        // Store document metadata
        NFT_METADATA.with(|storage| {
            storage.borrow_mut().insert(token_id.clone(), nft_info_to_bytes(&document_metadata));
        });

        // Update owner's token list
        OWNER_TOKENS.with(|owner_tokens| {
            let mut tokens = owner_tokens.borrow_mut();
            let caller_bytes = principal_to_bytes(&caller);
            let current_tokens_bytes = tokens.get(&caller_bytes).unwrap_or_default();
            let mut current_tokens = bytes_to_tokens(&current_tokens_bytes);
            current_tokens.push(token_id);
            tokens.insert(caller_bytes, tokens_to_bytes(&current_tokens));
        });
    }

    Ok(())
}

/// Example function showing how to mint a document with certificate metadata
#[update]
pub fn mint_certificate(
    token_id: String,
    certificate_data: CertificateData,
) -> MintResult {
    let caller = caller();
    
    // Check if token already exists
    if NFT_METADATA.with(|storage| storage.borrow().contains_key(&token_id)) {
        return Err(MintError::TokenExists);
    }

    // Create document metadata first
    let document_metadata = DocumentMetadata {
        collection_id: certificate_data.collection_id,
        document_id: token_id.clone(),
        owner: caller,
        name: certificate_data.name,
        description: certificate_data.description,
        image_url: certificate_data.image_url,
        document_hash: certificate_data.document_hash,
        file_size: certificate_data.file_size,
        file_type: certificate_data.file_type,
        minted_at: ic_cdk::api::time(),
    };

    // Create certificate metadata
    let certificate = Certificate {
        document_id: token_id.clone(),
        recipient_info: certificate_data.recipient_info,
        issued_date: certificate_data.issued_date,
        expiry_date: certificate_data.expiry_date,
    };

    // Store document metadata
    NFT_METADATA.with(|storage| {
        storage.borrow_mut().insert(token_id.clone(), nft_info_to_bytes(&document_metadata));
    });

    // Store certificate separately
    CERTIFICATES.with(|storage| {
        storage.borrow_mut().insert(token_id.clone(), certificate_to_bytes(&certificate));
    });

    // Update owner's token list
    OWNER_TOKENS.with(|owner_tokens| {
        let mut tokens = owner_tokens.borrow_mut();
        let caller_bytes = principal_to_bytes(&caller);
        let current_tokens_bytes = tokens.get(&caller_bytes).unwrap_or_default();
        let mut current_tokens = bytes_to_tokens(&current_tokens_bytes);
        current_tokens.push(token_id);
        tokens.insert(caller_bytes, tokens_to_bytes(&current_tokens));
    });

    Ok(())
}

// Helper struct for certificate data input
#[derive(candid::CandidType, candid::Deserialize)]
pub struct CertificateData {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub collection_id: String,
    pub document_hash: String,
    pub file_size: u64,
    pub file_type: String,
    pub issued_date: u64,
    pub expiry_date: Option<u64>,
    pub recipient_info: Option<RecipientInfo>,
} 