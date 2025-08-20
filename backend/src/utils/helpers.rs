use sha2::{Digest, Sha256};
use candid::Principal;

/// Calculate SHA256 hash of file data
pub fn calculate_file_hash(file_data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(file_data);
    hex::encode(hasher.finalize())
}

/// Generate unique token ID using timestamp
pub fn generate_token_id() -> String {
    // Use timestamp and a simple counter for unique IDs
    let timestamp = ic_cdk::api::time();
    format!("document_{}", timestamp)
}

/// Validate string length with min and max bounds
pub fn validate_string_length(value: &str, min: usize, max: usize, field_name: &str) -> Result<(), String> {
    if value.len() < min || value.len() > max {
        return Err(format!("{} must be between {} and {} characters", field_name, min, max));
    }
    Ok(())
}

/// Validate email format (basic validation)
pub fn validate_email(email: &str) -> Result<(), String> {
    if email.len() < 5 || email.len() > 100 {
        return Err("Email must be between 5 and 100 characters".to_string());
    }
    if !email.contains('@') || !email.contains('.') {
        return Err("Invalid email format".to_string());
    }
    Ok(())
}

/// Validate file type against allowed types
pub fn validate_file_type(file_type: &str, allowed_types: &[&str]) -> Result<(), String> {
    if !allowed_types.contains(&file_type) {
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