use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, writer::Writer, reader::Reader,
};
use std::cell::RefCell;
use candid::Principal;
use crate::types::{Document, CollectionMetadata, Institution, DocumentNft};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Store complete documents (metadata + file data) in a single storage area
    pub static DOCUMENTS: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    // Store collections separately from documents
    pub static COLLECTIONS: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    // Store institutions separately from collections
    pub static INSTITUTIONS: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

    // Store owner mappings as JSON strings
    pub static OWNER_TOKENS: RefCell<StableBTreeMap<Vec<u8>, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );

    // Store DocumentNfts separately from documents
    pub static DOCUMENT_NFTS: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
        )
    );

    // Store mapping from document IDs to NFT transaction IDs
    pub static DOCUMENT_TO_NFT: RefCell<StableBTreeMap<String, String, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
        )
    );
}

// Helper functions for storage operations
pub fn principal_to_bytes(principal: &Principal) -> Vec<u8> {
    principal.as_slice().to_vec()
}

pub fn bytes_to_principal(bytes: &[u8]) -> Result<Principal, String> {
    match Principal::try_from_slice(bytes) {
        Ok(principal) => Ok(principal),
        Err(_) => Err("Invalid principal bytes".to_string())
    }
}

// Helper function to safely get and deserialize a document
pub fn get_document_safe(document_id: &str) -> Option<Document> {
    DOCUMENTS.with(|storage| {
        storage.borrow().get(&document_id.to_string())
            .and_then(|bytes| bytes_to_document(&bytes).ok())
    })
}

// Helper function to safely get and deserialize a collection
pub fn get_collection_safe(collection_id: &str) -> Option<CollectionMetadata> {
    COLLECTIONS.with(|storage| {
        storage.borrow().get(&collection_id.to_string())
            .and_then(|bytes| bytes_to_collection(&bytes).ok())
    })
}

// Helper function to safely get and deserialize an institution
pub fn get_institution_safe(institution_id: &str) -> Option<Institution> {
    INSTITUTIONS.with(|storage| {
        storage.borrow().get(&institution_id.to_string())
            .and_then(|bytes| bytes_to_institution(&bytes).ok())
    })
}

// Helper function to safely update a document
pub fn update_document_safe(document_id: &str, document: &Document) -> Result<(), String> {
    let document_bytes = document_to_bytes(document)
        .map_err(|e| format!("Failed to serialize document: {}", e))?;
    DOCUMENTS.with(|storage| {
        storage.borrow_mut().insert(document_id.to_string(), document_bytes);
    });
    Ok(())
}

// Helper function to safely update a collection
pub fn update_collection_safe(collection_id: &str, collection: &CollectionMetadata) -> Result<(), String> {
    let collection_bytes = collection_to_bytes(collection)
        .map_err(|e| format!("Failed to serialize collection: {}", e))?;
    COLLECTIONS.with(|storage| {
        storage.borrow_mut().insert(collection_id.to_string(), collection_bytes);
    });
    Ok(())
}

// Helper function to safely update an institution
pub fn update_institution_safe(institution_id: &str, institution: &Institution) -> Result<(), String> {
    let institution_bytes = institution_to_bytes(institution)
        .map_err(|e| format!("Failed to serialize institution: {}", e))?;
    INSTITUTIONS.with(|storage| {
        storage.borrow_mut().insert(institution_id.to_string(), institution_bytes);
    });
    Ok(())
}

pub fn document_to_bytes(document: &Document) -> Result<Vec<u8>, String> {
    serde_json::to_vec(document).map_err(|e| format!("Failed to serialize document: {}", e))
}

pub fn bytes_to_document(bytes: &[u8]) -> Result<Document, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("Failed to deserialize document: {}", e))
}

pub fn collection_to_bytes(collection: &CollectionMetadata) -> Result<Vec<u8>, String> {
    serde_json::to_vec(collection).map_err(|e| format!("Failed to serialize collection: {}", e))
}

pub fn bytes_to_collection(bytes: &[u8]) -> Result<CollectionMetadata, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("Failed to deserialize collection: {}", e))
}

pub fn tokens_to_bytes(tokens: &[String]) -> Result<Vec<u8>, String> {
    serde_json::to_vec(tokens).map_err(|e| format!("Failed to serialize tokens: {}", e))
}

pub fn bytes_to_tokens(bytes: &[u8]) -> Result<Vec<String>, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("Failed to deserialize tokens: {}", e))
}

