use sha2::{Digest, Sha256};
use candid::Principal;

/// Get current timestamp from IC
pub fn get_current_timestamp() -> u64 {
    ic_cdk::api::time()
}

/// Calculate SHA256 hash of file data
pub fn calculate_file_hash(file_data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(file_data);
    hex::encode(hasher.finalize())
}

/// Generate unique token ID using timestamp
pub fn generate_document_id() -> String {
    // Use timestamp and a simple counter for unique IDs
    let timestamp = get_current_timestamp();
    format!("document_{}", timestamp)
}

/// Generate unique institution ID using timestamp
pub fn generate_institution_id() -> String {
    // Use timestamp for unique institution IDs
    let timestamp = get_current_timestamp();
    format!("INST_{}", timestamp)
}


/// Validate string length with min and max bounds (after trimming whitespace)
pub fn validate_string_length(value: &str, min: usize, max: usize, field_name: &str) -> Result<(), String> {
    let trimmed_value = value.trim();
    if trimmed_value.len() < min || trimmed_value.len() > max {
        return Err(format!("{} must be between {} and {} characters (after trimming whitespace)", field_name, min, max));
    }
    Ok(())
}

/// Validate email format (basic validation, after trimming whitespace)
pub fn validate_email(email: &str) -> Result<(), String> {
    let trimmed_email = email.trim();
    if trimmed_email.len() < 5 || trimmed_email.len() > 100 {
        return Err("Email must be between 5 and 100 characters (after trimming whitespace)".to_string());
    }
    if !trimmed_email.contains('@') || !trimmed_email.contains('.') {
        return Err("Invalid email format".to_string());
    }
    Ok(())
}

/// Validate file type against allowed types
pub fn validate_file_type(file_type: &str, allowed_types: &[&str]) -> Result<(), String> {
    let trimmed_file_type = file_type.trim();
    if !allowed_types.contains(&trimmed_file_type) {
        return Err(format!("Unsupported file type. Allowed types: {}", allowed_types.join(", ")));
    }
    Ok(())
}

/// Validate file size against maximum limit
pub fn validate_file_size(file_size: usize, max_size_mb: usize) -> Result<(), String> {
    let max_size_bytes = max_size_mb * 1024 * 1024;
    if file_size > max_size_bytes {
        return Err(format!("File size exceeds {}MB limit", max_size_mb));
    }
    Ok(())
}

/// Require that the caller is authenticated (not anonymous)
/// Returns the caller's Principal if authenticated, or an error if anonymous
pub fn require_authenticated_user() -> Result<Principal, String> {
    let caller = ic_cdk::api::msg_caller();
    if caller == Principal::anonymous() {
        return Err("Anonymous users cannot perform this action. Please log in with Internet Identity first.".to_string());
    }
    Ok(caller)
}

/// Get current canister cycles balance
pub fn get_canister_cycles_balance() -> u128 {
    ic_cdk::api::canister_balance() as u128
}

/// Get cycles status based on balance
pub fn get_cycles_status(cycles: u128) -> &'static str {
    if cycles < 1_000_000_000 {
        "CRITICAL"
    } else if cycles < 10_000_000_000 {
        "LOW"
    } else {
        "NORMAL"
    }
}

/// Format cycles balance for human-readable display with status indicator
pub fn format_cycles_balance_with_status(cycles: u128) -> String {
    let status = get_cycles_status(cycles);
    let formatted = format_cycles_balance(cycles);
    format!("{} ({})", formatted, status)
}

/// Format cycles balance for human-readable display
pub fn format_cycles_balance(cycles: u128) -> String {
    if cycles >= 1_000_000_000_000 {
        format!("{:.2}T cycles", cycles as f64 / 1_000_000_000_000.0)
    } else if cycles >= 1_000_000_000 {
        format!("{:.2}B cycles", cycles as f64 / 1_000_000_000.0)
    } else if cycles >= 1_000_000 {
        format!("{:.2}M cycles", cycles as f64 / 1_000_000.0)
    } else if cycles >= 1_000 {
        format!("{:.2}K cycles", cycles as f64 / 1_000.0)
    } else {
        format!("{} cycles", cycles)
    }
}

/// Format timestamp (nanoseconds) to human-readable date and time
/// Returns format like "Sunday, 21 September 2025 3:08 AM"
pub fn format_timestamp_to_human_readable(timestamp_nanos: u64) -> String {
    // Convert nanoseconds to seconds
    let timestamp_seconds = timestamp_nanos / 1_000_000_000;
    
    // Unix epoch: January 1, 1970, 00:00:00 UTC
    const UNIX_EPOCH: u64 = 0;
    const SECONDS_PER_DAY: u64 = 86400;
    const SECONDS_PER_HOUR: u64 = 3600;
    const SECONDS_PER_MINUTE: u64 = 60;
    
    // Days since epoch
    let days_since_epoch = timestamp_seconds / SECONDS_PER_DAY;
    
    // Calculate year (approximate, accounting for leap years)
    let mut year = 1970;
    let mut remaining_days = days_since_epoch;
    
    while remaining_days > 365 {
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_year = if is_leap { 366 } else { 365 };
        if remaining_days >= days_in_year {
            remaining_days -= days_in_year;
            year += 1;
        } else {
            break;
        }
    }
    
    // Calculate month and day
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let days_in_months = if is_leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    
    let mut month = 1;
    let mut day = remaining_days + 1;
    
    for &days_in_month in &days_in_months {
        if day > days_in_month {
            day -= days_in_month;
            month += 1;
        } else {
            break;
        }
    }
    
    // Calculate time components
    let seconds_in_day = timestamp_seconds % SECONDS_PER_DAY;
    let hour = seconds_in_day / SECONDS_PER_HOUR;
    let minute = (seconds_in_day % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    let second = seconds_in_day % SECONDS_PER_MINUTE;
    
    // Calculate day of week (Zeller's congruence)
    let mut y = year;
    let mut m = month;
    if m < 3 {
        m += 12;
        y -= 1;
    }
    let k = y % 100;
    let j = y / 100;
    let day_of_week = (day + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
    
    // Day names (0 = Saturday, 1 = Sunday, etc.)
    let day_names = ["Saturday", "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let day_name = day_names[day_of_week as usize];
    
    // Month names
    let month_names = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    let month_name = month_names[(month - 1) as usize];
    
    // Format AM/PM
    let (display_hour, period) = if hour == 0 {
        (12, "AM")
    } else if hour < 12 {
        (hour as u32, "AM")
    } else if hour == 12 {
        (12, "PM")
    } else {
        ((hour - 12) as u32, "PM")
    };
    
    format!("{}, {} {} {} {}:{:02} {}", 
            day_name, day, month_name, year, display_hour, minute, period)
} 