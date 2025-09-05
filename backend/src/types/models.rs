use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Institution {
    pub institution_id: String, 
    pub owner: Principal,
    pub name: String,
    pub email: String,
    pub created_at: u64,
}

impl Default for Institution {
    fn default() -> Self {
        Self {
            institution_id: String::default(),
            owner: Principal::anonymous(),
            name: String::default(),
            email: String::default(),
            created_at: 0,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollectionCategory {
    EarningRelease,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UserRole {
    SuperAdmin,
    RegularUser,
    InstitutionMember(String), // Institution ID they belong to
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserProfile {
    pub internet_identity: Principal,
    pub role: UserRole,
    pub assigned_institution_id: String, // Assigned by admin (empty string if none)
    pub created_at: u64,
    pub last_login: u64,
}

impl Default for CollectionCategory {
    fn default() -> Self {
        CollectionCategory::EarningRelease
    }
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct Document {
    pub institution_id: String, 
    pub document_id: String,
    pub owner: Principal,
    pub name: String,
    pub company_name: String,
    pub description: String,
    pub document_data: DocumentType,
    pub document_category: CollectionCategory,
    pub file_hash: String,
    pub file_size: u64,        
    pub file_type: String,    
    pub file_data: Vec<u8>,     
}

impl Default for Document {
    fn default() -> Self {
        Self {
            institution_id: String::default(),
            document_id: String::default(),
            owner: Principal::anonymous(),
            name: String::default(),
            company_name: String::default(),
            description: String::default(),
            document_data: DocumentType::default(),
            document_category: CollectionCategory::default(),
            file_hash: String::default(),
            file_size: 0,
            file_type: String::default(),
            file_data: Vec::default(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum DocumentType {
    EarningRelease(EarningReleaseData),
}

impl Default for DocumentType {
    fn default() -> Self {
        DocumentType::EarningRelease(EarningReleaseData::default())
    }
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, Default)]
pub struct EarningReleaseData {
    pub earning_release_id: String,
    pub quarter: u8,
    pub year: u16,
    pub consolidated_income_data: ConsolidatedIncomeData,
    pub consolidated_balance_sheet_data: ConsolidatedBalanceSheetData,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct ConsolidatedIncomeData {
    pub gross_profit: f64,
    pub operating_profit: f64,
    pub ebitda: f64,
    pub profit_before_tax: f64,
    pub net_profit: f64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct ConsolidatedBalanceSheetData {
    pub total_assets: f64,
    pub total_equity: f64,
    pub total_liabilities: f64,
    pub total_liabilities_and_equity: f64,
}

// Response type
#[derive(CandidType, Deserialize, Clone, Serialize, Debug, Default)]
pub struct DocumentResponse {
    pub success: bool,
    pub document_id: String,
    pub error_message: String,
    pub file_hash: String,
}