pub fn institution_to_bytes(institution: &Institution) -> Result<Vec<u8>, String> {
    serde_json::to_vec(institution).map_err(|e| format!("Failed to serialize institution: {}", e))
} 

pub fn bytes_to_institution(bytes: &[u8]) -> Result<Institution, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("Failed to deserialize institution: {}", e))
}

// Helper functions for DocumentNft storage operations
pub fn get_document_nft_safe(tx_id: &str) -> Option<DocumentNft> {
    DOCUMENT_NFTS.with(|storage| {
        storage.borrow().get(&tx_id.to_string())
            .and_then(|bytes| bytes_to_document_nft(&bytes).ok())
    })
}

pub fn get_nft_by_document_id(document_id: &str) -> Option<String> {
    DOCUMENT_TO_NFT.with(|mapping| {
        mapping.borrow().get(&document_id.to_string())
    })
}

pub fn create_document_nft_safe(tx_id: &str, nft: &DocumentNft) -> Result<(), String> {
    let nft_bytes = document_nft_to_bytes(nft)
        .map_err(|e| format!("Failed to serialize document nft: {}", e))?;
    DOCUMENT_NFTS.with(|storage| {
        storage.borrow_mut().insert(tx_id.to_string(), nft_bytes);
    });
    Ok(())
}

pub fn link_document_to_nft(document_id: &str, tx_id: &str) -> Result<(), String> {
    DOCUMENT_TO_NFT.with(|mapping| {
        mapping.borrow_mut().insert(document_id.to_string(), tx_id.to_string());
    });
    Ok(())
}

pub fn document_nft_to_bytes(nft: &DocumentNft) -> Result<Vec<u8>, String> {
    serde_json::to_vec(nft).map_err(|e| format!("Failed to serialize document nft: {}", e))
}

pub fn bytes_to_document_nft(bytes: &[u8]) -> Result<DocumentNft, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("Failed to deserialize document nft: {}", e))
}

// ============================================================================
// UPGRADE FUNCTIONS
// ============================================================================

