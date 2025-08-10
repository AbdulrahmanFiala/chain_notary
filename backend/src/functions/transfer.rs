use ic_cdk::{update, caller};
use crate::types::{
    TransferArgs, TransferResult, TransferError,
    ApproveArgs, ApproveResult, ApproveError,
    TransferFromArgs, TransferFromResult, TransferFromError,
    BurnArgs, BurnResult, BurnError, Document
};
use crate::storage::{
    NFT_METADATA, DOCUMENT_STORAGE, OWNER_TOKENS, APPROVALS,
    nft_info_to_bytes, bytes_to_nft_info,
    principal_to_bytes, tokens_to_bytes, bytes_to_tokens
};

/// Transfer document to another principal
#[update]
pub fn icrc37_transfer(args: TransferArgs) -> TransferResult {
    let caller = caller();
    
    // Check if document exists and caller owns it
    let document_metadata_bytes = NFT_METADATA.with(|storage| {
        storage.borrow().get(&args.token_id)
    });

    if document_metadata_bytes.is_none() {
        return Err(TransferError::TokenNotFound);
    }

    let document_metadata = bytes_to_nft_info(&document_metadata_bytes.unwrap());
    if document_metadata.is_none() {
        return Err(TransferError::TokenNotFound);
    }
    
    let mut document_metadata = document_metadata.unwrap();
    if document_metadata.owner != caller {
        return Err(TransferError::Unauthorized);
    }

    let old_owner = document_metadata.owner;

    // Remove from old owner
    OWNER_TOKENS.with(|owner_tokens| {
        let mut tokens = owner_tokens.borrow_mut();
        let old_owner_bytes = principal_to_bytes(&old_owner);
        let current_tokens_bytes = tokens.get(&old_owner_bytes).unwrap_or_default();
        let mut current_tokens = bytes_to_tokens(&current_tokens_bytes);
        current_tokens.retain(|id| id != &args.token_id);
        tokens.insert(old_owner_bytes, tokens_to_bytes(&current_tokens));
    });

    // Update owner in metadata
    document_metadata.owner = args.to;
    NFT_METADATA.with(|storage| {
        storage.borrow_mut().insert(args.token_id.clone(), nft_info_to_bytes(&document_metadata));
    });

    // Add to new owner
    OWNER_TOKENS.with(|owner_tokens| {
        let mut tokens = owner_tokens.borrow_mut();
        let new_owner_bytes = principal_to_bytes(&args.to);
        let current_tokens_bytes = tokens.get(&new_owner_bytes).unwrap_or_default();
        let mut current_tokens = bytes_to_tokens(&current_tokens_bytes);
        current_tokens.push(args.token_id);
        tokens.insert(new_owner_bytes, tokens_to_bytes(&current_tokens));
    });

    Ok(())
}

/// Approve another principal to transfer your document
#[update]
pub fn icrc37_approve(args: ApproveArgs) -> ApproveResult {
    let caller = caller();
    
    // Check if caller owns the document
    let document_metadata_bytes = NFT_METADATA.with(|storage| {
        storage.borrow().get(&args.token_id)
    });

    if document_metadata_bytes.is_none() {
        return Err(ApproveError::TokenNotFound);
    }

    let document_metadata = bytes_to_nft_info(&document_metadata_bytes.unwrap());
    if document_metadata.is_none() {
        return Err(ApproveError::TokenNotFound);
    }
    
    let document_metadata = document_metadata.unwrap();
    if document_metadata.owner != caller {
        return Err(ApproveError::Unauthorized);
    }

    // Store approval
    APPROVALS.with(|approvals| {
        approvals.borrow_mut().insert(args.token_id, principal_to_bytes(&args.spender));
    });

    Ok(())
}

/// Transfer document on behalf of another principal (using approval)
#[update]
pub fn icrc37_transfer_from(args: TransferFromArgs) -> TransferFromResult {
    let caller = caller();
    
    // Check if caller is approved or owner
    let is_approved = APPROVALS.with(|approvals| {
        approvals.borrow().get(&args.token_id) == Some(principal_to_bytes(&caller))
    });

    let document_metadata_bytes = NFT_METADATA.with(|storage| {
        storage.borrow().get(&args.token_id)
    });

    if document_metadata_bytes.is_none() {
        return Err(TransferFromError::TokenNotFound);
    }

    let document_metadata = bytes_to_nft_info(&document_metadata_bytes.unwrap());
    if document_metadata.is_none() {
        return Err(TransferFromError::TokenNotFound);
    }

    let document_metadata = document_metadata.unwrap();
    let is_owner = document_metadata.owner == caller;

    if !is_approved && !is_owner {
        return Err(TransferFromError::Unauthorized);
    }

    // Perform transfer
    match icrc37_transfer(TransferArgs {
        token_id: args.token_id.clone(),
        to: args.to,
    }) {
        Ok(()) => {
            // Remove approval after transfer
            APPROVALS.with(|approvals| {
                approvals.borrow_mut().remove(&args.token_id);
            });
            Ok(())
        }
        Err(TransferError::TokenNotFound) => Err(TransferFromError::TokenNotFound),
        Err(TransferError::Unauthorized) => Err(TransferFromError::Unauthorized),
        Err(TransferError::InvalidRecipient) => Err(TransferFromError::InvalidRecipient),
    }
}

/// Burn (destroy) a document
#[update]
pub fn icrc37_burn(args: BurnArgs) -> BurnResult {
    let caller = caller();
    
    // Check if caller owns the document
    let document_metadata_bytes = NFT_METADATA.with(|storage| {
        storage.borrow().get(&args.token_id)
    });

    if document_metadata_bytes.is_none() {
        return Err(BurnError::TokenNotFound);
    }

    let document_metadata = bytes_to_nft_info(&document_metadata_bytes.unwrap());
    if document_metadata.is_none() {
        return Err(BurnError::TokenNotFound);
    }
    
    let document_metadata = document_metadata.unwrap();
    if document_metadata.owner != caller {
        return Err(BurnError::Unauthorized);
    }

    // Remove from metadata storage
    NFT_METADATA.with(|storage| {
        storage.borrow_mut().remove(&args.token_id);
    });

    // Remove from document storage
    DOCUMENT_STORAGE.with(|storage| {
        storage.borrow_mut().remove(&args.token_id);
    });

    // Remove from owner's token list
    OWNER_TOKENS.with(|owner_tokens| {
        let mut tokens = owner_tokens.borrow_mut();
        let caller_bytes = principal_to_bytes(&caller);
        let current_tokens_bytes = tokens.get(&caller_bytes).unwrap_or_default();
        let mut current_tokens = bytes_to_tokens(&current_tokens_bytes);
        current_tokens.retain(|id| id != &args.token_id);
        tokens.insert(caller_bytes, tokens_to_bytes(&current_tokens));
    });

    // Remove approval if exists
    APPROVALS.with(|approvals| {
        approvals.borrow_mut().remove(&args.token_id);
    });

    Ok(())
} 