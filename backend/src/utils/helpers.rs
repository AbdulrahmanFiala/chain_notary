use sha2::{Digest, Sha256};
use candid::Principal;

/// Calculate SHA256 hash of file data
pub fn calculate_file_hash(file_data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(file_data);
    hex::encode(hasher.finalize())
}

/// Calculate base hash from DocumentBase attributes for integrity verification
pub fn calculate_base_hash(
    institution_id: &str,
    collection_id: &str,
    document_id: &str,
    owner: &Principal,
    name: &str,
    company_name: &str,
    description: &str,
    document_data: &crate::types::DocumentType,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(institution_id.as_bytes());
    hasher.update(collection_id.as_bytes());
    hasher.update(document_id.as_bytes());
    hasher.update(owner.as_slice());
    hasher.update(name.as_bytes());
    hasher.update(company_name.as_bytes());
    hasher.update(description.as_bytes());
    
    // Include document_data and all its sub-data in the hash
    match document_data {
        crate::types::DocumentType::EarningRelease(earning_data) => {
            // Hash earning_release_id
            hasher.update(earning_data.earning_release_id.as_bytes());
            
            // Hash quarter and year
            hasher.update(&earning_data.quarter.to_le_bytes());
            hasher.update(&earning_data.year.to_le_bytes());
            
            // Hash consolidated income data
            hasher.update(&earning_data.consolidated_income_data.gross_profit.to_le_bytes());
            hasher.update(&earning_data.consolidated_income_data.operating_profit.to_le_bytes());
            hasher.update(&earning_data.consolidated_income_data.ebitda.to_le_bytes());
            hasher.update(&earning_data.consolidated_income_data.profit_before_tax.to_le_bytes());
            hasher.update(&earning_data.consolidated_income_data.net_profit.to_le_bytes());
            
            // Hash consolidated balance sheet data
            hasher.update(&earning_data.consolidated_balance_sheet_data.total_assets.to_le_bytes());
            hasher.update(&earning_data.consolidated_balance_sheet_data.total_equity.to_le_bytes());
            hasher.update(&earning_data.consolidated_balance_sheet_data.total_liabilities.to_le_bytes());
            hasher.update(&earning_data.consolidated_balance_sheet_data.total_liabilities_and_equity.to_le_bytes());
        }
    }
    
    hex::encode(hasher.finalize())
}

/// Generate unique token ID using timestamp
pub fn generate_token_id() -> String {
    // Use timestamp and a simple counter for unique IDs
    let timestamp = ic_cdk::api::time();
    format!("document_{}", timestamp)
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