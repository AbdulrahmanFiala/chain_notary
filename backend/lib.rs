use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{export_candid, query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use serde::{Deserialize as SerdeDeserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

// ICRC-37 Standard Types
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct CollectionMetadata {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub url: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct TokenMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub external_url: Option<String>,
    pub attributes: Vec<TokenAttribute>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct TokenAttribute {
    pub trait_type: String,
    pub value: String,
    pub display_type: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct MintArgs {
    pub token_ids: Vec<String>,
    pub metadata: Option<TokenMetadata>,
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

// Error types
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub enum MintError {
    InvalidTokenId,
    TokenExists,
    Unauthorized,
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

// Custom types for file upload
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub external_url: Option<String>,
    pub attributes: Vec<Attribute>,
    pub properties: Option<Properties>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
    pub display_type: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct Properties {
    pub files: Option<Vec<FileProperty>>,
    pub category: Option<String>,
    pub creators: Option<Vec<Creator>>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct FileProperty {
    pub uri: String,
    pub r#type: String,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct Creator {
    pub address: String,
    pub share: u32,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct FileUploadRequest {
    pub file_data: Vec<u8>,
    pub file_name: String,
    pub file_type: String,
    pub metadata: NFTMetadata,
    pub owner: Principal,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct NFTResponse {
    pub success: bool,
    pub token_id: Option<String>,
    pub error_message: Option<String>,
    pub ipfs_hash: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct NFTInfo {
    pub token_id: String,
    pub metadata: NFTMetadata,
    pub owner: Principal,
    pub created_at: u64,
    pub file_hash: String,
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Store NFT metadata as JSON strings (Storable)
    static NFT_METADATA: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    // Store file data
    static FILE_STORAGE: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

    // Store owner mappings as JSON strings
    static OWNER_TOKENS: RefCell<StableBTreeMap<Vec<u8>, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );

    // Store approvals
    static APPROVALS: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
        )
    );
}

fn calculate_file_hash(file_data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(file_data);
    hex::encode(hasher.finalize())
}

fn generate_token_id() -> String {
    // Use timestamp and a simple counter for unique IDs
    let timestamp = ic_cdk::api::time();
    format!("nft_{}", timestamp)
}

fn principal_to_bytes(principal: &Principal) -> Vec<u8> {
    principal.as_slice().to_vec()
}

fn bytes_to_principal(bytes: &[u8]) -> Principal {
    Principal::from_slice(bytes)
}

fn nft_info_to_bytes(nft_info: &NFTInfo) -> Vec<u8> {
    serde_json::to_vec(nft_info).unwrap_or_default()
}

fn bytes_to_nft_info(bytes: &[u8]) -> Option<NFTInfo> {
    serde_json::from_slice(bytes).ok()
}

fn tokens_to_bytes(tokens: &[String]) -> Vec<u8> {
    serde_json::to_vec(tokens).unwrap_or_default()
}

fn bytes_to_tokens(bytes: &[u8]) -> Vec<String> {
    serde_json::from_slice(bytes).unwrap_or_default()
}

// ICRC-37 Standard Implementation
#[update]
fn icrc37_mint(args: MintArgs) -> MintResult {
    // Validate mint arguments
    if args.token_ids.is_empty() {
        return Err(MintError::InvalidTokenId);
    }

    let caller = ic_cdk::caller();
    
    for token_id in args.token_ids {
        // Check if token already exists
        if NFT_METADATA.with(|storage| storage.borrow().contains_key(&token_id)) {
            return Err(MintError::TokenExists);
        }

        // Create NFT info with default metadata
        let nft_info = NFTInfo {
            token_id: token_id.clone(),
            metadata: NFTMetadata {
                name: args.metadata.as_ref().map(|m| m.name.clone()).unwrap_or_else(|| "Minted NFT".to_string()),
                description: args.metadata.as_ref().map(|m| m.description.clone()).unwrap_or_else(|| "NFT minted via ICRC-37".to_string()),
                image_url: args.metadata.as_ref().map(|m| m.image.clone()).unwrap_or_else(|| "".to_string()),
                external_url: args.metadata.as_ref().and_then(|m| m.external_url.clone()),
                attributes: args.metadata.as_ref().map(|m| {
                    m.attributes.iter().map(|attr| Attribute {
                        trait_type: attr.trait_type.clone(),
                        value: attr.value.clone(),
                        display_type: attr.display_type.clone(),
                    }).collect()
                }).unwrap_or_default(),
                properties: None,
            },
            owner: caller,
            created_at: ic_cdk::api::time(),
            file_hash: "".to_string(),
        };

        // Store NFT metadata
        NFT_METADATA.with(|storage| {
            storage.borrow_mut().insert(token_id.clone(), nft_info_to_bytes(&nft_info));
        });

        // Update owner's token list
        OWNER_TOKENS.with(|owner_tokens| {
            let mut tokens = owner_tokens.borrow_mut();
            let caller_bytes = principal_to_bytes(&caller);
            let current_tokens_bytes = tokens.get(&caller_bytes).unwrap_or_default();
            let mut current_tokens = bytes_to_tokens(&current_tokens_bytes);
            current_tokens.push(token_id);
            tokens.insert(caller_bytes, tokens_to_bytes(&current_tokens));
        });
    }

    Ok(())
}

#[update]
fn icrc37_transfer(args: TransferArgs) -> TransferResult {
    let caller = ic_cdk::caller();
    
    // Check if token exists and caller owns it
    let nft_info_bytes = NFT_METADATA.with(|storage| {
        storage.borrow().get(&args.token_id)
    });

    if nft_info_bytes.is_none() {
        return Err(TransferError::TokenNotFound);
    }

    let nft_info = bytes_to_nft_info(&nft_info_bytes.unwrap());
    if nft_info.is_none() || nft_info.as_ref().unwrap().owner != caller {
        return Err(TransferError::Unauthorized);
    }

    let mut nft_info = nft_info.unwrap();
    let old_owner = nft_info.owner;

    // Remove from old owner
    OWNER_TOKENS.with(|owner_tokens| {
        let mut tokens = owner_tokens.borrow_mut();
        let old_owner_bytes = principal_to_bytes(&old_owner);
        let current_tokens_bytes = tokens.get(&old_owner_bytes).unwrap_or_default();
        let mut current_tokens = bytes_to_tokens(&current_tokens_bytes);
        current_tokens.retain(|id| id != &args.token_id);
        tokens.insert(old_owner_bytes, tokens_to_bytes(&current_tokens));
    });

    // Update owner
    nft_info.owner = args.to;
    NFT_METADATA.with(|storage| {
        storage.borrow_mut().insert(args.token_id.clone(), nft_info_to_bytes(&nft_info));
    });

    // Add to new owner
    OWNER_TOKENS.with(|owner_tokens| {
        let mut tokens = owner_tokens.borrow_mut();
        let new_owner_bytes = principal_to_bytes(&args.to);
        let current_tokens_bytes = tokens.get(&new_owner_bytes).unwrap_or_default();
        let mut current_tokens = bytes_to_tokens(&current_tokens_bytes);
        current_tokens.push(args.token_id);
        tokens.insert(new_owner_bytes, tokens_to_bytes(&current_tokens));
    });

    Ok(())
}

#[update]
fn icrc37_approve(args: ApproveArgs) -> ApproveResult {
    let caller = ic_cdk::caller();
    
    // Check if caller owns the token
    let nft_info_bytes = NFT_METADATA.with(|storage| {
        storage.borrow().get(&args.token_id)
    });

    if nft_info_bytes.is_none() {
        return Err(ApproveError::TokenNotFound);
    }

    let nft_info = bytes_to_nft_info(&nft_info_bytes.unwrap());
    if nft_info.is_none() || nft_info.as_ref().unwrap().owner != caller {
        return Err(ApproveError::Unauthorized);
    }

    // Store approval
    APPROVALS.with(|approvals| {
        approvals.borrow_mut().insert(args.token_id, principal_to_bytes(&args.spender));
    });

    Ok(())
}

#[update]
fn icrc37_transfer_from(args: TransferFromArgs) -> TransferFromResult {
    let caller = ic_cdk::caller();
    
    // Check if caller is approved or owner
    let is_approved = APPROVALS.with(|approvals| {
        approvals.borrow().get(&args.token_id) == Some(principal_to_bytes(&caller))
    });

    let nft_info_bytes = NFT_METADATA.with(|storage| {
        storage.borrow().get(&args.token_id)
    });

    if nft_info_bytes.is_none() {
        return Err(TransferFromError::TokenNotFound);
    }

    let nft_info = bytes_to_nft_info(&nft_info_bytes.unwrap());
    if nft_info.is_none() {
        return Err(TransferFromError::TokenNotFound);
    }

    let is_owner = nft_info.as_ref().unwrap().owner == caller;

    if !is_approved && !is_owner {
        return Err(TransferFromError::Unauthorized);
    }

    // Perform transfer
    match icrc37_transfer(TransferArgs {
        token_id: args.token_id.clone(),
        to: args.to,
    }) {
        Ok(()) => {
            // Remove approval after transfer
            APPROVALS.with(|approvals| {
                approvals.borrow_mut().remove(&args.token_id);
            });
            Ok(())
        }
        Err(TransferError::TokenNotFound) => Err(TransferFromError::TokenNotFound),
        Err(TransferError::Unauthorized) => Err(TransferFromError::Unauthorized),
        Err(TransferError::InvalidRecipient) => Err(TransferFromError::InvalidRecipient),
    }
}

#[update]
fn icrc37_burn(args: BurnArgs) -> BurnResult {
    let caller = ic_cdk::caller();
    
    // Check if caller owns the token
    let nft_info_bytes = NFT_METADATA.with(|storage| {
        storage.borrow().get(&args.token_id)
    });

    if nft_info_bytes.is_none() {
        return Err(BurnError::TokenNotFound);
    }

    let nft_info = bytes_to_nft_info(&nft_info_bytes.unwrap());
    if nft_info.is_none() || nft_info.as_ref().unwrap().owner != caller {
        return Err(BurnError::Unauthorized);
    }

    // Remove from storage
    NFT_METADATA.with(|storage| {
        storage.borrow_mut().remove(&args.token_id);
    });

    // Remove from owner's token list
    OWNER_TOKENS.with(|owner_tokens| {
        let mut tokens = owner_tokens.borrow_mut();
        let caller_bytes = principal_to_bytes(&caller);
        let current_tokens_bytes = tokens.get(&caller_bytes).unwrap_or_default();
        let mut current_tokens = bytes_to_tokens(&current_tokens_bytes);
        current_tokens.retain(|id| id != &args.token_id);
        tokens.insert(caller_bytes, tokens_to_bytes(&current_tokens));
    });

    // Remove approval if exists
    APPROVALS.with(|approvals| {
        approvals.borrow_mut().remove(&args.token_id);
    });

    Ok(())
}

#[query]
fn icrc37_metadata() -> CollectionMetadata {
    CollectionMetadata {
        name: "Chain Notary NFTs".to_string(),
        symbol: "CNFT".to_string(),
        description: Some("NFTs created on Chain Notary platform".to_string()),
        logo: None,
        url: None,
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
    }
}

// Custom upload endpoint (non-ICRC-37)
#[update]
async fn upload_file_and_create_nft(request: FileUploadRequest) -> NFTResponse {
    // Validate file size (max 2MB for demo)
    if request.file_data.len() > 2 * 1024 * 1024 {
        return NFTResponse {
            success: false,
            token_id: None,
            error_message: Some("File size exceeds 2MB limit".to_string()),
            ipfs_hash: None,
        };
    }

    // Validate file type
    let allowed_types = vec!["image/jpeg", "image/png", "image/gif", "image/webp"];
    if !allowed_types.contains(&request.file_type.as_str()) {
        return NFTResponse {
            success: false,
            token_id: None,
            error_message: Some("Unsupported file type. Only JPEG, PNG, GIF, and WebP are allowed.".to_string()),
            ipfs_hash: None,
        };
    }

    // Generate unique token ID
    let token_id = generate_token_id();
    
    // Calculate file hash
    let file_hash = calculate_file_hash(&request.file_data);
    
    // Get current timestamp
    let created_at = ic_cdk::api::time();

    // Store the file data
    FILE_STORAGE.with(|storage| {
        storage.borrow_mut().insert(token_id.clone(), request.file_data);
    });

    // Create NFT info
    let nft_info = NFTInfo {
        token_id: token_id.clone(),
        metadata: request.metadata,
        owner: request.owner,
        created_at,
        file_hash: file_hash.clone(),
    };

    // Store the NFT metadata
    NFT_METADATA.with(|storage| {
        storage.borrow_mut().insert(token_id.clone(), nft_info_to_bytes(&nft_info));
    });

    // Update owner's token list
    OWNER_TOKENS.with(|owner_tokens| {
        let mut tokens = owner_tokens.borrow_mut();
        let owner_bytes = principal_to_bytes(&request.owner);
        let current_tokens_bytes = tokens.get(&owner_bytes).unwrap_or_default();
        let mut current_tokens = bytes_to_tokens(&current_tokens_bytes);
        current_tokens.push(token_id.clone());
        tokens.insert(owner_bytes, tokens_to_bytes(&current_tokens));
    });

    NFTResponse {
        success: true,
        token_id: Some(token_id),
        error_message: None,
        ipfs_hash: Some(file_hash),
    }
}

#[query]
fn get_nft_metadata(token_id: String) -> Option<NFTInfo> {
    NFT_METADATA.with(|storage| {
        storage.borrow().get(&token_id).and_then(|bytes| bytes_to_nft_info(&bytes))
    })
}

#[query]
fn get_nft_file(token_id: String) -> Option<Vec<u8>> {
    FILE_STORAGE.with(|storage| {
        storage.borrow().get(&token_id)
    })
}

#[query]
fn list_all_nfts() -> Vec<String> {
    NFT_METADATA.with(|storage| {
        storage.borrow().iter().map(|(k, _)| k.clone()).collect()
    })
}

#[query]
fn get_nfts_by_owner(owner: Principal) -> Vec<String> {
    OWNER_TOKENS.with(|owner_tokens| {
        let owner_bytes = principal_to_bytes(&owner);
        owner_tokens.borrow().get(&owner_bytes).map(|bytes| bytes_to_tokens(&bytes)).unwrap_or_default()
    })
}

#[query]
fn get_nft_count() -> u64 {
    NFT_METADATA.with(|storage| {
        storage.borrow().len() as u64
    })
}

#[query]
fn get_total_supply() -> u64 {
    get_nft_count()
}

// Keep the original greeting functionality for compatibility
thread_local! {
    static GREETING: RefCell<ic_stable_structures::Cell<String, Memory>> = RefCell::new(
        ic_stable_structures::Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), "Hello, ".to_string()
        ).unwrap()
    );
}

#[update]
fn set_greeting(prefix: String) {
    GREETING.with_borrow_mut(|greeting| greeting.set(prefix).unwrap());
}

#[query]
fn greet(name: String) -> String {
    GREETING.with_borrow(|greeting| format!("{}{name}!", greeting.get()))
}

export_candid!();
