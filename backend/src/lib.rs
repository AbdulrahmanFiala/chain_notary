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
    storage::memory::restore_stable_data()
        .expect("Failed to restore stable data after upgrade");
}

// Export Candid interface
export_candid!(); 