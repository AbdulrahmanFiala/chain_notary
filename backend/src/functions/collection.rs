use ic_cdk::{update, caller};
use crate::types::{CollectionMetadata, CollectionCategory};
use crate::storage::{COLLECTIONS, DOCUMENTS, bytes_to_collection};

#[update]
pub fn create_collection(
    collection_id: String,
    name: String,
    description: Option<String>,
    external_url: Option<String>,
    category: Option<CollectionCategory>,
    institution_id: Option<String>, 
) -> Result<(), String> {
    let caller = caller();
    
    // Check if collection already exists
    if COLLECTIONS.with(|storage| storage.borrow().contains_key(&collection_id)) {
        return Err("Collection with this ID already exists".to_string());
    }
    
    // Validate collection ID format
    crate::utils::validate_string_length(&collection_id, 1, 100, "Collection ID")?;
    
    // Validate name
    crate::utils::validate_string_length(&name, 1, 200, "Collection name")?;
    
    // Validate institution if provided
    let final_institution_id = if let Some(inst_id) = institution_id {
        if inst_id.is_empty() {
            return Err("Institution ID cannot be empty if provided".to_string());
        }
        // Check if institution exists
        let institution_exists = crate::storage::get_institution_safe(&inst_id).is_some();
        if !institution_exists {
            return Err("Specified institution does not exist".to_string());
        }
        inst_id
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
    
    Ok(())
}

/// Update collection metadata
#[update]
pub fn update_collection(
    collection_id: String,
    name: Option<String>,
    description: Option<String>,
    external_url: Option<String>,
    category: Option<CollectionCategory>,
) -> Result<(), String> {
    let caller = caller();
    
    // Get existing collection
    let mut collection = crate::storage::get_collection_safe(&collection_id)
        .ok_or("Collection not found".to_string())?;
    
    // Check ownership
    if collection.owner != caller {
        return Err("Only collection owner can update collection".to_string());
    }
    
    // Update fields if provided
    if let Some(new_name) = name {
        crate::utils::validate_string_length(&new_name, 1, 200, "Collection name")?;
        collection.name = new_name;
    }
    
    if let Some(new_description) = description {
        collection.description = Some(new_description);
    }
    

    
    if let Some(new_external_url) = external_url {
        collection.external_url = Some(new_external_url);
    }
    
    if let Some(new_category) = category {
        collection.category = Some(new_category);
    }
    
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
    
    // Update document's collection_id field
    let mut updated_document = document;
    updated_document.collection_id = Some(collection_id.clone());
    
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
        
        // Update document's collection_id field to None
        if let Some(document) = crate::storage::get_document_safe(&document_id) {
            let mut updated_document = document;
            updated_document.collection_id = None;
            
            crate::storage::update_document_safe(&document_id, &updated_document)?;
        }
        
        Ok(())
    } else {
        Err("Document is not in this collection".to_string())
    }
}


