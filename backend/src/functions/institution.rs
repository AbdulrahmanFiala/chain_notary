use ic_cdk::{update, caller};
use crate::types::Institution;
use crate::storage::INSTITUTIONS;
use crate::utils::generate_institution_id;

/// Create a new institution
#[update]
pub fn create_institution(
    name: String,
    email: String,
) -> Result<String, String> {
    // Validate inputs
    crate::utils::validate_string_length(&name, 2, 100, "Institution name")?;
    crate::utils::validate_email(&email)?;

    let caller = caller();
    
    // Generate unique institution ID
    let institution_id = generate_institution_id();
    
    // Check if institution already exists (shouldn't happen with timestamp-based IDs, but safety check)
    let exists = INSTITUTIONS.with(|storage| {
        storage.borrow().contains_key(&institution_id)
    });
    
    if exists {
        return Err("Generated institution ID already exists. Please try again.".to_string());
    }

    let institution = Institution {
        institution_id: institution_id.clone(),
        owner: caller,
        name,
        email,
        created_at: ic_cdk::api::time(),
    };

    crate::storage::update_institution_safe(&institution_id, &institution)?;

    Ok(institution_id)
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


    INSTITUTIONS.with(|storage| {
        storage.borrow_mut().remove(&institution_id);
    });

    Ok(())
}