/// Save all stable data to stable memory during pre_upgrade
pub fn save_stable_data() -> Result<(), String> {
    // Get a dedicated memory for upgrade data (using a high memory ID to avoid conflicts)
    let mut upgrade_memory = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(100)));
    let mut writer = Writer::new(&mut upgrade_memory, 0);

    // Serialize the count of each storage type first for restoration
    let document_count = DOCUMENTS.with(|storage| storage.borrow().len() as u64);
    let collection_count = COLLECTIONS.with(|storage| storage.borrow().len() as u64);
    let institution_count = INSTITUTIONS.with(|storage| storage.borrow().len() as u64);
    let owner_token_count = OWNER_TOKENS.with(|storage| storage.borrow().len() as u64);
    let document_nft_count = DOCUMENT_NFTS.with(|storage| storage.borrow().len() as u64);
    let doc_to_nft_count = DOCUMENT_TO_NFT.with(|storage| storage.borrow().len() as u64);

    // Write counts
    writer.write(&document_count.to_le_bytes()).map_err(|e| format!("Failed to write document count: {:?}", e))?;
    writer.write(&collection_count.to_le_bytes()).map_err(|e| format!("Failed to write collection count: {:?}", e))?;
    writer.write(&institution_count.to_le_bytes()).map_err(|e| format!("Failed to write institution count: {:?}", e))?;
    writer.write(&owner_token_count.to_le_bytes()).map_err(|e| format!("Failed to write owner token count: {:?}", e))?;
    writer.write(&document_nft_count.to_le_bytes()).map_err(|e| format!("Failed to write document nft count: {:?}", e))?;
    writer.write(&doc_to_nft_count.to_le_bytes()).map_err(|e| format!("Failed to write doc to nft count: {:?}", e))?;

    // Save all documents
    DOCUMENTS.with(|storage| {
        for (key, value) in storage.borrow().iter() {
            let key_bytes = key.as_bytes();
            let key_len = (key_bytes.len() as u32).to_le_bytes();
            let value_len = (value.len() as u32).to_le_bytes();
            
            writer.write(&key_len).map_err(|e| format!("Failed to write key length: {:?}", e))?;
            writer.write(key_bytes).map_err(|e| format!("Failed to write key: {:?}", e))?;
            writer.write(&value_len).map_err(|e| format!("Failed to write value length: {:?}", e))?;
            writer.write(&value).map_err(|e| format!("Failed to write value: {:?}", e))?;
        }
        Ok::<(), String>(())
    })?;

    // Save all collections
    COLLECTIONS.with(|storage| {
        for (key, value) in storage.borrow().iter() {
            let key_bytes = key.as_bytes();
            let key_len = (key_bytes.len() as u32).to_le_bytes();
            let value_len = (value.len() as u32).to_le_bytes();
            
            writer.write(&key_len).map_err(|e| format!("Failed to write key length: {:?}", e))?;
            writer.write(key_bytes).map_err(|e| format!("Failed to write key: {:?}", e))?;
            writer.write(&value_len).map_err(|e| format!("Failed to write value length: {:?}", e))?;
            writer.write(&value).map_err(|e| format!("Failed to write value: {:?}", e))?;
        }
        Ok::<(), String>(())
    })?;

    // Save all institutions
    INSTITUTIONS.with(|storage| {
        for (key, value) in storage.borrow().iter() {
            let key_bytes = key.as_bytes();
            let key_len = (key_bytes.len() as u32).to_le_bytes();
            let value_len = (value.len() as u32).to_le_bytes();
            
            writer.write(&key_len).map_err(|e| format!("Failed to write key length: {:?}", e))?;
            writer.write(key_bytes).map_err(|e| format!("Failed to write key: {:?}", e))?;
            writer.write(&value_len).map_err(|e| format!("Failed to write value length: {:?}", e))?;
            writer.write(&value).map_err(|e| format!("Failed to write value: {:?}", e))?;
        }
        Ok::<(), String>(())
    })?;

    // Save all owner tokens
    OWNER_TOKENS.with(|storage| {
        for (key, value) in storage.borrow().iter() {
            let key_len = (key.len() as u32).to_le_bytes();
            let value_len = (value.len() as u32).to_le_bytes();
            
            writer.write(&key_len).map_err(|e| format!("Failed to write key length: {:?}", e))?;
            writer.write(&key).map_err(|e| format!("Failed to write key: {:?}", e))?;
            writer.write(&value_len).map_err(|e| format!("Failed to write value length: {:?}", e))?;
            writer.write(&value).map_err(|e| format!("Failed to write value: {:?}", e))?;
        }
        Ok::<(), String>(())
    })?;

    // Save all document NFTs
    DOCUMENT_NFTS.with(|storage| {
        for (key, value) in storage.borrow().iter() {
            let key_bytes = key.as_bytes();
            let key_len = (key_bytes.len() as u32).to_le_bytes();
            let value_len = (value.len() as u32).to_le_bytes();
            
            writer.write(&key_len).map_err(|e| format!("Failed to write key length: {:?}", e))?;
            writer.write(key_bytes).map_err(|e| format!("Failed to write key: {:?}", e))?;
            writer.write(&value_len).map_err(|e| format!("Failed to write value length: {:?}", e))?;
            writer.write(&value).map_err(|e| format!("Failed to write value: {:?}", e))?;
        }
        Ok::<(), String>(())
    })?;

    // Save all document to NFT mappings
    DOCUMENT_TO_NFT.with(|storage| {
        for (key, value) in storage.borrow().iter() {
            let key_bytes = key.as_bytes();
            let value_bytes = value.as_bytes();
            let key_len = (key_bytes.len() as u32).to_le_bytes();
            let value_len = (value_bytes.len() as u32).to_le_bytes();
            
            writer.write(&key_len).map_err(|e| format!("Failed to write key length: {:?}", e))?;
            writer.write(key_bytes).map_err(|e| format!("Failed to write key: {:?}", e))?;
            writer.write(&value_len).map_err(|e| format!("Failed to write value length: {:?}", e))?;
            writer.write(value_bytes).map_err(|e| format!("Failed to write value: {:?}", e))?;
        }
        Ok::<(), String>(())
    })?;

    Ok(())
}

