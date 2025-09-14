// Centralized logging module for ChainNotary
// Provides unified logging interface for all modules

pub mod memory_logger;
pub mod lifecycle_logger;
pub mod external;

// Re-export main logging functions for easy access
pub use memory_logger::*;
pub use lifecycle_logger::*;
pub use external::*;

// Common logging structures and utilities
use ic_cdk::{api::canister_self, println};
use crate::utils::helpers::get_current_timestamp;

// Log severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum LogSeverity {
    Critical,
    Warning,
    Info,
    Debug,
}

impl LogSeverity {
    pub fn as_str(&self) -> &str {
        match self {
            LogSeverity::Critical => "CRITICAL",
            LogSeverity::Warning => "WARNING", 
            LogSeverity::Info => "INFO",
            LogSeverity::Debug => "DEBUG",
        }
    }
}

// Main logging interface
pub struct Logger {
    pub module: String,
}

impl Logger {
    pub fn new(module: &str) -> Self {
        Self {
            module: module.to_string(),
        }
    }

    // Generic log function that all specialized loggers can use
    pub fn log(&self, severity: LogSeverity, event_type: &str, message: &str, detailed_data: Option<String>) {
        let timestamp = get_current_timestamp();
        let canister_id = canister_self();
        
        // Create structured log entry
        let log_entry = format!(
            "[{}] {} - {} - Module: {} - Canister: {} - Message: {}",
            timestamp, severity.as_str(), event_type, self.module, canister_id, message
        );
        
        // Log to IC system logs (these persist across upgrades)
        println!("LOG: {}", log_entry);
        
        // If detailed data is provided, log it separately
        if let Some(ref data) = detailed_data {
            println!("LOG_DATA: {}", data);
        }
    }

}

// Convenience function to get a logger for a specific module
pub fn get_logger(module: &str) -> Logger {
    Logger::new(module)
}

// Shared severity mapping for event types - eliminates duplication between modules
pub fn get_severity_for_event_type(event_type: &str) -> LogSeverity {
    match event_type {
        "POTENTIAL_MEMORY_WIPE" | "MEMORY_WIPE_DETECTED" | "HEARTBEAT_MEMORY_WIPE_DETECTED" => LogSeverity::Critical,
        "PRE_UPGRADE" | "POST_UPGRADE" | "POST_UPGRADE_FINAL" => LogSeverity::Info,
        "CANISTER_INIT" => LogSeverity::Info,
        "MANUAL_MEMORY_WIPE_CHECK" | "MEMORY_ANOMALY" | "STORAGE_ANOMALY" => LogSeverity::Warning,
        "HEARTBEAT_MEMORY_OK" => LogSeverity::Info,
        "STORAGE_VALIDATION" | "DATA_MIGRATION" | "DOCUMENT_MIGRATION" | "CLEANUP" => LogSeverity::Info,
        "SERIALIZATION_ERROR" | "DESERIALIZATION_ERROR" => LogSeverity::Critical,
        "CORRUPTED_DATA" | "VALIDATION_SKIP" => LogSeverity::Debug,
        "VALIDATION_WARNINGS" => LogSeverity::Warning,
        "USER_REGISTRATION" => LogSeverity::Info,
        _ => LogSeverity::Info,
    }
}