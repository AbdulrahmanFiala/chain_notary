// External logging service for memory wipe tracking
// This sends logs to external services that persist outside the IC

use serde_json::json;
use ic_cdk::api::canister_self;
use ic_cdk::management_canister::{HttpRequestArgs, HttpMethod, HttpHeader, http_request};
use ic_cdk::{println, futures};
use crate::utils::helpers::get_current_timestamp;

// Constants
const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;
const MAX_DISCORD_RESPONSE_BYTES: u64 = 2000;
const DISCORD_REQUEST_CYCLES: u128 = 5_000_000_000;
const DISCORD_WEBHOOK_URL: &str = "https://discordapp.com/api/webhooks/1415683134997139466/F894OVHPtYCQiOVKI7HGu_7uHQtPhViVHVadt7oKgYPpHISDkeI9137AhQW-yehjSUeA";

// External logging service for memory wipe tracking
// This sends logs to external services that persist outside the IC

pub struct ExternalLogger {
    webhook_url: String,
}

impl ExternalLogger {
    pub fn new() -> Self {
        Self {
            webhook_url: DISCORD_WEBHOOK_URL.to_string(),
        }
    }
}

// Convenience function to get a logger instance
pub fn get_discord_logger() -> ExternalLogger {
    ExternalLogger::new()
}

// Log memory wipe events to external services
pub fn log_memory_wipe_event(
    event_type: &str,
    message: &str,
    detailed_data: Option<String>,
    logger: &ExternalLogger,
) -> Result<(), String> {
    
    // Get timestamp once and use it for both logging and Discord
    let timestamp_nanos = get_current_timestamp();
    
    let log_data = json!({
        "timestamp": timestamp_nanos,
        "event_type": event_type,
        "canister_id": canister_self().to_string(),
        "message": message,
        "detailed_data": detailed_data,
        "source": "chain_notary_canister",
        "severity": get_severity_level(event_type),
    });
    
    // Always log to IC system logs as backup
    println!("EXTERNAL_LOG: {}", log_data.to_string());
    
    // Send webhook asynchronously
    let event_type_owned = event_type.to_string();
    let message_owned = message.to_string();
    let detailed_data_owned = detailed_data.clone();
    let webhook_url = logger.webhook_url.clone();
    let timestamp_nanos_owned = timestamp_nanos;
    
    futures::spawn(async move {
        match send_webhook(&webhook_url, &event_type_owned, &message_owned, detailed_data_owned, timestamp_nanos_owned).await {
            Ok(_) => println!("Webhook sent successfully"),
            Err(e) => println!("Failed to send webhook: {}", e),
        }
    });
    
    Ok(())
}

// Get severity level based on event type
pub fn get_severity_level(event_type: &str) -> &str {
    match event_type {
        "POTENTIAL_MEMORY_WIPE" | "MEMORY_WIPE_DETECTED" => "CRITICAL",
        "PRE_UPGRADE" | "POST_UPGRADE" => "INFO",
        "CANISTER_INIT" => "INFO",
        "MANUAL_MEMORY_WIPE_CHECK" => "WARNING",
        "MEMORY_ANOMALY" => "WARNING",
        _ => "INFO",
    }
}

// Discord webhook payload structure for memory wipe events
pub fn create_discord_webhook_payload(event_type: &str, message: &str, detailed_data: Option<String>, timestamp_nanos: u64) -> serde_json::Value {
    let color = match get_severity_level(event_type) {
        "CRITICAL" => 0xff0000, // Red
        "WARNING" => 0xffaa00,  // Orange
        "INFO" => 0x00ff00,     // Green
        _ => 0x0099ff,          // Blue
    };
    
    // Convert nanoseconds to seconds for Discord timestamp
    let timestamp_seconds = timestamp_nanos / NANOSECONDS_PER_SECOND;
    
    // Build fields array dynamically
    let mut fields = vec![
        json!({
            "name": "Canister ID",
            "value": canister_self().to_string(),
            "inline": true
        }),
        json!({
            "name": "Timestamp",
            "value": timestamp_seconds.to_string(),
            "inline": true
        }),
        json!({
            "name": "Date & Time",
            "value": format!("<t:{}:F>", timestamp_seconds),
            "inline": true
        }),
        json!({
            "name": "Severity",
            "value": get_severity_level(event_type),
            "inline": true
        })
    ];
    
    // Add detailed data field if provided
    if let Some(detailed) = detailed_data {
        fields.push(json!({
            "name": "Details",
            "value": detailed,
            "inline": false
        }));
    }
    
    json!({
        "embeds": [{
            "title": format!("ChainNotary Memory Event: {}", event_type),
            "description": message,
            "color": color,
            "fields": fields,
            "footer": {
                "text": "ChainNotary Memory Monitor"
            }
        }]
    })
}

// Send webhook to Discord
async fn send_webhook(
    webhook_url: &str,
    event_type: &str,
    message: &str,
    detailed_data: Option<String>,
    timestamp_nanos: u64,
) -> Result<(), String> {
    let discord_payload = create_discord_webhook_payload(event_type, message, detailed_data, timestamp_nanos);
    
    let request = HttpRequestArgs {
        url: webhook_url.to_string(),
        method: HttpMethod::POST,
        headers: vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
        ],
        body: Some(discord_payload.to_string().into_bytes()),
        max_response_bytes: Some(MAX_DISCORD_RESPONSE_BYTES),
        transform: None,
        is_replicated: Some(false),
    };
    
    match http_request(&request).await {
        Ok(response) => {
            if response.status >= 200u32 && response.status < 300u32 {
                Ok(())
            } else {
                let error_body = String::from_utf8(response.body.clone())
                    .unwrap_or_else(|_| "Unable to decode error response".to_string());
                Err(format!("HTTP error: {} - Response: {}", response.status, error_body))
            }
        }
        Err(e) => Err(format!("Request failed: {:?}", e)),
    }
}
