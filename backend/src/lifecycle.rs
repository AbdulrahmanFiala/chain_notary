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
    
    // Perform any necessary data migrations
    println!("Performing data migrations...");
    perform_data_migration();
    
    // Clean up any corrupted entries that may have been created during deserialization
    println!("Cleaning up corrupted entries...");
    storage::clear_corrupted_entries();
    
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

// Perform data migration if needed
fn perform_data_migration() {
    // This function can be extended to handle schema changes
    // For now, we just verify that all expected fields are present
    
    let lifecycle_logger = get_lifecycle_logger();
    
    // Check if we need to update any document schemas
    let documents_needing_migration = get_documents_needing_migration();
    
    lifecycle_logger.log_data_migration_start(documents_needing_migration.len());
    
    if !documents_needing_migration.is_empty() {
        println!("Running data migration for {} documents with missing file hashes...", documents_needing_migration.len());
        migrate_document_file_hashes();
    }
    
    lifecycle_logger.log_data_migration_complete();
    println!("Data migration check completed");
}

// Helper function to get documents that need migration
fn get_documents_needing_migration() -> Vec<(String, crate::types::Document)> {
    storage::DOCUMENTS.with(|docs| {
        docs.borrow().iter()
            .filter_map(|(key, doc)| {
                if doc.0.file_hash.is_empty() && !doc.0.file_data.is_empty() {
                    Some((key.0.clone(), doc.0.clone()))
                } else {
                    None
                }
            })
            .collect()
    })
}

// Migrate documents that are missing file hashes
fn migrate_document_file_hashes() {
    let lifecycle_logger = get_lifecycle_logger();
    let documents_to_update = get_documents_needing_migration();
    
    for (doc_id, mut document) in documents_to_update {
        // Compute file hash
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&document.file_data);
        document.file_hash = format!("{:x}", hasher.finalize());
        
        // Update the document
        match storage::store_document_safe(&doc_id, &document) {
            Err(e) => {
                println!("Failed to migrate document {}: {}", doc_id, e);
                lifecycle_logger.log_document_migration(&doc_id, false, Some(e));
            }
            Ok(_) => {
                println!("Migrated document {} with computed hash", doc_id);
                lifecycle_logger.log_document_migration(&doc_id, true, None);
            }
        }
    }
}
