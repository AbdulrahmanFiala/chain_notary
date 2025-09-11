use ic_cdk::{update, query, api::msg_caller};
use candid::Principal;
use crate::types::{UserProfile, UserRole};
use crate::utils::helpers::require_authenticated_user;

/// Check if user has a profile and what their role is
#[query]
pub fn get_user_profile() -> Option<UserProfile> {
    let caller = msg_caller();
    crate::storage::get_user_profile_safe(&caller)
}

/// Public function for users to register themselves (called after Internet Identity login)
/// Used as well to update the last_login timestamp for existing users
#[update]
pub fn register_user() -> Result<UserProfile, String> {
    let caller = require_authenticated_user()?;
    
    // Check if profile already exists
    if let Some(existing_profile) = crate::storage::get_user_profile_safe(&caller) {
        // Update last_login timestamp for existing users
        let mut updated_profile = existing_profile;
        updated_profile.last_login = ic_cdk::api::time();
        crate::storage::update_user_profile_safe(&caller, &updated_profile)?;
        return Ok(updated_profile);
    }
    
    // Create new regular user profile
    let new_profile = UserProfile {
        internet_identity: caller,
        role: UserRole::RegularUser,
        assigned_institution_id: String::new(), // Empty until admin assigns
        created_at: ic_cdk::api::time(),
        last_login: ic_cdk::api::time(),
    };
    
    // Save profile
    crate::storage::update_user_profile_safe(&caller, &new_profile)?;
    
    ic_cdk::println!("New user registered: {}", caller);
    Ok(new_profile)
}

#[ic_cdk::query]
pub fn whoami() -> Principal {
    msg_caller()
}
