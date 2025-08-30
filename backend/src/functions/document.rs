use ic_cdk::update;
use crate::types::{DocumentResponse, Document};
use crate::storage::{DOCUMENTS, OWNER_TOKENS, COLLECTIONS, principal_to_bytes, tokens_to_bytes, bytes_to_tokens, document_to_bytes};
use crate::utils::{calculate_file_hash, generate_token_id, calculate_base_hash, calculate_document_file_hash, calculate_combined_document_hash};

/// Custom upload endpoint for publishing documents to the icp blockchain
#[update]
pub async fn upload_file_and_publish_document(
    metadata: Document,
) -> DocumentResponse {
    // Validate file size (max 10MB for Excel and other document types)
    if let Err(e) = crate::utils::validate_file_size(metadata.file_data.len(), 10) {
        return DocumentResponse {
            success: false,
            document_id: String::new(),
            error_message: e,
            file_hash: String::new(),
            base_hash: String::new(),
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
            document_id: String::new(),
            error_message: e,
            file_hash: String::new(),
            base_hash: String::new(),
        };
    }

    // Normalize and validate collection_id (trim whitespace and check if empty)
    let normalized_collection_id = metadata.document_base_data.collection_id.trim().to_string();
    
    // Validate that collection exists if specified (non-empty collection_id after trimming)
    if !normalized_collection_id.is_empty() {
        let collection_exists = COLLECTIONS.with(|storage| {
            storage.borrow().contains_key(&normalized_collection_id)
        });
        
        if !collection_exists {
            return DocumentResponse {
                success: false,
                document_id: String::new(),
                error_message: "Specified collection does not exist".to_string(),
                file_hash: String::new(),
                base_hash: String::new(),
            };
        }
    }

    // Normalize and validate institution_id (trim whitespace and check if empty)
    let normalized_institution_id = metadata.document_base_data.institution_id.trim().to_string();
    
    // Validate that institution exists if specified (non-empty institution_id after trimming)
    if !normalized_institution_id.is_empty() {
        let institution_exists = crate::storage::get_institution_safe(&normalized_institution_id).is_some();
        
        if !institution_exists {
            return DocumentResponse {
                success: false,
                document_id: String::new(),
                error_message: "Specified institution does not exist".to_string(),
                file_hash: String::new(),
                base_hash: String::new(),
            };
        }
    }

    // Generate unique document ID
    let document_id = generate_token_id();
    
    // Get current timestamp
    let uploaded_at = ic_cdk::api::time();

    // Create complete document with normalized IDs first
    let mut document = metadata;
    document.document_base_data.document_id = document_id.clone();
    document.document_base_data.collection_id = normalized_collection_id;
    document.document_base_data.institution_id = normalized_institution_id;
    
    // Set timestamp fields
    document.published_at = uploaded_at;
    document.updated_at = uploaded_at;
    
    // Calculate all hashes after setting all metadata
    let base_hash = calculate_base_hash(&document);
    let file_hash = calculate_document_file_hash(&document.file_data);
    let combined_hash = calculate_combined_document_hash(&base_hash, &file_hash);
    
    // Set all calculated hashes in the DocumentBase structure
    document.document_base_data.base_hash = base_hash.clone();
    document.document_base_data.document_file_hash = file_hash;

    // Store the complete document in single storage
    let document_bytes = match document_to_bytes(&document) {
        Ok(bytes) => bytes,
        Err(e) => {
            return DocumentResponse {
                success: false,
                document_id: String::new(),
                error_message: format!("Failed to serialize document: {}", e),
                file_hash: String::new(),
                base_hash: String::new(),
            };
        }
    };
    DOCUMENTS.with(|storage| {
        storage.borrow_mut().insert(document_id.clone(), document_bytes);
    });

    // If document belongs to a collection, add it to the collection's documents list
    if !document.document_base_data.collection_id.is_empty() {
        if let Some(mut collection) = crate::storage::get_collection_safe(&document.document_base_data.collection_id) {
            collection.documents.push(document_id.clone());
            collection.updated_at = uploaded_at;
            if let Err(e) = crate::storage::update_collection_safe(&document.document_base_data.collection_id, &collection) {
                return DocumentResponse {
                    success: false,
                    document_id: String::new(),
                    error_message: format!("Failed to update collection: {}", e),
                    file_hash: String::new(),
                    base_hash: String::new(),
                };
            }
        }
    }

    // Store owner token mapping
    OWNER_TOKENS.with(|storage| {
        let mut owner_tokens = storage.borrow_mut();
        let owner_bytes = principal_to_bytes(&document.document_base_data.owner);
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
        document_id,
        error_message: String::new(),
        file_hash: combined_hash,
        base_hash: base_hash,
    }
}

/// Update document metadata and/or file content with automatic hash recalculation
#[update]
pub async fn update_document(
    document_id: String,
    updated_document: Document,
) -> DocumentResponse {
    let caller = ic_cdk::caller();
    
    // Get existing document
    let existing_document = match crate::storage::get_document_safe(&document_id) {
        Some(doc) => doc,
        None => {
            return DocumentResponse {
                success: false,
                document_id: String::new(),
                error_message: "Document not found".to_string(),
                file_hash: String::new(),
                base_hash: String::new(),
            };
        }
    };
    
    // Check ownership
    if existing_document.document_base_data.owner != caller {
        return DocumentResponse {
            success: false,
            document_id: String::new(),
            error_message: "Only document owner can update document".to_string(),
            file_hash: String::new(),
            base_hash: String::new(),
        };
    }
    
    // Validate file size if file data is being updated
    if !updated_document.file_data.is_empty() {
        if let Err(e) = crate::utils::validate_file_size(updated_document.file_data.len(), 10) {
            return DocumentResponse {
                success: false,
                document_id: String::new(),
                error_message: e,
                file_hash: String::new(),
                base_hash: String::new(),
            };
        }
    }
    
    // Validate file type if file is being updated
    if !updated_document.file_type.is_empty() {
        let allowed_types = vec![
            "image/jpeg", 
            "image/png", 
            "application/pdf", 
            "text/plain",
            "application/vnd.ms-excel",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "application/vnd.ms-excel.sheet.macroEnabled.12",
            "application/vnd.ms-excel.template.macroEnabled.12",
            "application/vnd.ms-excel.addin.macroEnabled.12",
            "application/vnd.ms-excel.sheet.binary.macroEnabled.12"
        ];
        if let Err(e) = crate::utils::validate_file_type(&updated_document.file_type, &allowed_types) {
            return DocumentResponse {
                success: false,
                document_id: String::new(),
                error_message: e,
                file_hash: String::new(),
                base_hash: String::new(),
            };
        }
    }
    
    // Create the updated document with preserved fields where appropriate
    let mut final_document = updated_document;
    final_document.document_base_data.document_id = document_id.clone(); // Preserve original document ID
    final_document.document_base_data.owner = existing_document.document_base_data.owner; // Preserve ownership
    
    // Use existing file data if no new file is provided
    if final_document.file_data.is_empty() {
        final_document.file_data = existing_document.file_data;
        final_document.file_size = existing_document.file_size;
        final_document.file_type = existing_document.file_type;
    } else {
        // Update file size if new file is provided
        final_document.file_size = final_document.file_data.len() as u64;
    }
    
    // Update timestamp to reflect modification
    final_document.updated_at = ic_cdk::api::time();
    
    // Recalculate all hashes since metadata or file content may have changed
    let base_hash = calculate_base_hash(&final_document);
    let file_hash = calculate_document_file_hash(&final_document.file_data);
    let combined_hash = calculate_combined_document_hash(&base_hash, &file_hash);
    
    // Set the calculated hashes
    final_document.document_base_data.base_hash = base_hash.clone();
    final_document.document_base_data.document_file_hash = file_hash;
    
    // Update the document in storage
    if let Err(e) = crate::storage::update_document_safe(&document_id, &final_document) {
        return DocumentResponse {
            success: false,
            document_id: String::new(),
            error_message: format!("Failed to update document: {}", e),
            file_hash: String::new(),
            base_hash: String::new(),
        };
    }
    
    // Return success response
    DocumentResponse {
        success: true,
        document_id,
        error_message: String::new(),
        file_hash: combined_hash,
        base_hash: base_hash,
    }
} 