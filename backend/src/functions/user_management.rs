use ic_cdk::{update, query, api::msg_caller};
use candid::Principal;
use crate::types::{UserProfile, UserRole};
use crate::utils::helpers::{require_authenticated_user, get_current_timestamp};
use crate::logging::{get_logger, LogSeverity, get_severity_for_event_type};

/// Check if user has a profile and what their role is
#[query]
pub fn get_user_profile() -> Result<Option<UserProfile>, String> {
    let caller = require_authenticated_user()?;
    Ok(crate::storage::get_user_profile_safe(&caller))
}

/// Public function for users to register themselves (called after Internet Identity login)
/// Used as well to update the last_login timestamp for existing users
#[update]
pub fn register_user(name: String, email: String) -> Result<UserProfile, String> {
    let caller = require_authenticated_user()?;
    
    // Validate inputs
    crate::utils::validate_string_length(&name, 2, 100, "Name")?;
    crate::utils::validate_email(&email)?;
    
    // Check if profile already exists
    if let Some(existing_profile) = crate::storage::get_user_profile_safe(&caller) {
        // If user exists but has empty name/email, update them
        if existing_profile.name.is_empty() || existing_profile.email.is_empty() {
            let mut updated_profile = existing_profile;
            updated_profile.name = name;
            updated_profile.email = email;
            updated_profile.last_login = get_current_timestamp();
            crate::storage::update_user_profile_safe(&caller, &updated_profile)?;
            return Ok(updated_profile);
        }
        
        // If user already has name/email, just update last_login timestamp
        let mut updated_profile = existing_profile;
        updated_profile.last_login = get_current_timestamp();
        crate::storage::update_user_profile_safe(&caller, &updated_profile)?;
        return Ok(updated_profile);
    }
    
    // Create new regular user profile
    let new_profile = UserProfile {
        internet_identity: caller,
        name,
        email,
        role: UserRole::RegularUser,
        assigned_institution_id: String::new(), // Empty until admin assigns
        created_at: get_current_timestamp(),
        last_login: get_current_timestamp(),
    };
    
    // Save profile
    crate::storage::update_user_profile_safe(&caller, &new_profile)?;
    
    // Log user registration using structured logging
    let logger = get_logger("user_management");
    let severity = get_severity_for_event_type("USER_REGISTRATION");
    logger.log(severity, "USER_REGISTRATION", &format!("New user registered: {}", caller), Some(caller.to_string()));
    
    Ok(new_profile)
}

#[query]
pub fn whoami() -> Principal {
    msg_caller()
}