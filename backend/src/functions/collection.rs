use ic_cdk::{update, query, caller};
use crate::types::{CollectionMetadata, CollectionCategory, Document};
use crate::storage::{COLLECTIONS, DOCUMENTS, collection_to_bytes, bytes_to_collection, document_to_bytes, bytes_to_document};
use std::collections::HashMap;

/// Create a new collection with metadata
#[update]
pub fn create_collection(
    collection_id: String,
    name: String,
    description: Option<String>,
    image_url: Option<String>,
    external_url: Option<String>,
    category: Option<CollectionCategory>,
    institution_id: Option<String>, // Optional institution to link to
) -> Result<(), String> {
    let caller = caller();
    
    // Check if collection already exists
    if COLLECTIONS.with(|storage| storage.borrow().contains_key(&collection_id)) {
        return Err("Collection with this ID already exists".to_string());
    }
    
    // Validate collection ID format
    if collection_id.is_empty() || collection_id.len() > 100 {
        return Err("Collection ID must be between 1 and 100 characters".to_string());
    }
    
    // Validate name
    if name.is_empty() || name.len() > 200 {
        return Err("Collection name must be between 1 and 200 characters".to_string());
    }
    
    // Validate institution if provided
    let final_institution_id = if let Some(inst_id) = institution_id {
        if inst_id.is_empty() {
            return Err("Institution ID cannot be empty if provided".to_string());
        }
        // Check if institution exists
        use crate::storage::INSTITUTIONS;
        let institution_exists = INSTITUTIONS.with(|storage| {
            storage.borrow().contains_key(&inst_id)
        });
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
        image_url,
        external_url,
        created_at: current_time,
        updated_at: current_time,
        category,
        documents: Vec::new(), // Start with empty documents list
    };
    
    // Store the collection
    COLLECTIONS.with(|storage| {
        storage.borrow_mut().insert(collection_id.clone(), collection_to_bytes(&collection));
    });
    
    // If institution is specified, add this collection to the institution
    if !final_institution_id.is_empty() {
        use crate::storage::INSTITUTIONS;
        use crate::storage::institution_to_bytes;
        INSTITUTIONS.with(|storage| {
            if let Some(inst_bytes) = storage.borrow().get(&final_institution_id) {
                if let Some(mut institution) = crate::storage::bytes_to_institution(&inst_bytes) {
                    institution.collections.push(collection_id.clone());
                    storage.borrow_mut().insert(final_institution_id, institution_to_bytes(&institution));
                }
            }
        });
    }
    
    Ok(())
}

/// Update collection metadata
#[update]
pub fn update_collection(
    collection_id: String,
    name: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    external_url: Option<String>,
    category: Option<CollectionCategory>,
) -> Result<(), String> {
    let caller = caller();
    
    // Get existing collection
    let mut collection = COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id)
            .and_then(|bytes| bytes_to_collection(&bytes))
            .ok_or("Collection not found".to_string())
    })?;
    
    // Check ownership
    if collection.owner != caller {
        return Err("Only collection owner can update collection".to_string());
    }
    
    // Update fields if provided
    if let Some(new_name) = name {
        if new_name.is_empty() || new_name.len() > 200 {
            return Err("Collection name must be between 1 and 200 characters".to_string());
        }
        collection.name = new_name;
    }
    
    if let Some(new_description) = description {
        collection.description = Some(new_description);
    }
    
    if let Some(new_image_url) = image_url {
        collection.image_url = Some(new_image_url);
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
    COLLECTIONS.with(|storage| {
        storage.borrow_mut().insert(collection_id, collection_to_bytes(&collection));
    });
    
    Ok(())
}

/// Delete a collection (only if it has no documents)
#[update]
pub fn delete_collection(collection_id: String) -> Result<(), String> {
    let caller = caller();
    
    // Get existing collection
    let collection = COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id)
            .and_then(|bytes| bytes_to_collection(&bytes))
            .ok_or("Collection not found".to_string())
    })?;
    
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

/// Get collection metadata by ID
#[query]
pub fn get_collection(collection_id: String) -> Option<CollectionMetadata> {
    COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id)
            .and_then(|bytes| bytes_to_collection(&bytes))
    })
}

/// List all collections
#[query]
pub fn list_all_collections() -> Vec<CollectionMetadata> {
    COLLECTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_collection(&bytes))
            .collect()
    })
}



/// Add a document to a collection
#[update]
pub fn add_document_to_collection(collection_id: String, document_id: String) -> Result<(), String> {
    let caller = caller();
    
    // Check if collection exists
    let mut collection = COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id)
            .and_then(|bytes| bytes_to_collection(&bytes))
            .ok_or("Collection not found".to_string())
    })?;
    
    // Check ownership
    if collection.owner != caller {
        return Err("Only collection owner can modify collection".to_string());
    }
    
    // Check if document exists
    let document = DOCUMENTS.with(|storage| {
        storage.borrow().get(&document_id)
            .and_then(|bytes| bytes_to_document(&bytes))
            .ok_or("Document not found".to_string())
    })?;
    
    // Check if document is already in collection
    if collection.documents.contains(&document_id) {
        return Err("Document is already in this collection".to_string());
    }
    
    // Add document to collection
    collection.documents.push(document_id.clone());
    collection.updated_at = ic_cdk::api::time();
    
    // Update collection
    COLLECTIONS.with(|storage| {
        storage.borrow_mut().insert(collection_id.clone(), collection_to_bytes(&collection));
    });
    
    // Update document's collection_id field
    let mut updated_document = document;
    updated_document.collection_id = collection_id.clone();
    
    DOCUMENTS.with(|storage| {
        storage.borrow_mut().insert(document_id, document_to_bytes(&updated_document));
    });
    
    Ok(())
}

/// Remove a document from a collection
#[update]
pub fn remove_document_from_collection(collection_id: String, document_id: String) -> Result<(), String> {
    let caller = caller();
    
    // Check if collection exists
    let mut collection = COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id)
            .and_then(|bytes| bytes_to_collection(&bytes))
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
        COLLECTIONS.with(|storage| {
            storage.borrow_mut().insert(collection_id.clone(), collection_to_bytes(&collection));
        });
        
        // Update document's collection_id field to empty
        if let Some(document) = DOCUMENTS.with(|storage| {
            storage.borrow().get(&document_id)
                .and_then(|bytes| bytes_to_document(&bytes))
        }) {
            let mut updated_document = document;
            updated_document.collection_id = String::new();
            
            DOCUMENTS.with(|storage| {
                storage.borrow_mut().insert(document_id, document_to_bytes(&updated_document));
            });
        }
        
        Ok(())
    } else {
        Err("Document is not in this collection".to_string())
    }
}