/// Restore all stable data from stable memory during post_upgrade
pub fn restore_stable_data() -> Result<(), String> {
    // Get the same memory used for upgrade data
    let upgrade_memory = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(100)));
    let mut reader = Reader::new(&upgrade_memory, 0);

    // Try to read counts - if this fails, it means no backup data exists (first deployment)
    let mut count_buf = [0u8; 8];
    match reader.read(&mut count_buf) {
        Ok(_) => {
            // Backup data exists, proceed with restoration
        },
        Err(_) => {
            // No backup data exists (first deployment with upgrade hooks)
            // This is normal and not an error
            return Ok(());
        }
    }
    let document_count = u64::from_le_bytes(count_buf);

    reader.read(&mut count_buf).map_err(|e| format!("Failed to read collection count: {:?}", e))?;
    let collection_count = u64::from_le_bytes(count_buf);

    reader.read(&mut count_buf).map_err(|e| format!("Failed to read institution count: {:?}", e))?;
    let institution_count = u64::from_le_bytes(count_buf);

    reader.read(&mut count_buf).map_err(|e| format!("Failed to read owner token count: {:?}", e))?;
    let owner_token_count = u64::from_le_bytes(count_buf);

    reader.read(&mut count_buf).map_err(|e| format!("Failed to read document nft count: {:?}", e))?;
    let document_nft_count = u64::from_le_bytes(count_buf);

    reader.read(&mut count_buf).map_err(|e| format!("Failed to read doc to nft count: {:?}", e))?;
    let doc_to_nft_count = u64::from_le_bytes(count_buf);

    // Restore documents
    for _ in 0..document_count {
        let (key, value) = read_key_value_bytes(&mut reader)?;
        let key_str = String::from_utf8(key).map_err(|e| format!("Invalid UTF-8 in document key: {}", e))?;
        DOCUMENTS.with(|storage| {
            storage.borrow_mut().insert(key_str, value);
        });
    }

    // Restore collections
    for _ in 0..collection_count {
        let (key, value) = read_key_value_bytes(&mut reader)?;
        let key_str = String::from_utf8(key).map_err(|e| format!("Invalid UTF-8 in collection key: {}", e))?;
        COLLECTIONS.with(|storage| {
            storage.borrow_mut().insert(key_str, value);
        });
    }

    // Restore institutions
    for _ in 0..institution_count {
        let (key, value) = read_key_value_bytes(&mut reader)?;
        let key_str = String::from_utf8(key).map_err(|e| format!("Invalid UTF-8 in institution key: {}", e))?;
        INSTITUTIONS.with(|storage| {
            storage.borrow_mut().insert(key_str, value);
        });
    }

    // Restore owner tokens
    for _ in 0..owner_token_count {
        let (key, value) = read_key_value_bytes(&mut reader)?;
        OWNER_TOKENS.with(|storage| {
            storage.borrow_mut().insert(key, value);
        });
    }

    // Restore document NFTs
    for _ in 0..document_nft_count {
        let (key, value) = read_key_value_bytes(&mut reader)?;
        let key_str = String::from_utf8(key).map_err(|e| format!("Invalid UTF-8 in document nft key: {}", e))?;
        DOCUMENT_NFTS.with(|storage| {
            storage.borrow_mut().insert(key_str, value);
        });
    }

    // Restore document to NFT mappings
    for _ in 0..doc_to_nft_count {
        let (key, value) = read_key_value_bytes(&mut reader)?;
        let key_str = String::from_utf8(key).map_err(|e| format!("Invalid UTF-8 in doc to nft key: {}", e))?;
        let value_str = String::from_utf8(value).map_err(|e| format!("Invalid UTF-8 in doc to nft value: {}", e))?;
        DOCUMENT_TO_NFT.with(|storage| {
            storage.borrow_mut().insert(key_str, value_str);
        });
    }

    Ok(())
}

/// Helper function to read key-value pairs from stable memory
fn read_key_value_bytes(reader: &mut Reader<VirtualMemory<DefaultMemoryImpl>>) -> Result<(Vec<u8>, Vec<u8>), String> {
    let mut len_buf = [0u8; 4];
    
    // Read key length
    reader.read(&mut len_buf).map_err(|e| format!("Failed to read key length: {:?}", e))?;
    let key_len = u32::from_le_bytes(len_buf) as usize;
    
    // Read key
    let mut key = vec![0u8; key_len];
    reader.read(&mut key).map_err(|e| format!("Failed to read key: {:?}", e))?;
    
    // Read value length
    reader.read(&mut len_buf).map_err(|e| format!("Failed to read value length: {:?}", e))?;
    let value_len = u32::from_le_bytes(len_buf) as usize;
    
    // Read value
    let mut value = vec![0u8; value_len];
    reader.read(&mut value).map_err(|e| format!("Failed to read value: {:?}", e))?;
    
    Ok((key, value))
} 