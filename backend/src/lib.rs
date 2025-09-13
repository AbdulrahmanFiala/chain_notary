// ChainNotary Canister - Main Entry Point
// This file serves as the main entry point and module organizer

// External dependencies
use candid::Principal;
use ic_cdk::management_canister::TransformArgs;
use ic_cdk::api::management_canister::http_request::HttpResponse;

// Internal modules
pub mod types;
pub mod storage;
pub mod functions;
pub mod utils;
pub mod lifecycle;
pub mod logging;

// Re-export main types and functions
pub use types::*;
pub use functions::*;
pub use logging::memory_logger::*;

// Export Candid interface
ic_cdk::export_candid!();