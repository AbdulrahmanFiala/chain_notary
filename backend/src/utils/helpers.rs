use sha2::{Digest, Sha256};

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
    format!("nft_{}", timestamp)
} 