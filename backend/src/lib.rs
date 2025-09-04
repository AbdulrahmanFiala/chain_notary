// External dependencies
use candid::Principal;
use ic_cdk::{export_candid, pre_upgrade, post_upgrade};

// Internal modules
pub mod types;
pub mod storage;
pub mod functions;
pub mod utils;

// Re-export main types and functions
pub use types::*;
pub use functions::*;

/// Save all stable data before canister upgrade
#[pre_upgrade]
fn pre_upgrade() {
    storage::memory::save_stable_data()
        .expect("Failed to save stable data during upgrade");
}

/// Restore all stable data after canister upgrade
#[post_upgrade]
fn post_upgrade() {
    // Use unwrap_or_else to provide a fallback in case of upgrade issues
    storage::memory::restore_stable_data()
        .unwrap_or_else(|err| {
            // Log the error but don't panic - allow the canister to start with empty state
            ic_cdk::println!("Warning: Failed to restore stable data after upgrade: {}. Starting with empty state.", err);
        });
}

// Export Candid interface
export_candid!(); 