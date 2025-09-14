// Memory-specific logging functionality
// Handles memory event logging, storage health, and Discord webhooks

use ic_cdk::heartbeat;
use crate::utils::helpers::get_current_timestamp;
use crate::storage;
use super::{Logger, LogSeverity, get_logger, get_severity_for_event_type};
use super::external::{log_memory_wipe_event, get_discord_logger};
use std::cell::RefCell;

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

// Structure for memory wipe logs response
#[derive(candid::CandidType, serde::Serialize, Clone, Debug)]
pub struct MemoryWipeLogs {
    pub current_state: storage::StorageStats,
    pub is_potentially_wiped: bool,
    pub log_instructions: String,
    pub last_check_timestamp: u64,
}

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

// Helper function to get storage counts
pub fn get_storage_counts() -> (u64, u64, u64) {
    let documents_count = storage::DOCUMENTS.with(|docs| docs.borrow().len());
    let institutions_count = storage::INSTITUTIONS.with(|insts| insts.borrow().len());
    let owner_tokens_count = storage::OWNER_TOKENS.with(|tokens| tokens.borrow().len());
    (documents_count, institutions_count, owner_tokens_count)
}

// Helper function to get storage counts including user profiles
pub fn get_all_storage_counts() -> (u64, u64, u64, u64) {
    let documents_count = storage::DOCUMENTS.with(|docs| docs.borrow().len());
    let institutions_count = storage::INSTITUTIONS.with(|insts| insts.borrow().len());
    let owner_tokens_count = storage::OWNER_TOKENS.with(|tokens| tokens.borrow().len());
    let user_profiles_count = storage::USER_PROFILES.with(|profiles| profiles.borrow().len());
    (documents_count, institutions_count, owner_tokens_count, user_profiles_count)
}

// Helper function to format storage stats
pub fn format_storage_stats(prefix: &str, documents: u64, institutions: u64, owner_mappings: u64) -> String {
    format!("{}: {} documents, {} institutions, {} owner mappings", 
            prefix, documents, institutions, owner_mappings)
}

// Helper function to check if storage is empty
pub fn is_storage_empty() -> bool {
    let (documents_count, institutions_count, owner_tokens_count) = get_storage_counts();
    documents_count == 0 && institutions_count == 0 && owner_tokens_count == 0
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

// Query function to get memory wipe logs
#[ic_cdk::query]
pub fn get_memory_wipe_logs() -> MemoryWipeLogs {
    // This function returns information about potential memory wipes
    
    let current_stats = storage::get_storage_stats();
    let anomalies = storage::detect_memory_anomalies();
    let is_empty = is_storage_empty();
    
    let log_instructions = if !anomalies.is_empty() {
        format!("ANOMALIES DETECTED: {}. Check IC system logs for LOG and LOG_DATA entries.", anomalies.join("; "))
    } else {
        "Check IC system logs for LOG entries. Look for POTENTIAL_MEMORY_WIPE events.".to_string()
    };
    
    MemoryWipeLogs {
        current_state: current_stats,
        is_potentially_wiped: is_empty || !anomalies.is_empty(),
        log_instructions,
        last_check_timestamp: get_current_timestamp(),
    }
}


// Core memory wipe detection logic - reusable function
fn perform_memory_wipe_check(check_type: &str, use_discord_only: bool) -> (String, bool) {
    let stats = storage::get_storage_stats();
    let total_items = stats.document_count + stats.institution_count + stats.owner_mapping_count;
    
    let stats_message = format!("Memory stats: documents={}, institutions={}, owner_mappings={}, total={}", 
                     stats.document_count, stats.institution_count, stats.owner_mapping_count, total_items);
    
    let is_wiped = total_items == 0;
    
    if use_discord_only {
        // Send directly to Discord only
        let discord_logger = get_discord_logger();
        if is_wiped {
            let message = format!("🚨 {}: MEMORY WIPE DETECTED - All storage is empty!", check_type);
            let _ = log_memory_wipe_event(&format!("{}_MEMORY_WIPE_DETECTED", check_type), &message, Some(format!("Stats: {:?}", stats)), &discord_logger);
            (message, true)
        } else {
            let message = format!("✅ {}: Memory appears intact. Total items: {}", check_type, total_items);
            let _ = log_memory_wipe_event(&format!("{}_MEMORY_OK", check_type), &message, Some(format!("Stats: {:?}", stats)), &discord_logger);
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

// Query function to get recent memory events (for debugging)
#[ic_cdk::query]
pub fn get_recent_memory_events() -> Vec<String> {
    // This function returns a summary of recent memory events
    // Note: This only shows current state, not historical logs
    let (documents_count, institutions_count, owner_mapping_count) = get_storage_counts();
    let anomalies = storage::detect_memory_anomalies();
    
    let mut events = Vec::new();
    
    // Add current state
    events.push(format!("Current time: {}", get_current_timestamp()));
    events.push(format!("Document count: {}", documents_count));
    events.push(format!("Institution count: {}", institutions_count));
    events.push(format!("Owner mapping count: {}", owner_mapping_count));
    
    // Add any anomalies
    for anomaly in anomalies {
        events.push(format!("ANOMALY: {}", anomaly));
    }
    
    // Add instructions for accessing full logs
    events.push("To see full logs, check IC Dashboard or use 'dfx canister logs'".to_string());
    
    events
}

// Storage for tracking last memory check timestamp
thread_local! {
    static LAST_MEMORY_CHECK: RefCell<u64> = RefCell::new(0);
}

// Heartbeat function that runs every hour to check for memory wipes
#[heartbeat]
fn heartbeat() {
    const ONE_HOUR_NANOSECONDS: u64 = 3_600_000_000_000; // 1 hour in nanoseconds
    
    let current_time = get_current_timestamp();
    let last_check = LAST_MEMORY_CHECK.with(|last| *last.borrow());
    
    // Check if an hour has passed since last check
    if current_time - last_check >= ONE_HOUR_NANOSECONDS {
        // Update the last check timestamp
        LAST_MEMORY_CHECK.with(|last| *last.borrow_mut() = current_time);
        
        // Run the memory wipe check using Discord only
        let (_message, _is_wiped) = perform_memory_wipe_check("HEARTBEAT", true);
    }
}