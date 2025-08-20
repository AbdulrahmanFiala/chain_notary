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

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct CollectionMetadata {
    pub institution_id: String, 
    pub collection_id: String, 
    pub owner: Principal,
    pub name: String,
    pub description: Option<String>,
    pub external_url: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub category: Option<CollectionCategory>,
    pub documents: Vec<String>, 
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollectionCategory {
    EarningRelease,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct Document {
    pub institution_id: Option<String>, 
    pub collection_id: Option<String>, 
    pub document_id: String,
    pub owner: Principal,
    pub name: String,
    pub description: Option<String>,
    pub document_hash: Option<String>, // Now optional - backend will calculate it
    pub document_data: DocumentType,
    pub file_size: u64,        
    pub file_type: String,    
    pub file_data: Option<Vec<u8>>,     
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum DocumentType {
    EarningRelease(EarningReleaseData),
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct EarningReleaseData {
    pub earning_release_id: String,
    pub quarter: u8,
    pub year: u16,
    pub consolidated_income_data: ConsolidatedIncomeData,
    pub consolidated_balance_sheet_data: ConsolidatedBalanceSheetData,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct ConsolidatedIncomeData {
    pub gross_profit: f64,
    pub operating_profit: f64,
    pub ebitda: f64,
    pub profit_before_tax: f64,
    pub net_profit: f64,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct ConsolidatedBalanceSheetData {
    pub total_assets: f64,
    pub total_equity: f64,
    pub total_liabilities: f64,
    pub total_liabilities_and_equity: f64,
}

// Response type
#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct DocumentResponse {
    pub success: bool,
    pub document_id: Option<String>,
    pub error_message: Option<String>,
    pub document_hash: Option<String>,
}
