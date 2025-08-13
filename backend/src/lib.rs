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

// Export Candid interface
export_candid!(); 