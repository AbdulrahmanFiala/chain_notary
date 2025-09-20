// Canister lifecycle management
// Handles initialization, upgrades, and logging

use ic_cdk::{post_upgrade, init, pre_upgrade, println};
use crate::storage;
use crate::types::UpgradeCycleConsumption;
use crate::storage::memory::{StorableUpgradeKey, PRE_UPGRADE_CYCLES, StorableUpgradeCycleConsumption};
use crate::utils::helpers::{get_current_timestamp, get_canister_cycles_balance, format_cycles_balance_with_status, format_cycles_balance, require_super_admin};
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

// Pre-upgrade hook - capture cycles before upgrade
#[pre_upgrade]
fn pre_upgrade() {
    println!("=== Pre-Upgrade Process ===");
    
    // Capture cycles before upgrade
    let cycles_before = get_canister_cycles_balance();
    let timestamp = get_current_timestamp();
    
    // Store in stable memory
    let upgrade_key = StorableUpgradeKey(format!("upgrade_{}", timestamp));
    let cycles_data = UpgradeCycleConsumption {
        cycles_before: cycles_before,
        cycles_after: 0, // Will be updated in post_upgrade
        cycles_consumed: 0, // Will be calculated in post_upgrade
        timestamp,
    };
    
    PRE_UPGRADE_CYCLES.with(|storage| {
        storage.borrow_mut().insert(upgrade_key, StorableUpgradeCycleConsumption(cycles_data));
    });
    
    let formatted_cycles = format_cycles_balance_with_status(cycles_before);
    let message = format!("Pre-upgrade cycles: {}", formatted_cycles);
    
    log_lifecycle_event("PRE_UPGRADE_CYCLES", &message, None);
    
    println!("=== PRE-UPGRADE COMPLETE ===");
}

// Post-upgrade hook - called after canister upgrade
#[post_upgrade]
fn post_upgrade() {
    println!("=== Post-Upgrade Process ===");
    
    // Get cycles after upgrade
    let cycles_after = get_canister_cycles_balance();
    let formatted_cycles_after = format_cycles_balance_with_status(cycles_after);
    
    // Get the most recent pre-upgrade data from stable memory
    let mut latest_upgrade_data = None;
    let mut latest_timestamp = 0;
    let mut latest_key = None;
    
    PRE_UPGRADE_CYCLES.with(|storage| {
        for (key, data) in storage.borrow().iter() {
            if key.0.starts_with("upgrade_") && data.0.timestamp > latest_timestamp {
                latest_upgrade_data = Some(data.0.clone());
                latest_timestamp = data.0.timestamp;
                latest_key = Some(key.0.clone());
            }
        }
    });
    
    // Calculate cycle consumption and update the data
    if let (Some(mut upgrade_data), Some(key)) = (latest_upgrade_data, latest_key) {
        upgrade_data.cycles_after = cycles_after;
        upgrade_data.cycles_consumed = upgrade_data.cycles_before.saturating_sub(cycles_after);
        
        // Update the stored data
        let upgrade_key = StorableUpgradeKey(key);
        PRE_UPGRADE_CYCLES.with(|storage| {
            storage.borrow_mut().insert(upgrade_key, StorableUpgradeCycleConsumption(upgrade_data.clone()));
        });
        
        let message = format!("Post-upgrade cycles: {}", formatted_cycles_after);
        let detailed_data = Some(format!(
            "Pre-upgrade: {}, Post-upgrade: {}, Consumption: {}",
            format_cycles_balance_with_status(upgrade_data.cycles_before),
            formatted_cycles_after,
            format_cycles_balance(upgrade_data.cycles_consumed)
        ));
        
        log_lifecycle_event("POST_UPGRADE_CYCLES", &message, detailed_data);
        
    } else {
        println!("No pre-upgrade data found for comparison");
    }
    
    let (docs, inst, users) = get_current_stats();
    let message = format!("Post-upgrade: {} documents, {} institutions, {} users", docs, inst, users);
    
    log_lifecycle_event("POST_UPGRADE", &message, None);
    
    //Start the memory check timer
    start_memory_check_timer();
    println!("Memory monitoring timer started with {} hours interval", 24);
    println!("=== POST-UPGRADE COMPLETE ===");
}

// Query function to get upgrade cycle history (super admin only)
#[ic_cdk::query]
pub fn get_upgrade_cycle_history() -> Result<Vec<(String, UpgradeCycleConsumption)>, String> {
    require_super_admin()?;
    
    let mut history = Vec::new();
    
    PRE_UPGRADE_CYCLES.with(|storage| {
        for (key, data) in storage.borrow().iter() {
            history.push((key.0.clone(), data.0.clone()));
        }
    });
    
    // Sort by timestamp (newest first)
    history.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
    Ok(history)
}

