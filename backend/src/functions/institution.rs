use ic_cdk::{update, caller};
use crate::types::{Institution, CollectionMetadata};
use crate::storage::{INSTITUTIONS, COLLECTIONS};

/// Create a new institution
#[update]
pub fn create_institution(
    institution_id: String,
    name: String,
    email: String,
) -> Result<(), String> {
    // Validate inputs
    crate::utils::validate_string_length(&institution_id, 3, 50, "Institution ID")?;
    crate::utils::validate_string_length(&name, 2, 100, "Institution name")?;
    crate::utils::validate_email(&email)?;

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

    crate::storage::update_institution_safe(&institution_id, &institution)?;

    Ok(())
}

/// Update institution metadata (only owner can update)
#[update]
pub fn update_institution(
    institution_id: String,
    name: String,
    email: String,
) -> Result<(), String> {
    let caller = caller();
    
    let mut institution = crate::storage::get_institution_safe(&institution_id)
        .ok_or("Institution not found")?;

    // Check ownership
    if institution.owner != caller {
        return Err("Only the institution owner can update metadata".to_string());
    }

    // Update fields
    crate::utils::validate_string_length(&name, 2, 100, "Institution name")?;
    institution.name = name;
    crate::utils::validate_email(&email)?;
    institution.email = email;

    crate::storage::update_institution_safe(&institution_id, &institution)?;

    Ok(())
}

/// Delete an institution (only if it has no collections)
#[update]
pub fn delete_institution(institution_id: String) -> Result<(), String> {
    let caller = caller();
    
    let institution = crate::storage::get_institution_safe(&institution_id)
        .ok_or("Institution not found")?;

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

/// Add a collection to an institution
#[update]
pub fn add_collection_to_institution(
    institution_id: String,
    collection_id: String,
) -> Result<(), String> {
    let caller = caller();
    
    // Check if institution exists and caller owns it
    let mut institution = crate::storage::get_institution_safe(&institution_id)
        .ok_or("Institution not found")?;

    if institution.owner != caller {
        return Err("Only the institution owner can add collections".to_string());
    }

    // Check if collection exists
    let collection_exists = crate::storage::get_collection_safe(&collection_id).is_some();
    
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
    if let Some(mut collection) = crate::storage::get_collection_safe(&collection_id) {
        collection.institution_id = institution_id.clone();
        if let Err(e) = crate::storage::update_collection_safe(&collection_id, &collection) {
            return Err(format!("Failed to update collection: {}", e));
        }
    }

    // Update institution
    crate::storage::update_institution_safe(&institution_id, &institution)?;

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
    let mut institution = crate::storage::get_institution_safe(&institution_id)
        .ok_or("Institution not found")?;

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
    if let Some(mut collection) = crate::storage::get_collection_safe(&collection_id) {
        collection.institution_id = String::new();
        if let Err(e) = crate::storage::update_collection_safe(&collection_id, &collection) {
            return Err(format!("Failed to update collection: {}", e));
        }
    }

    // Update institution
    crate::storage::update_institution_safe(&institution_id, &institution)?;

    Ok(())
}




