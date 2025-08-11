use ic_cdk::{update, caller};
use crate::types::{MintArgs, MintResult, MintError, Document, RecipientInfo};
use crate::storage::{DOCUMENTS, OWNER_TOKENS, COLLECTIONS, document_to_bytes, principal_to_bytes, tokens_to_bytes, bytes_to_tokens, bytes_to_collection, collection_to_bytes};
use crate::utils::{generate_token_id, validate_document_metadata};
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
        if DOCUMENTS.with(|storage| storage.borrow().contains_key(&token_id)) {
            return Err(MintError::TokenExists);
        }

        // Determine collection ID from metadata
        let collection_id = args.metadata.as_ref()
            .and_then(|m| if m.collection_id.is_empty() { None } else { Some(m.collection_id.clone()) })
            .unwrap_or_default();

        // Validate that collection exists if specified
        if !collection_id.is_empty() {
            let collection_exists = COLLECTIONS.with(|storage| {
                storage.borrow().contains_key(&collection_id)
            });
            
            if !collection_exists {
                return Err(MintError::InvalidMetadata);
            }
        }

        // Create document metadata with default values
        let mut document = Document {
            collection_id: collection_id.clone(),
            document_id: token_id.clone(),
            owner: caller,
            name: args.metadata.as_ref().map(|m| m.name.clone()).unwrap_or_else(|| "Minted Document".to_string()),
            description: args.metadata.as_ref().and_then(|m| m.description.clone()),
            image_url: args.metadata.as_ref().and_then(|m| m.image_url.clone()),
            document_hash: args.metadata.as_ref()
                .and_then(|m| if m.document_hash.is_empty() { None } else { Some(m.document_hash.clone()) })
                .unwrap_or_else(|| format!("mint_{}", token_id)),
            file_size: args.metadata.as_ref().map(|m| m.file_size).unwrap_or(0),
            file_type: args.metadata.as_ref()
                .and_then(|m| if m.file_type.is_empty() { None } else { Some(m.file_type.clone()) })
                .unwrap_or_else(|| "application/octet-stream".to_string()),
            file_data: args.metadata.as_ref().and_then(|m| m.file_data.clone()),
            minted_at: ic_cdk::api::time(),
            recipient: args.metadata.as_ref().and_then(|m| m.recipient.clone()),
        };

        // Validate the document metadata
        if let Err(validation_error) = validate_document_metadata(&document) {
            return Err(MintError::InvalidMetadata);
        }

        // Store document metadata
        DOCUMENTS.with(|storage| {
            storage.borrow_mut().insert(token_id.clone(), document_to_bytes(&document));
        });

        // If document belongs to a collection, add it to the collection's documents list
        if !document.collection_id.is_empty() {
            COLLECTIONS.with(|storage| {
                if let Some(collection_bytes) = storage.borrow().get(&document.collection_id) {
                    if let Some(mut collection) = bytes_to_collection(&collection_bytes) {
                        collection.documents.push(token_id.clone());
                        collection.updated_at = document.minted_at;
                        storage.borrow_mut().insert(document.collection_id.clone(), collection_to_bytes(&collection));
                    }
                }
            });
        }

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