use ic_cdk::query;
use candid::Principal;
use crate::types::DocumentNft;

// ============================================================================
// NFT QUERY FUNCTIONS
// ============================================================================

/// Get NFT metadata by transaction ID (fast query, no file data)
#[query]
pub fn get_nft_metadata(tx_id: String) -> Option<DocumentNft> {
    crate::storage::get_document_nft_safe(&tx_id)
}

/// Get all NFT transaction IDs (fast query)
#[query]
pub fn get_all_nft_tx_ids() -> Vec<String> {
    crate::storage::DOCUMENT_NFTS.with(|storage| {
        storage.borrow().iter().map(|(k, _)| k.clone()).collect()
    })
}

/// Get NFT transaction ID for a specific document
#[query]
pub fn get_nft_tx_id_by_document(document_id: String) -> Option<String> {
    crate::storage::get_nft_by_document_id(&document_id)
}

/// Get NFTs owned by a specific principal
#[query]
pub fn get_nfts_by_owner(owner: Principal) -> Vec<DocumentNft> {
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
#[query]
pub fn get_nft_count() -> u64 {
    crate::storage::DOCUMENT_NFTS.with(|storage| {
        storage.borrow().len() as u64
    })
}

/// Get NFTs that have confirmed blockchain transactions (tx_id is Some)
#[query]
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
#[query]
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
