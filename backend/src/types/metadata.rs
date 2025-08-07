use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::collections::HashMap;

// Custom types for file upload and chain notary
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Institution {
    pub institution_id: String, 
    pub owner: Principal,
    pub name: String,
    pub email: String,
    pub verified: bool, // What's the purpose of this field?
    pub created_at: u64,
    pub collections: Vec<String>, // List of collection IDs
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct CollectionMetadata {
    pub institution_id: String, 
    pub collection_id: String, 
    pub owner: Principal,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>, // Consider minting it as well
    pub external_url: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub category: CollectionCategory,
    pub documents: Vec<String>, // List of document IDs
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CollectionCategory {
    UniversityGraduationCertificate,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct DocumentMetadata {
    pub collection_id: String, 
    pub document_id: String,
    pub owner: Principal,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub document_hash: String,
    pub file_size: u64,        
    pub file_type: String,     
    pub minted_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct Document {
    pub document_id: String,
    pub file_data: Vec<u8>,     
    pub file_type: String,
    pub uploaded_at: u64,
}

// Certificate-specific metadata (linked via document_id)
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Certificate {
    pub document_id: String, // Links to DocumentMetadata
    pub recipient_info: Option<RecipientInfo>,
    pub issued_date: u64,
    pub expiry_date: Option<u64>,
}

// Common types
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct RecipientInfo {
    pub name: String,
    pub id: Option<String>,
    pub email: Option<String>,
}


// Response type
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct NFTResponse {
    pub success: bool,
    pub document_id: Option<String>,
    pub error_message: Option<String>,
    pub document_hash: Option<String>,
}
