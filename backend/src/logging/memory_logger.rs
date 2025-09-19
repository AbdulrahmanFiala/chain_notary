// Memory-specific logging functionality
// Handles memory event logging, storage health, and Discord webhooks

use crate::utils::helpers::get_current_timestamp;
use crate::storage;
use super::{Logger, get_logger, get_severity_for_event_type};
use super::external::{log_memory_wipe_event, get_discord_logger};
use std::cell::RefCell;
use std::time::Duration;


// Memory-specific logger
pub struct MemoryLogger {
    logger: Logger,
}

impl MemoryLogger {
    pub fn new() -> Self {
        Self {
            logger: get_logger("memory_monitoring"),
        }
    }

    // Log memory events to IC system logs
    pub fn log_memory_event(&self, event_type: &str, message: &str, detailed_data: Option<String>) {
        let severity = get_severity_for_event_type(event_type);
        self.logger.log(severity, event_type, message, detailed_data);
    }

    // Helper function to log memory event with Discord webhook
    pub fn log_memory_event_with_webhook(&self, event_type: &str, message: &str, detailed_data: Option<String>) {
        // Log to IC system logs
        self.log_memory_event(event_type, message, detailed_data.clone());
        
        // Send Discord webhook
        let discord_logger = get_discord_logger();
        let _ = log_memory_wipe_event(event_type, message, detailed_data, &discord_logger);
    }

}

// Convenience function to get a memory logger instance
pub fn get_memory_logger() -> MemoryLogger {
    MemoryLogger::new()
}

// Helper function to format storage stats
pub fn format_storage_stats(prefix: &str, documents: u64, institutions: u64, user_profiles: u64) -> String {
    format!("{}: {} documents, {} institutions, {} user profiles", 
            prefix, documents, institutions, user_profiles)
}

// Helper function to check if storage is empty
pub fn is_storage_empty() -> bool {
    let stats = storage::get_storage_stats();
    let documents_count = stats.document_count;
    let institutions_count = stats.institution_count;
    let user_profiles_count = stats.user_profile_count;
    documents_count == 0 && institutions_count == 0 && user_profiles_count == 0
}


// Core memory wipe detection logic - reusable function
pub fn perform_memory_wipe_check(check_type: &str, use_discord_only: bool) -> (String, bool) {
    let stats = storage::get_storage_stats();
    let total_items = stats.document_count + stats.institution_count + stats.user_profile_count;
    
    let stats_message = format!("Memory stats: documents={}, institutions={}, user_profiles={}, total={}", 
                     stats.document_count, stats.institution_count, stats.user_profile_count, total_items);
    
    let is_wiped = total_items == 0;
    
    if use_discord_only {
        // Send directly to Discord only
        let discord_logger = get_discord_logger();
        if is_wiped {
            let message = format!("🚨 {}: MEMORY WIPE DETECTED - All storage is empty!", check_type);
            let _ = log_memory_wipe_event(&format!("{} - WIPE DETECTED", check_type), &message, Some(format!("Stats: {:?}", stats)), &discord_logger);
            (message, true)
        } else {
            let message = format!("✅ {}: Memory appears intact. Total items: {}", check_type, total_items);
            let _ = log_memory_wipe_event(&format!("{} - OK", check_type), &message, Some(format!("Stats: {:?}", stats)), &discord_logger);
            (message, false)
        }
    } else {
        // Use memory logger (logs to IC + Discord)
        let memory_logger = get_memory_logger();
        memory_logger.log_memory_event(&format!("{}_MEMORY_WIPE_CHECK", check_type), &format!("{} function called", check_type), None);
        memory_logger.log_memory_event(&format!("{}_MEMORY_WIPE_CHECK", check_type), &stats_message, None);
        
        if is_wiped {
            let message = format!("MEMORY WIPE DETECTED: All storage is empty!");
            memory_logger.log_memory_event_with_webhook(&format!("{}_MEMORY_WIPE_CHECK", check_type), &message, Some(format!("Stats: {:?}", stats)));
            (message, true)
        } else {
            let message = format!("Memory appears intact. Total items: {}", total_items);
            memory_logger.log_memory_event_with_webhook(&format!("{}_MEMORY_CHECK", check_type), &message, Some(format!("Stats: {:?}", stats)));
            (message, false)
        }
    }
}

// Function to start the 24-hour memory check timer
pub fn start_memory_check_timer() {
    // For production: 24 hours interval
    const TWENTY_FOUR_HOURS_SECONDS: u64 = 24 * 60 * 60; // 24 hours in seconds
    
    ic_cdk_timers::set_timer_interval(
        Duration::from_secs(TWENTY_FOUR_HOURS_SECONDS),
        || {
            ic_cdk::spawn(async {
                // Run the memory wipe check using Discord only
                let (_message, _is_wiped) = perform_memory_wipe_check("Periodic Memory Check", true);
            });
        }
    );
}

// Function to manually trigger memory wipe detection
#[ic_cdk::update]
pub fn check_for_memory_wipe() -> Result<String, String> {
    let (message, _is_wiped) = perform_memory_wipe_check("MANUAL", false);
    Ok(message)
}

// Function to send Discord webhook (separate from memory check to avoid consensus issues)
#[ic_cdk::update]
pub async fn send_discord_webhook(event_type: String, message: String) -> Result<String, String> {
    let logger = get_discord_logger();
    
    match log_memory_wipe_event(&event_type, &message, None, &logger) {
        Ok(_) => Ok("Discord webhook sent successfully!".to_string()),
        Err(e) => Err(format!("Discord webhook failed: {}", e)),
    }
}

// Query function to get storage information in a human-readable format
#[ic_cdk::query]
pub fn get_storage_info() -> Vec<String> {
    // This function returns storage information in a readable format
    let stats = storage::get_storage_stats();
    let documents_count = stats.document_count;
    let institutions_count = stats.institution_count;
    let user_profiles_count = stats.user_profile_count;
    
    let mut info = Vec::new();
    
    // Add current state
    info.push(format!("Current time: {}", get_current_timestamp()));
    info.push(format!("Document count: {}", documents_count));
    info.push(format!("Institution count: {}", institutions_count));
    info.push(format!("User profiles count: {}", user_profiles_count));
    
    // Add instructions for accessing full logs
    info.push("To see full logs, check IC Dashboard or use 'dfx canister logs'".to_string());
    
    info
}
