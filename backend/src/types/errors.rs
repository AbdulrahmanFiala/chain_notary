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
pub enum TransferError {
    TokenNotFound,
    Unauthorized,
    InvalidRecipient,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub enum ApproveError {
    TokenNotFound,
    Unauthorized,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub enum TransferFromError {
    TokenNotFound,
    Unauthorized,
    InvalidRecipient,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub enum BurnError {
    TokenNotFound,
    Unauthorized,
}

// Result types
pub type MintResult = Result<(), MintError>;
pub type TransferResult = Result<(), TransferError>;
pub type ApproveResult = Result<(), ApproveError>;
pub type TransferFromResult = Result<(), TransferFromError>;
pub type BurnResult = Result<(), BurnError>; 