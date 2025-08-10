use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use crate::types::Document;

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct MintArgs {
    pub token_ids: Vec<String>,
    pub metadata: Option<Document>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct BurnArgs {
    pub token_id: String,
} 