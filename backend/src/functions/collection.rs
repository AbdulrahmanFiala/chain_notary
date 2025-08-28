use ic_cdk::{update, caller};
use crate::types::{CollectionMetadata, CollectionCategory};
use crate::storage::{COLLECTIONS, DOCUMENTS, bytes_to_collection};
use crate::utils::generate_collection_id;

#[update]
pub fn create_collection(
    name: String,
    description: String,
    external_url: String,
    category: CollectionCategory,
    institution_id: String, 
) -> Result<String, String> {
    let caller = caller();
    
    // Generate unique collection ID
    let collection_id = generate_collection_id();
    
    // Check if collection already exists (shouldn't happen with timestamp-based IDs, but safety check)
    if COLLECTIONS.with(|storage| storage.borrow().contains_key(&collection_id)) {
        return Err("Generated collection ID already exists. Please try again.".to_string());
    }
    
    // Validate name
    crate::utils::validate_string_length(&name, 1, 200, "Collection name")?;
    
    // Normalize and validate institution_id (trim whitespace and check if empty)
    let normalized_institution_id = institution_id.trim().to_string();
    
    // Validate institution if provided (non-empty institution_id after trimming)
    let final_institution_id = if !normalized_institution_id.is_empty() {
        // Check if institution exists
        let institution_exists = crate::storage::get_institution_safe(&normalized_institution_id).is_some();
        if !institution_exists {
            return Err("Specified institution does not exist".to_string());
        }
        normalized_institution_id
    } else {
        String::new() // No institution linked
    };
    
    let current_time = ic_cdk::api::time();
    
    let collection = CollectionMetadata {
        institution_id: final_institution_id.clone(),
        collection_id: collection_id.clone(),
        owner: caller,
        name,
        description,
        external_url,
        created_at: current_time,
        updated_at: current_time,
        category,
        documents: Vec::new(), // Start with empty documents list
    };
    
    // Store the collection
    crate::storage::update_collection_safe(&collection_id, &collection)?;
    
    // If institution is specified, add this collection to the institution
    if !final_institution_id.is_empty() {
        if let Some(mut institution) = crate::storage::get_institution_safe(&final_institution_id) {
            institution.collections.push(collection_id.clone());
            if let Err(e) = crate::storage::update_institution_safe(&final_institution_id, &institution) {
                return Err(format!("Failed to update institution: {}", e));
            }
        }
    }
    
    Ok(collection_id)
}

/// Update collection metadata
#[update]
pub fn update_collection(
    collection_id: String,
    name: String,
    description: String,
    external_url: String,
    category: CollectionCategory,
) -> Result<(), String> {
    let caller = caller();
    
    // Get existing collection
    let mut collection = crate::storage::get_collection_safe(&collection_id)
        .ok_or("Collection not found".to_string())?;
    
    // Check ownership
    if collection.owner != caller {
        return Err("Only collection owner can update collection".to_string());
    }
    
    // Update fields
    crate::utils::validate_string_length(&name, 1, 200, "Collection name")?;
    collection.name = name;
    collection.description = description;
    collection.external_url = external_url;
    collection.category = category;
    
    // Update timestamp
    collection.updated_at = ic_cdk::api::time();
    
    // Store updated collection
    crate::storage::update_collection_safe(&collection_id, &collection)?;
    
    Ok(())
}

