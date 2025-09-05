use ic_cdk::{query, update, caller};
use candid::Principal;
use crate::types::{UserProfile, UserRole};
use crate::storage::USER_PROFILES;

/// Check if current caller is a super admin
fn require_super_admin() -> Result<Principal, String> {
    let caller = caller();
    
    match crate::storage::get_user_profile_safe(&caller) {
        Some(profile) => {
            if profile.role == UserRole::SuperAdmin {
                Ok(caller)
            } else {
                Err("Access denied. Super admin privileges required.".to_string())
            }
        },
        None => Err("User profile not found. Please contact an administrator.".to_string())
    }
}

/// Admin function: Get all users (admin only)
#[query]
pub fn admin_get_all_users() -> Result<Vec<UserProfile>, String> {
    require_super_admin()?;
    
    let users = USER_PROFILES.with(|profiles| {
        profiles.borrow().iter()
            .map(|(_, storable_profile)| storable_profile.0)
            .collect()
    });
    
    Ok(users)
}

/// Admin function: Get users without institutions (admin only)
#[query]
pub fn admin_get_users_without_institutions() -> Result<Vec<UserProfile>, String> {
    require_super_admin()?;
    
    let users = USER_PROFILES.with(|profiles| {
        profiles.borrow().iter()
            .map(|(_, storable_profile)| storable_profile.0)
            .filter(|profile| profile.assigned_institution_id.is_empty())
            .collect()
    });
    
    Ok(users)
}

/// Admin function: Create institution for a specific user
#[update]
pub fn admin_create_institution_for_user(
    user_identity: Principal,
    institution_name: String,
    institution_email: String,
) -> Result<String, String> {
    // Require super admin privileges
    require_super_admin()?;
    
    // Validate inputs
    crate::utils::validate_string_length(&institution_name, 2, 100, "Institution name")?;
    crate::utils::validate_email(&institution_email)?;
    
    // Check if user already has an institution assigned
    if let Some(existing_profile) = crate::storage::get_user_profile_safe(&user_identity) {
        if !existing_profile.assigned_institution_id.is_empty() {
            return Err("User already has an institution assigned".to_string());
        }
    }
    
    // Generate institution ID
    let institution_id = crate::utils::generate_institution_id();
    
    // Create institution with user as owner
    let institution = crate::types::Institution {
        institution_id: institution_id.clone(),
        owner: user_identity, // User becomes owner, not admin
        name: institution_name,
        email: institution_email,
        created_at: ic_cdk::api::time(),
    };
    
    // Store institution
    crate::storage::update_institution_safe(&institution_id, &institution)?;
    
    // Create or update user profile
    let user_profile = UserProfile {
        internet_identity: user_identity,
        role: UserRole::InstitutionMember(institution_id.clone()),
        assigned_institution_id: institution_id.clone(),
        created_at: ic_cdk::api::time(),
        last_login: ic_cdk::api::time(),
    };
    
    crate::storage::update_user_profile_safe(&user_identity, &user_profile)?;
    
    Ok(institution_id)
}

/// Admin function: Promote user to super admin (admin only)
#[update]
pub fn admin_promote_to_super_admin(user_identity: Principal) -> Result<(), String> {
    require_super_admin()?; // Only super admins can promote others
    
    USER_PROFILES.with(|profiles| {
        let profile_key = crate::storage::memory::StorablePrincipal(user_identity);
        match profiles.borrow().get(&profile_key) {
            Some(mut profile) => {
                profile.0.role = UserRole::SuperAdmin;
                profiles.borrow_mut().insert(profile_key, profile);
                Ok(())
            },
            None => {
                // Create new super admin profile
                let new_profile = UserProfile {
                    internet_identity: user_identity,
                    role: UserRole::SuperAdmin,
                    assigned_institution_id: String::new(),
                    created_at: ic_cdk::api::time(),
                    last_login: 0,
                };
                profiles.borrow_mut().insert(profile_key, crate::storage::memory::StorableUserProfile(new_profile));
                Ok(())
            }
        }
    })
}

/// Admin function: Delete a user (super admin only)
#[update]
pub fn admin_delete_user(user_identity: Principal) -> Result<(), String> {
    require_super_admin()?; // Only super admins can delete users
    
    // Prevent deleting super admins
    if let Some(profile) = crate::storage::get_user_profile_safe(&user_identity) {
        if profile.role == UserRole::SuperAdmin {
            return Err("Cannot delete super admin users".to_string());
        }
    }
    
    // Check if user exists
    let profile_key = crate::storage::memory::StorablePrincipal(user_identity);
    let user_exists = USER_PROFILES.with(|profiles| {
        profiles.borrow().contains_key(&profile_key)
    });
    
    if !user_exists {
        return Err("User not found".to_string());
    }
    
    // Delete user profile
    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().remove(&profile_key);
    });
    
    ic_cdk::println!("Admin deleted user: {}", user_identity);
    Ok(())
}

/// Bootstrap function: Create first super admin (only works if no super admins exist)
#[update]
pub fn bootstrap_first_super_admin() -> Result<(), String> {
    let caller = caller();
    
    // Check if any super admins already exist
    let has_super_admin = USER_PROFILES.with(|profiles| {
        profiles.borrow().iter().any(|(_, profile)| 
            profile.0.role == UserRole::SuperAdmin
        )
    });
    
    if has_super_admin {
        return Err("Super admin already exists. Use admin_promote_to_super_admin instead.".to_string());
    }
    
    // Create first super admin profile
    let super_admin_profile = UserProfile {
        internet_identity: caller,
        role: UserRole::SuperAdmin,
        assigned_institution_id: String::new(),
        created_at: ic_cdk::api::time(),
        last_login: ic_cdk::api::time(),
    };
    
    crate::storage::update_user_profile_safe(&caller, &super_admin_profile)?;
    
    ic_cdk::println!("Bootstrap: First super admin created for {}", caller);
    Ok(())
}
