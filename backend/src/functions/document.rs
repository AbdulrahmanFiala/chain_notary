use ic_cdk::update;
use crate::types::{DocumentResponse, Document};
use crate::storage;
use crate::utils::{calculate_file_hash, generate_document_id, get_current_timestamp};

/// Custom upload endpoint for publishing documents to the icp blockchain
#[update]
pub async fn upload_file_and_publish_document(
    metadata: Document,
) -> DocumentResponse {
    // Validate file data is not empty
    if metadata.file_data.is_empty() {
        return DocumentResponse {
            success: false,
            document_id: String::new(),
            error_message: "File data cannot be empty. Please upload a valid file.".to_string(),
            file_hash: String::new(),
        };
    }
    
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


    // Normalize and validate institution_id (trim whitespace and check if empty)
    let normalized_institution_id = metadata.institution_id.trim().to_string();
    
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
    let document_id = generate_document_id();
    
    // Calculate file hash for integrity verification and storage
    let calculated_hash = calculate_file_hash(&metadata.file_data);
    
    // Get current timestamp (for potential future use)
    let _uploaded_at = get_current_timestamp();

    // Create complete document with file data and calculated hash, using normalized IDs
    let mut document = metadata;
    document.document_id = document_id.clone();
    document.file_hash = calculated_hash.clone();
    document.institution_id = normalized_institution_id;
    document.publication_date = get_current_timestamp();

    // Store the complete document using safe storage function
    if let Err(e) = crate::storage::store_document_safe(&document_id, &document) {
        return DocumentResponse {
            success: false,
            document_id: String::new(),
            error_message: format!("Failed to store document: {}", e),
            file_hash: String::new(),
        };
    }

    // Return success response
    DocumentResponse {
        success: true,
        document_id,
        error_message: String::new(),
        file_hash: calculated_hash,
    }
}

/// Delete a document
#[update]
pub async fn delete_document(document_id: String) -> DocumentResponse {
    // Require authentication
    let caller = match crate::utils::require_authenticated_user() {
        Ok(principal) => principal,
        Err(e) => {
            return DocumentResponse {
                success: false,
                document_id: String::new(),
                error_message: e,
                file_hash: String::new(),
            };
        }
    };

    // Get the document to check ownership
    let document = match crate::storage::get_document_safe(&document_id) {
        Some(doc) => doc,
        None => {
            return DocumentResponse {
                success: false,
                document_id: String::new(),
                error_message: "Document not found".to_string(),
                file_hash: String::new(),
            };
        }
    };

    // Check ownership
    if document.owner != caller {
        return DocumentResponse {
            success: false,
            document_id: String::new(),
            error_message: "Access denied. You can only delete your own documents.".to_string(),
            file_hash: String::new(),
        };
    }

    // Delete the document
    let deleted = crate::storage::DOCUMENTS.with(|storage| {
        storage.borrow_mut().remove(&crate::storage::memory::StorableString(document_id.clone()))
    });

    if deleted.is_some() {
        ic_cdk::println!("Document {} deleted by user {}", document_id, caller);
        DocumentResponse {
            success: true,
            document_id: document_id.clone(),
            error_message: String::new(),
            file_hash: document.file_hash,
        }
    } else {
        DocumentResponse {
            success: false,
            document_id: String::new(),
            error_message: "Failed to delete document".to_string(),
            file_hash: String::new(),
        }
    }
} 