// Canister lifecycle management
// Handles initialization, upgrades, and logging

use ic_cdk::{post_upgrade, init, println};
use crate::storage;
use crate::utils::helpers::get_current_timestamp;
use crate::logging::{get_logger, get_severity_for_event_type};
use crate::logging::memory_logger::start_memory_check_timer;

// Helper function for logging lifecycle events
fn log_lifecycle_event(event_type: &str, message: &str, detailed_data: Option<String>) {
    let logger = get_logger("lifecycle");
    let severity = get_severity_for_event_type(event_type);
    logger.log(severity, event_type, message, detailed_data);
}

// Helper function to get current stats
fn get_current_stats() -> (u64, u64, u64) {
    let stats = storage::get_storage_stats();
    (stats.document_count, stats.institution_count, stats.user_profile_count)
}

// Initialize canister
#[init]
fn init() {
    println!("=== ChainNotary Canister Initialization ===");
    
    let timestamp = get_current_timestamp();
    let canister_id = ic_cdk::api::canister_self();
    let (docs, inst, users) = get_current_stats();
    
    let message = format!("Canister initialized: {} documents, {} institutions, {} users", docs, inst, users);
    let detailed_data = Some(format!("Canister ID: {}, Timestamp: {}", canister_id, timestamp));
    
    log_lifecycle_event("CANISTER_INIT", &message, detailed_data);
    
    println!("=== INITIALIZATION COMPLETE ===");
}

// Post-upgrade hook - called after canister upgrade
#[post_upgrade]
fn post_upgrade() {
    println!("=== Post-Upgrade Process ===");
    
    let (docs, inst, users) = get_current_stats();
    let message = format!("Post-upgrade: {} documents, {} institutions, {} users", docs, inst, users);
    
    log_lifecycle_event("POST_UPGRADE", &message, None);
    
    //Start the memory check timer
    start_memory_check_timer();
    println!("Memory monitoring timer started with {} hours interval", 24);
    println!("=== POST-UPGRADE COMPLETE ===");
}

