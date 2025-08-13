use ic_cdk::{update, query, caller};
use crate::types::{Institution, CollectionMetadata, CollectionCategory};
use crate::storage::{INSTITUTIONS, COLLECTIONS, institution_to_bytes, bytes_to_institution, bytes_to_collection, collection_to_bytes};

/// Create a new institution
#[update]
pub fn create_institution(
    institution_id: String,
    name: String,
    email: String,
) -> Result<(), String> {
    // Validate inputs
    if institution_id.len() < 3 || institution_id.len() > 50 {
        return Err("Institution ID must be between 3 and 50 characters".to_string());
    }
    
    if name.len() < 2 || name.len() > 100 {
        return Err("Institution name must be between 2 and 100 characters".to_string());
    }
    
    if email.len() < 5 || email.len() > 100 {
        return Err("Email must be between 5 and 100 characters".to_string());
    }

    let caller = caller();
    
    // Check if institution already exists
    let exists = INSTITUTIONS.with(|storage| {
        storage.borrow().contains_key(&institution_id)
    });
    
    if exists {
        return Err("Institution with this ID already exists".to_string());
    }

    let institution = Institution {
        institution_id: institution_id.clone(),
        owner: caller,
        name,
        email,
        created_at: ic_cdk::api::time(),
        collections: Vec::new(),
    };

    INSTITUTIONS.with(|storage| {
        storage.borrow_mut().insert(institution_id, institution_to_bytes(&institution));
    });

    Ok(())
}

/// Update institution metadata (only owner can update)
#[update]
pub fn update_institution(
    institution_id: String,
    name: Option<String>,
    email: Option<String>,
) -> Result<(), String> {
    let caller = caller();
    
    let mut institution = INSTITUTIONS.with(|storage| {
        storage.borrow().get(&institution_id)
            .and_then(|bytes| bytes_to_institution(&bytes))
            .ok_or("Institution not found")
    })?;

    // Check ownership
    if institution.owner != caller {
        return Err("Only the institution owner can update metadata".to_string());
    }

    // Update fields if provided
    if let Some(new_name) = name {
        if new_name.len() < 2 || new_name.len() > 100 {
            return Err("Institution name must be between 2 and 100 characters".to_string());
        }
        institution.name = new_name;
    }

    if let Some(new_email) = email {
        if new_email.len() < 5 || new_email.len() > 100 {
            return Err("Email must be between 5 and 100 characters".to_string());
        }
        institution.email = new_email;
    }

    INSTITUTIONS.with(|storage| {
        storage.borrow_mut().insert(institution_id, institution_to_bytes(&institution));
    });

    Ok(())
}

/// Delete an institution (only if it has no collections)
#[update]
pub fn delete_institution(institution_id: String) -> Result<(), String> {
    let caller = caller();
    
    let institution = INSTITUTIONS.with(|storage| {
        storage.borrow().get(&institution_id)
            .and_then(|bytes| bytes_to_institution(&bytes))
            .ok_or("Institution not found")
    })?;

    // Check ownership
    if institution.owner != caller {
        return Err("Only the institution owner can delete the institution".to_string());
    }

    // Check if institution has collections
    if !institution.collections.is_empty() {
        return Err("Cannot delete institution with existing collections".to_string());
    }

    INSTITUTIONS.with(|storage| {
        storage.borrow_mut().remove(&institution_id);
    });

    Ok(())
}

/// Get institution metadata
#[query]
pub fn get_institution(institution_id: String) -> Option<Institution> {
    INSTITUTIONS.with(|storage| {
        storage.borrow().get(&institution_id)
            .and_then(|bytes| bytes_to_institution(&bytes))
    })
}

/// List all institutions
#[query]
pub fn list_all_institutions() -> Vec<Institution> {
    INSTITUTIONS.with(|storage| {
        storage.borrow().iter()
            .filter_map(|(_, bytes)| bytes_to_institution(&bytes))
            .collect()
    })
}



/// Add a collection to an institution
#[update]
pub fn add_collection_to_institution(
    institution_id: String,
    collection_id: String,
) -> Result<(), String> {
    let caller = caller();
    
    // Check if institution exists and caller owns it
    let mut institution = match INSTITUTIONS.with(|storage| {
        storage.borrow().get(&institution_id)
            .and_then(|bytes| bytes_to_institution(&bytes))
    }) {
        Some(inst) => inst,
        None => return Err("Institution not found".to_string()),
    };

    if institution.owner != caller {
        return Err("Only the institution owner can add collections".to_string());
    }

    // Check if collection exists
    let collection_exists = COLLECTIONS.with(|storage| {
        storage.borrow().contains_key(&collection_id)
    });
    
    if !collection_exists {
        return Err("Collection does not exist".to_string());
    }

    // Check if collection is already in institution
    if institution.collections.contains(&collection_id) {
        return Err("Collection is already part of this institution".to_string());
    }

    // Add collection to institution
    institution.collections.push(collection_id.clone());

    // Update collection's institution_id
    let collection_data = COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id).map(|bytes| bytes.clone())
    });
    
    if let Some(collection_bytes) = collection_data {
        if let Some(mut collection) = bytes_to_collection(&collection_bytes) {
            collection.institution_id = institution_id.clone();
            let updated_bytes = collection_to_bytes(&collection);
            COLLECTIONS.with(|storage| {
                storage.borrow_mut().insert(collection_id, updated_bytes);
            });
        }
    }

    // Update institution
    INSTITUTIONS.with(|storage| {
        storage.borrow_mut().insert(institution_id, institution_to_bytes(&institution));
    });

    Ok(())
}

/// Remove a collection from an institution
#[update]
pub fn remove_collection_from_institution(
    institution_id: String,
    collection_id: String,
) -> Result<(), String> {
    let caller = caller();
    
    // Check if institution exists and caller owns it
    let mut institution = INSTITUTIONS.with(|storage| {
        storage.borrow().get(&institution_id)
            .and_then(|bytes| bytes_to_institution(&bytes))
            .ok_or("Institution not found")
    })?;

    if institution.owner != caller {
        return Err("Only the institution owner can remove collections".to_string());
    }

    // Check if collection is in institution
    if !institution.collections.contains(&collection_id) {
        return Err("Collection is not part of this institution".to_string());
    }

    // Remove collection from institution
    institution.collections.retain(|id| id != &collection_id);

    // Clear collection's institution_id
    let collection_data = COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id).map(|bytes| bytes.clone())
    });
    
    if let Some(collection_bytes) = collection_data {
        if let Some(mut collection) = bytes_to_collection(&collection_bytes) {
            collection.institution_id = String::new();
            let updated_bytes = collection_to_bytes(&collection);
            COLLECTIONS.with(|storage| {
                storage.borrow_mut().insert(collection_id, updated_bytes);
            });
        }
    }

    // Update institution
    INSTITUTIONS.with(|storage| {
        storage.borrow_mut().insert(institution_id, institution_to_bytes(&institution));
    });

    Ok(())
}




