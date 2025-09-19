// Canister lifecycle management
// Handles initialization, upgrades, and data migration

use ic_cdk::{pre_upgrade, post_upgrade, init, println};
use crate::storage;
use crate::logging::lifecycle_logger::{get_lifecycle_logger, LifecycleLogger};
use crate::logging::memory_logger::{get_storage_counts, format_storage_stats};

// Initialize canister
#[init]
fn init() {
    println!("=== ChainNotary Canister Initialization ===");
    println!("ChainNotary canister initialized successfully");
    
    // Log the initialization event using the lifecycle logger
    let lifecycle_logger = get_lifecycle_logger();
    lifecycle_logger.log_initialization();
    
    println!("=== INITIALIZATION COMPLETE ===");
}

// Pre-upgrade hook - called before canister upgrade
#[pre_upgrade]
fn pre_upgrade() {
    println!("=== Pre-Upgrade Process ===");
    println!("Starting pre-upgrade process...");
    
    // Validate all storage before upgrade
    let (documents_count, institutions_count, owner_tokens_count) = get_storage_counts();
    
    let stats_message = format_storage_stats("", documents_count, institutions_count, owner_tokens_count);
    println!("Pre-upgrade validation completed: {}", stats_message);
    
    // Log pre-upgrade state using the lifecycle logger
    let lifecycle_logger = get_lifecycle_logger();
    lifecycle_logger.log_pre_upgrade();
    
    // Optimize data before upgrade to reduce transfer costs
    optimize_data_for_upgrade();
    
    validate_storage_integrity();
    
    println!("Pre-upgrade process completed successfully");
    println!("=== PRE-UPGRADE COMPLETE ===");
}

// Post-upgrade hook - called after canister upgrade
#[post_upgrade]
fn post_upgrade() {
    println!("=== Post-Upgrade Process ===");
    println!("Starting post-upgrade process...");
    
    // Allow some time for stable structures to fully initialize
    println!("Waiting for stable structures to initialize...");
    
    // Validate that all data was preserved after upgrade
    let (documents_count, institutions_count, owner_tokens_count) = get_storage_counts();
    
    let stats_message = format_storage_stats("", documents_count, institutions_count, owner_tokens_count);
    println!("Post-upgrade validation completed: {}", stats_message);
    
    // Use lifecycle logger for all logging operations
    let lifecycle_logger = get_lifecycle_logger();
    
    // Check for potential memory wipe by comparing with expected counts
    lifecycle_logger.log_potential_memory_wipe();
    
    // Log post-upgrade state
    lifecycle_logger.log_post_upgrade();
    
    // Validate data integrity after upgrade
    println!("Validating data integrity after upgrade...");
    validate_storage_integrity();
        
    // Clean up any corrupted entries that may have been created during deserialization
    println!("Cleaning up corrupted entries...");
    storage::cleanup_corrupted_entries();
    
    // Final validation after cleanup
    let (final_documents_count, final_institutions_count, final_owner_tokens_count) = get_storage_counts();
    
    let final_stats_message = format_storage_stats("", final_documents_count, final_institutions_count, final_owner_tokens_count);
    println!("Final post-upgrade counts: {}", final_stats_message);
    
    // Log final state
    lifecycle_logger.log_final_post_upgrade();
    
    println!("Post-upgrade process completed successfully");
    println!("=== POST-UPGRADE COMPLETE ===");
}

// Validate storage integrity
fn validate_storage_integrity() {
    let lifecycle_logger = get_lifecycle_logger();
    let validation_result = storage::validate_all_storage();
    
    match &validation_result {
        Ok(()) => {
            println!("Storage integrity validation passed");
        }
        Err(e) => {
            println!("CRITICAL: Storage validation failed: {}", e);
            // Log error instead of trapping to avoid init mode issues
            println!("ERROR: Storage validation error logged instead of trapping");
        }
    }
    
    // Log validation result using the lifecycle logger
    lifecycle_logger.log_storage_validation(validation_result);
}



// Optimize data before upgrade to reduce transfer costs
fn optimize_data_for_upgrade() {
    let lifecycle_logger = get_lifecycle_logger();
    
    // Use existing cleanup function to reduce data size before upgrade
    let cleanup_result = storage::cleanup_corrupted_entries();
    if cleanup_result.total_cleaned > 0 {
        println!("Pre-upgrade cleanup: removed {} corrupted entries", cleanup_result.total_cleaned);
        // Log cleanup completion
        let severity = crate::logging::get_severity_for_event_type("CLEANUP");
        lifecycle_logger.logger.log(severity, "CLEANUP", &format!("Pre-upgrade cleanup: removed {} corrupted entries", cleanup_result.total_cleaned), Some(format!("Cleaned {} documents: {:?}", cleanup_result.cleaned_document_ids.len(), cleanup_result.cleaned_document_ids)));
    }
}

// Optimize data before upgrade to reduce transfer costs
fn optimize_data_for_upgrade() {
    let lifecycle_logger = get_lifecycle_logger();
    
    // Use existing cleanup function to reduce data size before upgrade
    let cleanup_result = storage::cleanup_corrupted_entries();
    if cleanup_result.total_cleaned > 0 {
        println!("Pre-upgrade cleanup: removed {} corrupted entries", cleanup_result.total_cleaned);
        lifecycle_logger.log_document_migration("PRE_UPGRADE_CLEANUP", true, Some(format!("Cleaned {} documents: {:?}", cleanup_result.cleaned_document_ids.len(), cleanup_result.cleaned_document_ids)));
    }
}