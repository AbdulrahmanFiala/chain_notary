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

    // Convenience methods for different severity levels
    pub fn critical(&self, event_type: &str, message: &str, detailed_data: Option<String>) {
        self.log(LogSeverity::Critical, event_type, message, detailed_data);
    }

    pub fn warning(&self, event_type: &str, message: &str, detailed_data: Option<String>) {
        self.log(LogSeverity::Warning, event_type, message, detailed_data);
    }

    pub fn info(&self, event_type: &str, message: &str, detailed_data: Option<String>) {
        self.log(LogSeverity::Info, event_type, message, detailed_data);
    }

    pub fn debug(&self, event_type: &str, message: &str, detailed_data: Option<String>) {
        self.log(LogSeverity::Debug, event_type, message, detailed_data);
    }
}

// Convenience function to get a logger for a specific module
pub fn get_logger(module: &str) -> Logger {
    Logger::new(module)
}
