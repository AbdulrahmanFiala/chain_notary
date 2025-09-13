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