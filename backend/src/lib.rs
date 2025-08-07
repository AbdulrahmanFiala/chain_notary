// External dependencies
use candid::Principal;
use ic_cdk::{query, update, export_candid};

// Internal modules
pub mod types;
pub mod storage;
pub mod functions;
pub mod utils;

// Re-export main types and functions
pub use types::*;
pub use functions::*;

// Greeting functionality for compatibility
use storage::GREETING;

/// Set the greeting prefix
#[update]
pub fn set_greeting(prefix: String) {
    GREETING.with_borrow_mut(|greeting| greeting.set(prefix).unwrap());
}

/// Get a greeting message
#[query]
pub fn greet(name: String) -> String {
    GREETING.with_borrow(|greeting| format!("{}{name}!", greeting.get()))
}

// Export Candid interface
export_candid!(); 