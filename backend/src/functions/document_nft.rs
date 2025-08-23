use ic_cdk::{update, api};
use candid::Principal;
use crate::types::{DocumentNft, NFTResponse};
use crate::storage::{create_document_nft_safe, link_document_to_nft, get_document_safe, get_nft_by_document_id};

/// Mint an NFT for an existing document using ICP's native transaction system
#[update]
pub async fn mint_document_nft(
    document_id: String,
) -> NFTResponse {
    // Validate that the document exists
    let document = match get_document_safe(&document_id) {
        Some(doc) => doc,
        None => {
            return NFTResponse {
                success: false,
                token_id: String::new(),
                error_message: "Document not found".to_string(),
            };
        }
    };

    // Check if an NFT already exists for this document
    if let Some(existing_tx_id) = get_nft_by_document_id(&document_id) {
        return NFTResponse {
            success: false,
            token_id: String::new(),
            error_message: format!("Document already has an NFT with transaction ID: {}", existing_tx_id),
        };
    }

    // Get current timestamp for NFT creation
    let created_at = api::time();

    // Create the DocumentNft from the document (without tx_id yet)
    let mut document_nft = DocumentNft {
        document_base_data: crate::types::DocumentBase {
            institution_id: document.document_base_data.institution_id.clone(),
            collection_id: document.document_base_data.collection_id.clone(),
            document_id: document.document_base_data.document_id.clone(),
            owner: document.document_base_data.owner.clone(),
            name: document.document_base_data.name.clone(),
            company_name: document.document_base_data.company_name.clone(),
            description: document.document_base_data.description.clone(),
            base_hash: document.document_base_data.base_hash.clone(),
            document_file_hash: document.document_base_data.document_file_hash.clone(),
            document_data: document.document_base_data.document_data.clone(),
        },
        created_at,
        tx_id: None, // Will be set to Some(real_tx_id) after blockchain confirmation
    };

    // Prepare NFT data for blockchain transaction
    let nft_payload = match serde_cbor::to_vec(&document_nft) {
        Ok(payload) => payload,
        Err(e) => {
            return NFTResponse {
                success: false,
                token_id: String::new(),
                error_message: format!("Failed to serialize NFT data: {}", e),
            };
        }
    };

    // Submit NFT to ICP blockchain using native transaction system
    let tx_result = match api::call_with_payment(
        Principal::management_canister(),
        "submit_nft_transaction",
        (nft_payload,),
        0, // No payment needed for this transaction
    ).await {
        Ok(result) => result,
        Err(e) => {
            return NFTResponse {
                success: false,
                token_id: String::new(),
                error_message: format!("Failed to submit NFT transaction: {}", e),
            };
        }
    };

    // Wait for transaction to be included in a block
    let tx_id = match wait_for_block_confirmation(&tx_result).await {
        Ok(confirmed_tx_id) => confirmed_tx_id,
        Err(e) => {
            return NFTResponse {
                success: false,
                token_id: String::new(),
                error_message: format!("Transaction confirmation failed: {}", e),
            };
        }
    };

    // Now update the NFT with the confirmed blockchain transaction ID
    document_nft.tx_id = Some(tx_id.clone());

    // Store the NFT with the confirmed blockchain transaction ID
    if let Err(e) = create_document_nft_safe(&tx_id, &document_nft) {
        return NFTResponse {
            success: false,
            token_id: String::new(),
            error_message: format!("Failed to store NFT: {}", e),
        };
    }

    // Link the document to the NFT
    if let Err(e) = link_document_to_nft(&document_id, &tx_id) {
        return NFTResponse {
            success: false,
            token_id: String::new(),
            error_message: format!("Failed to link document to NFT: {}", e),
        };
    }

    // Return success response with the actual blockchain transaction ID
    NFTResponse {
        success: true,
        token_id: tx_id,
        error_message: String::new(),
    }
}

/// Wait for transaction to be included in a block
async fn wait_for_block_confirmation(tx_result: &[u8]) -> Result<String, String> {
    // Parse transaction result to get transaction ID
    let tx_id = match serde_cbor::from_slice::<String>(tx_result) {
        Ok(id) => id,
        Err(_) => return Err("Invalid transaction result format".to_string()),
    };

    // Wait for block confirmation (this would need to be implemented based on ICP's confirmation mechanism)
    // For now, we'll simulate waiting for confirmation
    // In a real implementation, you would:
    // 1. Query the blockchain for transaction status
    // 2. Wait until transaction is included in a confirmed block
    // 3. Return the confirmed transaction ID
    
    // TODO: Implement actual block confirmation waiting
    // This is a placeholder - you'll need to implement the actual ICP confirmation mechanism
    
    Ok(tx_id)
}

// ============================================================================
// NFT QUERY FUNCTIONS
// ============================================================================

/// Get NFT metadata by transaction ID (fast query, no file data)
#[ic_cdk::query]
pub fn get_nft_metadata(tx_id: String) -> Option<DocumentNft> {
    crate::storage::get_document_nft_safe(&tx_id)
}

/// Get all NFT transaction IDs (fast query)
#[ic_cdk::query]
pub fn get_all_nft_tx_ids() -> Vec<String> {
    crate::storage::DOCUMENT_NFTS.with(|storage| {
        storage.borrow().iter().map(|(k, _)| k.clone()).collect()
    })
}

/// Get NFT transaction ID for a specific document
#[ic_cdk::query]
pub fn get_nft_tx_id_by_document(document_id: String) -> Option<String> {
    crate::storage::get_nft_by_document_id(&document_id)
}

/// Get NFTs owned by a specific principal
#[ic_cdk::query]
pub fn get_nfts_by_owner(owner: candid::Principal) -> Vec<DocumentNft> {
    let mut owner_nfts = Vec::new();
    
    crate::storage::DOCUMENT_NFTS.with(|storage| {
        for (_, nft_bytes) in storage.borrow().iter() {
            if let Ok(nft) = crate::storage::bytes_to_document_nft(&nft_bytes) {
                if nft.document_base_data.owner == owner {
                    owner_nfts.push(nft);
                }
            }
        }
    });
    
    owner_nfts
}

/// Get total number of NFTs
#[ic_cdk::query]
pub fn get_nft_count() -> u64 {
    crate::storage::DOCUMENT_NFTS.with(|storage| {
        storage.borrow().len() as u64
    })
}


/// Get NFTs that have confirmed blockchain transactions (tx_id is Some)
#[ic_cdk::query]
pub fn get_confirmed_nfts() -> Vec<DocumentNft> {
    let mut confirmed_nfts = Vec::new();
    
    crate::storage::DOCUMENT_NFTS.with(|storage| {
        for (_, nft_bytes) in storage.borrow().iter() {
            if let Ok(nft) = crate::storage::bytes_to_document_nft(&nft_bytes) {
                if nft.tx_id.is_some() {
                    confirmed_nfts.push(nft);
                }
            }
        }
    });
    
    confirmed_nfts
}

/// Get NFTs that are still pending blockchain confirmation (tx_id is None)
#[ic_cdk::query]
pub fn get_pending_nfts() -> Vec<DocumentNft> {
    let mut pending_nfts = Vec::new();
    
    crate::storage::DOCUMENT_NFTS.with(|storage| {
        for (_, nft_bytes) in storage.borrow().iter() {
            if let Ok(nft) = crate::storage::bytes_to_document_nft(&nft_bytes) {
                if nft.tx_id.is_none() {
                    pending_nfts.push(nft);
                }
            }
        }
    });
    
    pending_nfts
}
