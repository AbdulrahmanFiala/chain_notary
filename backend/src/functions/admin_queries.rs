use ic_cdk::{query, update};
use candid::Principal;
use crate::types::{UserProfile, UserRole, CycleMonitoringData};
use crate::storage::USER_PROFILES;
use crate::utils::helpers::{require_authenticated_user, get_current_timestamp, get_canister_cycles_balance, format_cycles_balance_with_status};

/// Check if current caller is a super admin
pub fn require_super_admin() -> Result<Principal, String> {
    let caller = require_authenticated_user()?;
    
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
        created_at: get_current_timestamp(),
    };
    
    // Store institution
    crate::storage::update_institution_safe(&institution_id, &institution)?;
    
    // Create or update user profile
    let user_profile = UserProfile {
        internet_identity: user_identity,
        name: String::new(), // Will be set when user updates their profile
        email: String::new(), // Will be set when user updates their profile
        role: UserRole::InstitutionMember(institution_id.clone()),
        assigned_institution_id: institution_id.clone(),
        created_at: get_current_timestamp(),
        last_login: get_current_timestamp(),
    };
    
    crate::storage::update_user_profile_safe(&user_identity, &user_profile)?;
    
    Ok(institution_id)
}

/// Admin function: Promote user to super admin (admin only)
#[update]
pub fn admin_promote_to_super_admin(user_identity: Principal) -> Result<(), String> {
    require_super_admin()?; // Only super admins can promote others
    
    // Prevent promoting anonymous users to super admin
    if user_identity == Principal::anonymous() {
        return Err("Cannot promote anonymous users to super admin. Target user must be authenticated.".to_string());
    }
    
    let profile_key = crate::storage::memory::StorablePrincipal(user_identity);
    
    // First, check if user exists and get their current profile
    let existing_profile = USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&profile_key).map(|storable| storable.0.clone())
    });
    
    match existing_profile {
        Some(mut profile) => {
            // Update existing profile
            profile.role = UserRole::SuperAdmin;
            USER_PROFILES.with(|profiles| {
                profiles.borrow_mut().insert(profile_key, crate::storage::memory::StorableUserProfile(profile));
            });
            Ok(())
        },
        None => {
            // Create new super admin profile
            let new_profile = UserProfile {
                internet_identity: user_identity,
                name: String::new(), // Will be set when user updates their profile
                email: String::new(), // Will be set when user updates their profile
                role: UserRole::SuperAdmin,
                assigned_institution_id: String::new(),
                created_at: get_current_timestamp(),
                last_login: 0,
            };
            USER_PROFILES.with(|profiles| {
                profiles.borrow_mut().insert(profile_key, crate::storage::memory::StorableUserProfile(new_profile));
            });
            Ok(())
        }
    }
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

/// Admin function: Link existing user to existing institution (super admin only)
#[update]
pub fn admin_link_user_to_institution(
    user_identity: Principal,
    institution_id: String,
) -> Result<(), String> {
    require_super_admin()?; // Only super admins can link users
    
    // Check if user exists
    let profile_key = crate::storage::memory::StorablePrincipal(user_identity);
    let user_exists = USER_PROFILES.with(|profiles| {
        profiles.borrow().contains_key(&profile_key)
    });
    
    if !user_exists {
        return Err("User not found".to_string());
    }
    
    // Check if institution exists
    let institution_exists = crate::storage::get_institution_safe(&institution_id).is_some();
    if !institution_exists {
        return Err("Institution not found".to_string());
    }
    
    // Get current user profile
    let current_profile = crate::storage::get_user_profile_safe(&user_identity)
        .ok_or("User profile not found")?;
    
    // Check if user is already linked to an institution
    if !current_profile.assigned_institution_id.is_empty() {
        return Err("User is already linked to an institution. Use admin_unlink_user_from_institution first to unlink them.".to_string());
    }
    
    // Update user profile to link to institution
    let updated_profile = UserProfile {
        internet_identity: user_identity,
        name: current_profile.name,
        email: current_profile.email,
        role: UserRole::InstitutionMember(institution_id.clone()),
        assigned_institution_id: institution_id.clone(),
        created_at: current_profile.created_at,
        last_login: current_profile.last_login,
    };
    
    crate::storage::update_user_profile_safe(&user_identity, &updated_profile)?;
    
    ic_cdk::println!("Admin linked user {} to institution {}", user_identity, institution_id);
    Ok(())
}

/// Admin function: Unlink user from their institution (super admin only)
#[update]
pub fn admin_unlink_user_from_institution(user_identity: Principal) -> Result<(), String> {
    require_super_admin()?; // Only super admins can unlink users
    
    // Check if user exists
    let profile_key = crate::storage::memory::StorablePrincipal(user_identity);
    let user_exists = USER_PROFILES.with(|profiles| {
        profiles.borrow().contains_key(&profile_key)
    });
    
    if !user_exists {
        return Err("User not found".to_string());
    }
    
    // Get current user profile
    let current_profile = crate::storage::get_user_profile_safe(&user_identity)
        .ok_or("User profile not found")?;
    
    // Check if user is already not linked to any institution
    if current_profile.assigned_institution_id.is_empty() {
        return Err("User is not linked to any institution".to_string());
    }
    
    // Update user profile to unlink from institution
    let updated_profile = UserProfile {
        internet_identity: user_identity,
        name: current_profile.name,
        email: current_profile.email,
        role: UserRole::RegularUser, // Change role back to RegularUser
        assigned_institution_id: String::new(), // Clear institution assignment
        created_at: current_profile.created_at,
        last_login: current_profile.last_login,
    };
    
    crate::storage::update_user_profile_safe(&user_identity, &updated_profile)?;
    
    ic_cdk::println!("Admin unlinked user {} from institution", user_identity);
    Ok(())
}

/// Bootstrap function: Create first super admin (only works if no super admins exist)
#[update]
pub fn bootstrap_first_super_admin() -> Result<(), String> {
    let caller = require_authenticated_user()?;
    
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
        name: String::new(), // Will be set when user updates their profile
        email: String::new(), // Will be set when user updates their profile
        role: UserRole::SuperAdmin,
        assigned_institution_id: String::new(),
        created_at: get_current_timestamp(),
        last_login: get_current_timestamp(),
    };
    
    crate::storage::update_user_profile_safe(&caller, &super_admin_profile)?;
    
    ic_cdk::println!("Bootstrap: First super admin created for {}", caller);
    Ok(())
}

/// Admin function: Get cycle monitoring information (admin only)
#[query]
pub fn admin_get_cycle_monitoring() -> Result<CycleMonitoringData, String> {
    require_super_admin()?;
    
    let current_balance = get_canister_cycles_balance();
    let formatted_balance = format_cycles_balance_with_status(current_balance);
    let status = crate::utils::helpers::get_cycles_status(current_balance).to_string();
    
    // Get memory size (approximate)
    let memory_size_bytes = ic_cdk::stable::stable_size() * 64 * 1024; // 64 KB per page
    
    
    Ok(CycleMonitoringData {
        current_balance,
        formatted_balance,
        status,
        memory_size_bytes,
        timestamp: get_current_timestamp(),
    })
}



