// External dependencies
use candid::Principal;
use ic_cdk::{export_candid, pre_upgrade, post_upgrade, init};
use ic_cdk::api::management_canister::http_request::{TransformArgs, HttpResponse};

// Internal modules
pub mod types;
pub mod storage;
pub mod functions;
pub mod utils;

// Re-export main types and functions
pub use types::*;
pub use functions::*;

// Initialize canister
#[init]
fn init() {
    ic_cdk::println!("ChainNotary canister initialized successfully");
}

// Pre-upgrade hook - called before canister upgrade
#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("Starting pre-upgrade process...");
    
    // Validate all storage before upgrade
    let documents_count = storage::DOCUMENTS.with(|docs| docs.borrow().len());
    let institutions_count = storage::INSTITUTIONS.with(|insts| insts.borrow().len());
    let owner_tokens_count = storage::OWNER_TOKENS.with(|tokens| tokens.borrow().len());
    
    ic_cdk::println!(
        "Pre-upgrade validation: {} documents, {} institutions, {} owner mappings", 
        documents_count, institutions_count, owner_tokens_count
    );
    
    // The ic_stable_structures automatically handle stable storage,
    // but we should validate data integrity
    validate_storage_integrity();
    
    ic_cdk::println!("Pre-upgrade process completed successfully");
}

// Post-upgrade hook - called after canister upgrade
#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("Starting post-upgrade process...");
    
    // Allow some time for stable structures to fully initialize
    ic_cdk::println!("Waiting for stable structures to initialize...");
    
    // Validate that all data was preserved after upgrade
    let documents_count = storage::DOCUMENTS.with(|docs| docs.borrow().len());
    let institutions_count = storage::INSTITUTIONS.with(|insts| insts.borrow().len());
    let owner_tokens_count = storage::OWNER_TOKENS.with(|tokens| tokens.borrow().len());
    
    ic_cdk::println!(
        "Post-upgrade validation: {} documents, {} institutions, {} owner mappings", 
        documents_count, institutions_count, owner_tokens_count
    );
    
    // Validate data integrity after upgrade
    validate_storage_integrity();
    
    // Perform any necessary data migrations
    perform_data_migration();
    
    // Clean up any corrupted entries that may have been created during deserialization
    storage::clear_corrupted_entries();
    
    // Final validation after cleanup
    let final_documents_count = storage::DOCUMENTS.with(|docs| docs.borrow().len());
    let final_institutions_count = storage::INSTITUTIONS.with(|insts| insts.borrow().len());
    let final_owner_tokens_count = storage::OWNER_TOKENS.with(|tokens| tokens.borrow().len());
    
    ic_cdk::println!(
        "Final post-upgrade counts: {} documents, {} institutions, {} owner mappings", 
        final_documents_count, final_institutions_count, final_owner_tokens_count
    );
    
    ic_cdk::println!("Post-upgrade process completed successfully");
}

// Validate storage integrity
fn validate_storage_integrity() {
    match storage::validate_all_storage() {
        Ok(()) => {
            ic_cdk::println!("Storage integrity validation passed");
        }
        Err(e) => {
            ic_cdk::println!("CRITICAL: Storage validation failed: {}", e);
            ic_cdk::trap(&format!("Storage validation failed: {}", e));
        }
    }
}

// Perform data migration if needed
fn perform_data_migration() {
    // This function can be extended to handle schema changes
    // For now, we just verify that all expected fields are present
    
    // Check if we need to update any document schemas
    let migration_needed = storage::DOCUMENTS.with(|docs| {
        for (_, doc) in docs.borrow().iter() {
            // Check if document has all required fields
            if doc.0.file_hash.is_empty() && !doc.0.file_data.is_empty() {
                return true; // Need to compute missing file hashes
            }
        }
        false
    });
    
    if migration_needed {
        ic_cdk::println!("Running data migration for missing file hashes...");
        migrate_document_file_hashes();
    }
    
    ic_cdk::println!("Data migration check completed");
}

// Migrate documents that are missing file hashes
fn migrate_document_file_hashes() {
    let documents_to_update: Vec<(String, crate::types::Document)> = storage::DOCUMENTS.with(|docs| {
        docs.borrow().iter()
            .filter_map(|(key, doc)| {
                if doc.0.file_hash.is_empty() && !doc.0.file_data.is_empty() {
                    Some((key.0.clone(), doc.0.clone()))
                } else {
                    None
                }
            })
            .collect()
    });
    
    for (doc_id, mut document) in documents_to_update {
        // Compute file hash
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&document.file_data);
        document.file_hash = format!("{:x}", hasher.finalize());
        
        // Update the document
        if let Err(e) = storage::store_document_safe(&doc_id, &document) {
            ic_cdk::println!("Failed to migrate document {}: {}", doc_id, e);
        } else {
            ic_cdk::println!("Migrated document {} with computed hash", doc_id);
        }
    }
}

// Query function to check storage health and statistics
#[ic_cdk::query]
pub fn get_storage_health() -> StorageHealthReport {
    let stats = storage::get_storage_stats();
    
    let validation_result = match storage::validate_all_storage() {
        Ok(()) => "Healthy".to_string(),
        Err(e) => format!("Issues found: {}", e),
    };
    
    StorageHealthReport {
        is_healthy: validation_result == "Healthy",
        validation_message: validation_result,
        document_count: stats.document_count,
        institution_count: stats.institution_count,
        owner_mapping_count: stats.owner_mapping_count,
        total_file_size_bytes: stats.total_file_size_bytes,
    }
}

// Structure for storage health report
#[derive(candid::CandidType, serde::Serialize, Clone, Debug)]
pub struct StorageHealthReport {
    pub is_healthy: bool,
    pub validation_message: String,
    pub document_count: u64,
    pub institution_count: u64,
    pub owner_mapping_count: u64,
    pub total_file_size_bytes: u64,
}

// Export Candid interface
export_candid!(); 