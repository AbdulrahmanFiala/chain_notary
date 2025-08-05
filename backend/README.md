# NFT Backend API

This backend provides NFT (Non-Fungible Token) functionality for the Internet Computer (IC) blockchain. It allows users to upload files and create NFTs with metadata.

## Features

- File upload and NFT creation
- Metadata storage with standard NFT metadata format
- File type validation (JPEG, PNG, GIF, WebP)
- File size validation (max 2MB)
- Owner-based NFT tracking
- File hash calculation for integrity
- Query endpoints for NFT retrieval

## API Endpoints

### Upload and Create NFT
```candid
upload_file_and_create_nft : (FileUploadRequest) -> (NFTResponse)
```

**Request Structure:**
```candid
type FileUploadRequest = record {
    file_data : vec nat8;        // Binary file data
    file_name : text;            // Original filename
    file_type : text;            // MIME type (e.g., "image/jpeg")
    metadata : NFTMetadata;      // NFT metadata
    owner : principal;           // Owner's principal ID
};
```

**Response Structure:**
```candid
type NFTResponse = record {
    success : bool;              // Operation success status
    token_id : opt text;        // Generated token ID
    error_message : opt text;    // Error message if failed
    ipfs_hash : opt text;       // File hash (IPFS hash in production)
};
```

### Query Endpoints

#### Get NFT Metadata
```candid
get_nft_metadata : (text) -> (opt NFTInfo) query
```

#### Get NFT File Data
```candid
get_nft_file : (text) -> (opt vec nat8) query
```

#### List All NFTs
```candid
list_all_nfts : () -> (vec text) query
```

#### Get NFTs by Owner
```candid
get_nfts_by_owner : (principal) -> (vec text) query
```

#### Get NFT Count
```candid
get_nft_count : () -> (nat64) query
```

#### Get Total Supply
```candid
get_total_supply : () -> (nat64) query
```

## Data Structures

### NFT Metadata
```candid
type NFTMetadata = record {
    name : text;                 // NFT name
    description : text;          // NFT description
    image_url : text;           // Image URL
    external_url : opt text;    // External URL
    attributes : vec Attribute;  // NFT attributes
    properties : opt Properties; // Additional properties
};
```

### Attribute
```candid
type Attribute = record {
    trait_type : text;          // Attribute type
    value : text;               // Attribute value
    display_type : opt text;    // Display type
};
```

### NFT Info
```candid
type NFTInfo = record {
    token_id : text;            // Unique token ID
    metadata : NFTMetadata;     // NFT metadata
    owner : principal;          // Owner's principal
    created_at : nat64;        // Creation timestamp
    file_hash : text;          // File hash
};
```

## Usage Example

### Frontend Integration

```javascript
// Example of uploading a file and creating an NFT
const uploadNFT = async (file, metadata, owner) => {
    const fileData = await file.arrayBuffer();
    const request = {
        file_data: Array.from(new Uint8Array(fileData)),
        file_name: file.name,
        file_type: file.type,
        metadata: {
            name: "My NFT",
            description: "A unique digital asset",
            image_url: "https://example.com/image.jpg",
            external_url: null,
            attributes: [
                {
                    trait_type: "Color",
                    value: "Blue",
                    display_type: null
                }
            ],
            properties: null
        },
        owner: owner
    };

    const response = await backend.upload_file_and_create_nft(request);
    return response;
};
```

## Validation Rules

1. **File Size**: Maximum 2MB per file
2. **File Types**: Only JPEG, PNG, GIF, and WebP are supported
3. **Metadata**: Must include name, description, and image_url
4. **Owner**: Must be a valid principal ID

## Storage

The backend uses stable memory to store:
- NFT metadata and information
- File binary data
- Owner-to-token mappings

## Production Considerations

For production deployment, consider:
1. Implementing proper IPFS integration for file storage
2. Adding support for NFT standards (EXT, DIP721)
3. Implementing transfer functionality
4. Adding access control and permissions
5. Implementing proper error handling and logging
6. Adding rate limiting and spam protection

## Building and Deploying

```bash
# Build the canister
dfx build backend

# Deploy to local network
dfx deploy backend

# Deploy to mainnet
dfx deploy --network ic backend
``` 