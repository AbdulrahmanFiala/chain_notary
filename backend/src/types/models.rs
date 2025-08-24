use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Institution {
    pub institution_id: String, 
    pub owner: Principal,
    pub name: String,
    pub email: String,
    pub created_at: u64,
    pub collections: Vec<String>, 
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CollectionMetadata {
    pub institution_id: String, 
    pub collection_id: String, 
    pub owner: Principal,
    pub name: String,
    pub description: String,
    pub external_url: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub category: CollectionCategory,
    pub documents: Vec<String>, 
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollectionCategory {
    EarningRelease,
}

// Base document structure shared between Document and NFT
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct DocumentBase {
    pub institution_id: String, 
    pub collection_id: String, 
    pub document_id: String,
    pub owner: Principal,
    pub name: String,
    pub company_name: String,
    pub description: String,
    pub document_data: DocumentType,
    pub base_hash: String,     // Hash of all DocumentBase attributes
    pub document_file_hash: String, // Hash of the uploaded document file
}

// Full document with file data
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Document {
    pub document_base_data: DocumentBase,
    pub published_at: u64,
    pub updated_at: u64,
    pub file_size: u64,        
    pub file_type: String,    
    pub file_data: Vec<u8>,     
}

// NFT version without file data
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct DocumentNft {
    pub document_base_data: DocumentBase,
    pub token_id: String,
    pub created_at: u64,        // NFT creation timestamp
    pub tx_id: Option<String>,  // Blockchain transaction ID (None initially, Some(tx_id) after confirmation)
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum DocumentType {
    EarningRelease(EarningReleaseData),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EarningReleaseData {
    pub earning_release_id: String,
    pub quarter: u8,
    pub year: u16,
    pub consolidated_income_data: ConsolidatedIncomeData,
    pub consolidated_balance_sheet_data: ConsolidatedBalanceSheetData,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ConsolidatedIncomeData {
    pub gross_profit: f64,
    pub operating_profit: f64,
    pub ebitda: f64,
    pub profit_before_tax: f64,
    pub net_profit: f64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ConsolidatedBalanceSheetData {
    pub total_assets: f64,
    pub total_equity: f64,
    pub total_liabilities: f64,
    pub total_liabilities_and_equity: f64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct NFTResponse {
    pub success: bool,
    pub tx_id: String,
    pub token_id: String,
    pub error_message: String,
}

// Response type
#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct DocumentResponse {
    pub success: bool,
    pub document_id: String,
    pub error_message: String,
    pub file_hash: String, // Hash of the uploaded file for integrity verification
}