/// Delete a collection (only if it has no documents)
#[update]
pub fn delete_collection(collection_id: String) -> Result<(), String> {
    let caller = caller();
    
    // Get existing collection
    let collection = crate::storage::get_collection_safe(&collection_id)
        .ok_or("Collection not found".to_string())?;
    
    // Check ownership
    if collection.owner != caller {
        return Err("Only collection owner can delete collection".to_string());
    }
    
    // Check if collection has documents
    if !collection.documents.is_empty() {
        return Err("Cannot delete collection with existing documents. Remove all documents first.".to_string());
    }
    
    // If collection is associated with an institution, remove it from the institution's collection list
    if !collection.institution_id.is_empty() {
        if let Some(mut institution) = crate::storage::get_institution_safe(&collection.institution_id) {
            // Remove collection from institution's collections list
            institution.collections.retain(|id| id != &collection_id);
            
            // Update the institution
            if let Err(e) = crate::storage::update_institution_safe(&collection.institution_id, &institution) {
                return Err(format!("Failed to update institution during collection deletion: {}", e));
            }
        }
    }
    
    // Delete the collection
    COLLECTIONS.with(|storage| {
        storage.borrow_mut().remove(&collection_id);
    });
    
    Ok(())
}

/// Add a document to a collection
#[update]
pub fn add_document_to_collection(collection_id: String, document_id: String) -> Result<(), String> {
    let caller = caller();
    
    // Check if collection exists
    let mut collection = COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id)
            .and_then(|bytes| bytes_to_collection(&bytes).ok())
            .ok_or("Collection not found".to_string())
    })?;
    
    // Check ownership
    if collection.owner != caller {
        return Err("Only collection owner can modify collection".to_string());
    }
    
    // Check if document exists
    let document = crate::storage::get_document_safe(&document_id)
        .ok_or("Document not found".to_string())?;
    
    // Check if document is already in collection
    if collection.documents.contains(&document_id) {
        return Err("Document is already in this collection".to_string());
    }
    
    // Add document to collection
    collection.documents.push(document_id.clone());
    collection.updated_at = ic_cdk::api::time();
    
    // Update collection
    crate::storage::update_collection_safe(&collection_id, &collection)?;
    
    // Update document's collection_id field and recalculate hashes
    let mut updated_document = document;
    updated_document.document_base_data.collection_id = collection_id.clone();
    updated_document.updated_at = ic_cdk::api::time();
    
    // Recalculate hashes since collection_id (metadata) has changed
    let base_hash = crate::utils::calculate_base_hash(&updated_document);
    let file_hash = crate::utils::calculate_document_file_hash(&updated_document.file_data);
    
    // Update the hashes in the document
    updated_document.document_base_data.base_hash = base_hash;
    updated_document.document_base_data.document_file_hash = file_hash;
    
    crate::storage::update_document_safe(&document_id, &updated_document)?;
    
    Ok(())
}

/// Remove a document from a collection
#[update]
pub fn remove_document_from_collection(collection_id: String, document_id: String) -> Result<(), String> {
    let caller = caller();
    
    // Check if collection exists
    let mut collection = COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id)
            .and_then(|bytes| bytes_to_collection(&bytes).ok())
            .ok_or("Collection not found".to_string())
    })?;
    
    // Check ownership
    if collection.owner != caller {
        return Err("Only collection owner can modify collection".to_string());
    }
    
    // Remove document from collection
    if let Some(pos) = collection.documents.iter().position(|id| id == &document_id) {
        collection.documents.remove(pos);
        collection.updated_at = ic_cdk::api::time();
        
        // Update collection
        crate::storage::update_collection_safe(&collection_id, &collection)?;
        
        // Update document's collection_id field to empty string and recalculate hashes
        if let Some(document) = crate::storage::get_document_safe(&document_id) {
            let mut updated_document = document;
            updated_document.document_base_data.collection_id = String::new();
            updated_document.updated_at = ic_cdk::api::time();
            
            // Recalculate hashes since collection_id (metadata) has changed
            let base_hash = crate::utils::calculate_base_hash(&updated_document);
            let file_hash = crate::utils::calculate_document_file_hash(&updated_document.file_data);
            
            // Update the hashes in the document
            updated_document.document_base_data.base_hash = base_hash;
            updated_document.document_base_data.document_file_hash = file_hash;
            
            crate::storage::update_document_safe(&document_id, &updated_document)?;
        }
        
        Ok(())
    } else {
        Err("Document is not in this collection".to_string())
    }
}


