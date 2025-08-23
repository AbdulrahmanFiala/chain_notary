use ic_cdk::update;
use crate::types::{DocumentResponse, Document};
use crate::storage::{DOCUMENTS, OWNER_TOKENS, COLLECTIONS, principal_to_bytes, tokens_to_bytes, bytes_to_tokens, document_to_bytes};
use crate::utils::{calculate_file_hash, calculate_base_hash, generate_token_id};

/// Custom upload endpoint for publishing documents to the icp blockchain
#[update]
pub fn upload_file_and_publish_document(
    metadata: Document,
) -> DocumentResponse {
    // Validate file size (max 10MB for Excel and other document types)
    if let Err(e) = crate::utils::validate_file_size(metadata.file_data.len(), 10) {
        return DocumentResponse {
            success: false,
            document_id: String::new(),
            error_message: e,
            file_hash: String::new(),
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
            };
        }
    }

    // Generate unique document ID
    let document_id = generate_token_id();
    
    // Calculate file hash for integrity verification and storage
    let calculated_hash = calculate_file_hash(&metadata.file_data);
    
    // Get current timestamp
    let uploaded_at = ic_cdk::api::time();

    // Create complete document with file data and calculated hash, using normalized IDs
    let mut document = metadata;
    document.document_base_data.document_id = document_id.clone();
    document.document_base_data.collection_id = normalized_collection_id;
    document.document_base_data.institution_id = normalized_institution_id;
    document.document_base_data.document_file_hash = calculated_hash.clone();
    
    // Calculate base hash from all DocumentBase attributes for integrity verification
    let base_hash = calculate_base_hash(
        &document.document_base_data.institution_id,
        &document.document_base_data.collection_id,
        &document.document_base_data.document_id,
        &document.document_base_data.owner,
        &document.document_base_data.name,
        &document.document_base_data.company_name,
        &document.document_base_data.description,
        &document.document_base_data.document_data,
    );
    document.document_base_data.base_hash = base_hash;

    // Store the complete document in single storage
    let document_bytes = match document_to_bytes(&document) {
        Ok(bytes) => bytes,
        Err(e) => {
            return DocumentResponse {
                success: false,
                document_id: String::new(),
                error_message: format!("Failed to serialize document: {}", e),
                file_hash: String::new(),
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
        file_hash: calculated_hash,
    }
} 