use ic_cdk::{update, query, api::msg_caller};
use candid::Principal;
use crate::types::{UserProfile, UserRole};
use crate::utils::helpers::{require_authenticated_user, get_current_timestamp};
use crate::logging::{get_logger, get_severity_for_event_type};

/// Check if user has a profile and what their role is
#[query]
pub fn get_user_profile() -> Result<Option<UserProfile>, String> {
    let caller = require_authenticated_user()?;
    let profile = crate::storage::get_user_profile_safe(&caller);
    
    // Populate institution name if ID exists but name is empty
    if let Some(mut user) = profile {
        if !user.assigned_institution_id.is_empty() && user.assigned_institution_name.is_empty() {
            if let Some(institution) = crate::storage::get_institution_safe(&user.assigned_institution_id) {
                user.assigned_institution_name = institution.name;
            }
        }
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

/// Register a new user (called after Internet Identity login for first-time users)
#[update]
pub fn register_user(name: String, email: String) -> Result<UserProfile, String> {
    let caller = require_authenticated_user()?;
    
    // Validate inputs
    crate::utils::validate_string_length(&name, 2, 100, "Name")?;
    crate::utils::validate_email(&email)?;
    
    // Check if profile already exists
    if crate::storage::get_user_profile_safe(&caller).is_some() {
        return Err("User already registered. Use login_user to track login.".to_string());
    }
    
    // Check for duplicate email
    if crate::storage::email_exists(&email) {
        return Err("Email already registered. Please use a different email address.".to_string());
    }
    
    // Create new regular user profile
    let new_profile = UserProfile {
        internet_identity: caller,
        name,
        email,
        role: UserRole::RegularUser,
        assigned_institution_id: String::new(),
        assigned_institution_name: String::new(),
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

/// Login existing user (updates last login timestamp)
#[update]
pub fn login_user() -> Result<UserProfile, String> {
    let caller = require_authenticated_user()?;
    
    // Get existing profile
    let mut profile = crate::storage::get_user_profile_safe(&caller)
        .ok_or_else(|| "User not found. Please register first.".to_string())?;
    
    // Update last_login timestamp
    profile.last_login = get_current_timestamp();
    crate::storage::update_user_profile_safe(&caller, &profile)?;
    
    Ok(profile)
}

/// Update user name and email (users can update their own profile, super admins can update any profile)
#[update]
pub fn update_user_profile(user_identity: Option<Principal>, name: String, email: String) -> Result<UserProfile, String> {
    let caller = require_authenticated_user()?;
    
    // If user_identity is not provided, default to caller
    let target_user = user_identity.unwrap_or(caller);
    
    // Check if caller is the user being updated OR is a super admin
    let is_super_admin = if let Some(caller_profile) = crate::storage::get_user_profile_safe(&caller) {
        caller_profile.role == UserRole::SuperAdmin
    } else {
        false
    };
    
    if caller != target_user && !is_super_admin {
        return Err("Access denied. You can only update your own profile.".to_string());
    }
    
    // Validate inputs
    crate::utils::validate_string_length(&name, 2, 100, "Name")?;
    crate::utils::validate_email(&email)?;
    
    // Check for duplicate email (excluding the user being updated)
    if crate::storage::email_exists_excluding_user(&email, &target_user) {
        return Err("Email already registered. Please use a different email address.".to_string());
    }
    
    // Get existing profile
    let mut profile = crate::storage::get_user_profile_safe(&target_user)
        .ok_or_else(|| "User not found.".to_string())?;
    
    // Update only name and email, preserve other fields
    profile.name = name;
    profile.email = email;
    
    // Save updated profile
    crate::storage::update_user_profile_safe(&target_user, &profile)?;
    
    // Log the update
    let logger = get_logger("user_management");
    let severity = get_severity_for_event_type("USER_PROFILE_UPDATE");
    logger.log(severity, "USER_PROFILE_UPDATE", &format!("User profile updated: {} by {}", target_user, caller), Some(caller.to_string()));
    
    Ok(profile)
}

#[query]
pub fn whoami() -> Principal {
    msg_caller()
}