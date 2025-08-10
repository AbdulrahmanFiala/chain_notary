use candid::{CandidType, Deserialize};
use serde::Serialize;

// Error types
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub enum MintError {
    InvalidTokenId,
    TokenExists,
    Unauthorized,
    InvalidMetadata,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub enum BurnError {
    TokenNotFound,
    Unauthorized,
}

// Result types
pub type MintResult = Result<(), MintError>;
pub type BurnResult = Result<(), BurnError>; 