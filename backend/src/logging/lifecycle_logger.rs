// Lifecycle-specific logging functionality
// Handles initialization, upgrades, and data migration logging

use crate::utils::helpers::get_current_timestamp;
use crate::storage;
use super::{Logger, LogSeverity, get_logger, get_severity_for_event_type};
use super::memory_logger::{get_storage_counts, format_storage_stats};

// Lifecycle-specific logger
pub struct LifecycleLogger {
    logger: Logger,
}

impl LifecycleLogger {
    pub fn new() -> Self {
        Self {
            logger: get_logger("lifecycle"),
        }
    }

    // Log canister initialization
    pub fn log_initialization(&self) {
        let timestamp = get_current_timestamp();
        let canister_id = ic_cdk::api::canister_self();
        let message = format!("Canister initialized at timestamp {}", timestamp);
        
        let severity = get_severity_for_event_type("CANISTER_INIT");
        self.logger.log(severity, "CANISTER_INIT", &message, Some(format!("Canister ID: {}", canister_id)));
    }

    // Log pre-upgrade state
    pub fn log_pre_upgrade(&self) {
        let (documents_count, institutions_count, owner_tokens_count) = get_storage_counts();
        let stats_message = format_storage_stats("Pre-upgrade", documents_count, institutions_count, owner_tokens_count);
        
        let severity = get_severity_for_event_type("PRE_UPGRADE");
        self.logger.log(severity, "PRE_UPGRADE", &stats_message, Some(stats_message.clone()));
    }

    // Log post-upgrade state
    pub fn log_post_upgrade(&self) {
        let (documents_count, institutions_count, owner_tokens_count) = get_storage_counts();
        let stats_message = format_storage_stats("Post-upgrade", documents_count, institutions_count, owner_tokens_count);
        
        let severity = get_severity_for_event_type("POST_UPGRADE");
        self.logger.log(severity, "POST_UPGRADE", &stats_message, Some(stats_message.clone()));
    }

    // Log potential memory wipe detection
    pub fn log_potential_memory_wipe(&self) {
        let (documents_count, institutions_count, owner_tokens_count) = get_storage_counts();
        let total_items = documents_count + institutions_count + owner_tokens_count;
        
        if total_items == 0 {
            let message = "All storage counts are zero after upgrade";
            let severity = get_severity_for_event_type("POTENTIAL_MEMORY_WIPE");
            self.logger.log(severity, "POTENTIAL_MEMORY_WIPE", message, None);
        }
    }

    // Log final post-upgrade state
    pub fn log_final_post_upgrade(&self) {
        let (documents_count, institutions_count, owner_tokens_count) = get_storage_counts();
        let final_stats = format_storage_stats("Final post-upgrade", documents_count, institutions_count, owner_tokens_count);
        
        let severity = get_severity_for_event_type("POST_UPGRADE_FINAL");
        self.logger.log(severity, "POST_UPGRADE_FINAL", &final_stats, Some(final_stats.clone()));
    }

    // Log storage validation results
    pub fn log_storage_validation(&self, validation_result: Result<(), String>) {
        match validation_result {
            Ok(()) => {
                let severity = get_severity_for_event_type("STORAGE_VALIDATION");
                self.logger.log(severity, "STORAGE_VALIDATION", "Storage integrity validation passed", None);
            }
            Err(e) => {
                let message = format!("Storage validation failed: {}", e);
                let severity = get_severity_for_event_type("STORAGE_VALIDATION");
                self.logger.log(severity, "STORAGE_VALIDATION", &message, Some(e));
            }
        }
    }

    // Log data migration events
    pub fn log_data_migration_start(&self, documents_needing_migration: usize) {
        if documents_needing_migration > 0 {
            let message = format!("Starting data migration for {} documents with missing file hashes", documents_needing_migration);
            let severity = get_severity_for_event_type("DATA_MIGRATION");
            self.logger.log(severity, "DATA_MIGRATION", &message, Some(format!("Document count: {}", documents_needing_migration)));
        }
    }

    // Log data migration completion
    pub fn log_data_migration_complete(&self) {
        let severity = get_severity_for_event_type("DATA_MIGRATION");
        self.logger.log(severity, "DATA_MIGRATION", "Data migration check completed", None);
    }

    // Log individual document migration
    pub fn log_document_migration(&self, doc_id: &str, success: bool, error: Option<String>) {
        if success {
            let severity = get_severity_for_event_type("DOCUMENT_MIGRATION");
            self.logger.log(severity, "DOCUMENT_MIGRATION", &format!("Migrated document {} with computed hash", doc_id), None);
        } else {
            let message = format!("Failed to migrate document {}", doc_id);
            let severity = get_severity_for_event_type("DOCUMENT_MIGRATION");
            self.logger.log(severity, "DOCUMENT_MIGRATION", &message, error);
        }
    }

    // Log cleanup operations
    pub fn log_cleanup_start(&self) {
        let severity = get_severity_for_event_type("CLEANUP");
        self.logger.log(severity, "CLEANUP", "Starting cleanup of corrupted entries", None);
    }

    // Log cleanup completion
    pub fn log_cleanup_complete(&self) {
        let severity = get_severity_for_event_type("CLEANUP");
        self.logger.log(severity, "CLEANUP", "Cleanup of corrupted entries completed", None);
    }

}

// Convenience function to get a lifecycle logger instance
pub fn get_lifecycle_logger() -> LifecycleLogger {
    LifecycleLogger::new()
}
