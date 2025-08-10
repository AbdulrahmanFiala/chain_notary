use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Institution {
    pub institution_id: String, 
    pub owner: Principal,
    pub name: String,
    pub email: String,
    pub created_at: u64,
    pub collections: Vec<String>, 
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
    pub category: Option<CollectionCategory>,
    pub documents: Vec<String>, // List of document IDs
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CollectionCategory {
    UniversityGraduationCertificate,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct Document {
    pub collection_id: String, 
    pub document_id: String,
    pub owner: Principal,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub document_hash: String,
    pub file_size: u64,        
    pub file_type: String,    
    pub file_data: Option<Vec<u8>>,     
    pub minted_at: u64,
    pub recipient: Option<RecipientInfo>
}


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
