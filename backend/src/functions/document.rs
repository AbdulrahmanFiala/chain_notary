use ic_cdk::update;
use crate::types::{DocumentResponse, Document};
use crate::storage::{DOCUMENTS, OWNER_TOKENS, COLLECTIONS, principal_to_bytes, tokens_to_bytes, bytes_to_tokens, document_to_bytes};
use crate::utils::{calculate_file_hash, generate_token_id};

/// Custom upload endpoint for publishing documents to the icp blockchain
#[update]
pub async fn upload_file_and_publish_document(
    metadata: Document,
) -> DocumentResponse {
    // Validate file size (max 10MB for Excel and other document types)
    if let Some(ref file_data) = metadata.file_data {
        if let Err(e) = crate::utils::validate_file_size(file_data.len(), 10) {
            return DocumentResponse {
                success: false,
                document_id: None,
                error_message: Some(e),
                document_hash: None,
            };
        }
    } else {
        return DocumentResponse {
            success: false,
            document_id: None,
            error_message: Some("File data is required".to_string()),
            document_hash: None,
        };
    }

    // Validate file type
    let allowed_types = vec![
        "image/jpeg", 
        "image/png", 
        "application/pdf", 
        "text/plain",
        "application/vnd.ms-excel",                    // .xls
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", // .xlsx
        "application/vnd.ms-excel.sheet.macroEnabled.12", // .xlsm
        "application/vnd.ms-excel.template.macroEnabled.12", // .xltm
        "application/vnd.ms-excel.addin.macroEnabled.12", // .xlam
        "application/vnd.ms-excel.sheet.binary.macroEnabled.12" // .xlsb
    ];
    if let Err(e) = crate::utils::validate_file_type(&metadata.file_type, &allowed_types) {
        return DocumentResponse {
            success: false,
            document_id: None,
            error_message: Some(e),
            document_hash: None,
        };
    }

    // Validate that collection exists if specified
    if let Some(ref collection_id) = metadata.collection_id {
        if !collection_id.is_empty() {
            let collection_exists = COLLECTIONS.with(|storage| {
                storage.borrow().contains_key(collection_id)
            });
            
            if !collection_exists {
                return DocumentResponse {
                    success: false,
                    document_id: None,
                    error_message: Some("Specified collection does not exist".to_string()),
                    document_hash: None,
                };
            }
        }
    }

    // Generate unique document ID
    let document_id = generate_token_id();
    
    // Calculate file hash for integrity verification and storage
    let calculated_hash = calculate_file_hash(&metadata.file_data.as_ref().unwrap());
    
    // Get current timestamp
    let uploaded_at = ic_cdk::api::time();

    // Create complete document with file data and calculated hash
    let mut document = metadata;
    document.document_id = document_id.clone();
    document.document_hash = Some(calculated_hash.clone()); // Set the calculated hash
    // No need to set file_size, file_type, or file_data since they're already in metadata

    // Store the complete document in single storage
    let document_bytes = match document_to_bytes(&document) {
        Ok(bytes) => bytes,
        Err(e) => {
            return DocumentResponse {
                success: false,
                document_id: None,
                error_message: Some(format!("Failed to serialize document: {}", e)),
                document_hash: None,
            };
        }
    };
    DOCUMENTS.with(|storage| {
        storage.borrow_mut().insert(document_id.clone(), document_bytes);
    });

    // If document belongs to a collection, add it to the collection's documents list
    if let Some(ref collection_id) = document.collection_id {
        if !collection_id.is_empty() {
            if let Some(mut collection) = crate::storage::get_collection_safe(collection_id) {
                collection.documents.push(document_id.clone());
                collection.updated_at = uploaded_at;
                if let Err(e) = crate::storage::update_collection_safe(collection_id, &collection) {
                    return DocumentResponse {
                        success: false,
                        document_id: None,
                        error_message: Some(format!("Failed to update collection: {}", e)),
                        document_hash: None,
                    };
                }
            }
        }
    }

    // Store owner token mapping
    OWNER_TOKENS.with(|storage| {
        let mut owner_tokens = storage.borrow_mut();
        let owner_bytes = principal_to_bytes(&document.owner);
        let current_tokens_bytes = owner_tokens.get(&owner_bytes).unwrap_or_default();
        let mut current_tokens = bytes_to_tokens(&current_tokens_bytes).unwrap_or_default();
        current_tokens.push(document_id.clone());
        if let Ok(tokens_bytes) = tokens_to_bytes(&current_tokens) {
            owner_tokens.insert(owner_bytes, tokens_bytes);
        }
    });

    // Return success response
    DocumentResponse {
        success: true,
        document_id: Some(document_id),
        error_message: None,
        document_hash: Some(calculated_hash),
    }
} 