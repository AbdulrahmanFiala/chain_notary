use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use crate::types::DocumentMetadata;

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct MintArgs {
    pub token_ids: Vec<String>,
    pub metadata: Option<DocumentMetadata>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct TransferArgs {
    pub token_id: String,
    pub to: Principal,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct ApproveArgs {
    pub token_id: String,
    pub spender: Principal,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct TransferFromArgs {
    pub token_id: String,
    pub from: Principal,
    pub to: Principal,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct BurnArgs {
    pub token_id: String,
} 