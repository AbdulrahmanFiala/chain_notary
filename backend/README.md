# Chain Notary Backend API

This backend provides a comprehensive document management system for the Internet Computer (IC) blockchain. It allows institutions to create collections and publish documents with structured metadata, particularly focused on financial documents like earning releases.

## Features

- **Institution Management**: Create and manage financial institutions
- **Collection Management**: Organize documents into themed collections
- **Document Publishing**: Upload and publish documents with structured metadata
- **Financial Data Support**: Specialized support for earning release documents
- **File Validation**: File type and size validation
- **Access Control**: Owner-based permissions for all operations
- **Search & Query**: Comprehensive querying across institutions, collections, and documents

## API Endpoints

### Institution Management

#### Create Institution
```candid
create_institution : (text, text, text) -> (Result)
```
**Parameters:**
- `institution_id : text` - Unique identifier for the institution
- `name : text` - Institution name
- `email : text` - Institution email address

#### Update Institution
```candid
update_institution : (text, text, text) -> (Result)
```

#### Delete Institution
```candid
delete_institution : (text) -> (Result)
```

#### Add/Remove Collection from Institution
```candid
add_collection_to_institution : (text, text) -> (Result)
remove_collection_from_institution : (text, text) -> (Result)
```

### Collection Management

#### Create Collection
```candid
create_collection : (text, text, text, text, CollectionCategory, text) -> (Result)
```
**Parameters:**
- `collection_id : text` - Unique identifier for the collection
- `name : text` - Collection name
- `description : text` - Collection description
- `external_url : text` - External URL for the collection
- `category : CollectionCategory` - Collection category (currently supports EarningRelease)
- `institution_id : text` - Institution ID (can be empty for standalone collections)

#### Update Collection
```candid
update_collection : (text, text, text, text, CollectionCategory) -> (Result)
```

#### Delete Collection
```candid
delete_collection : (text) -> (Result)
```

#### Add/Remove Document from Collection
```candid
add_document_to_collection : (text, text) -> (Result)
remove_document_from_collection : (text, text) -> (Result)
```

### Document Management

#### Upload and Publish Document
```candid
upload_file_and_publish_document : (Document) -> (DocumentResponse)
```

**Document Structure:**
```candid
type Document = record {
    institution_id : text;       // Institution ID (can be empty)
    collection_id : text;        // Collection ID (can be empty)
    document_id : text;          // Will be set by the canister
    owner : principal;           // Owner's principal ID
    name : text;                 // Document name
    company_name : text;         // Company name
    description : text;          // Document description
    document_hash : text;        // Must match calculated file hash
    document_data : DocumentType; // Document type and structured data
    file_size : nat64;          // File size in bytes
    file_type : text;           // MIME type
    file_data : blob;           // Binary file data
};
```

**Supported Document Types:**
```candid
type DocumentType = variant {
    EarningRelease : EarningReleaseData
};
```

**Earning Release Data Structure:**
```candid
type EarningReleaseData = record {
    earning_release_id : text;
    quarter : nat8;
    year : nat16;
    consolidated_income_data : ConsolidatedIncomeData;
    consolidated_balance_sheet_data : ConsolidatedBalanceSheetData;
};
```

**Response Structure:**
```candid
type DocumentResponse = record {
    success : bool;              // Operation success status
    document_id : text;          // Generated document ID
    error_message : text;        // Error message if failed
    document_hash : text;        // File hash for verification
};
```

### Query Endpoints

#### Institution Queries
```candid
get_institution_metadata : (text) -> (opt Institution) query
get_all_institutions : () -> (vec Institution) query
get_institutions_by_owner : (principal) -> (vec Institution) query
get_institution_count : () -> (nat64) query
search_institutions_by_name : (text) -> (vec Institution) query
```

#### Collection Queries
```candid
get_collection_metadata : (text) -> (opt CollectionMetadata) query
get_all_collections : () -> (vec CollectionMetadata) query
get_collections_by_institution : (text) -> (vec CollectionMetadata) query
get_collections_by_owner : (principal) -> (vec CollectionMetadata) query
get_collection_count : () -> (nat64) query
search_collections_by_name : (text) -> (vec CollectionMetadata) query
```

#### Document Queries
```candid
get_document_metadata : (text) -> (opt Document) query
get_document_file : (text) -> (opt blob) query
get_complete_document : (text) -> (opt record { Document; blob }) query
get_all_document_ids : () -> (vec text) query
get_documents_by_owner : (principal) -> (vec text) query
get_document_count : () -> (nat64) query
get_documents_by_collection : (text) -> (vec Document) query
get_documents_by_collection_category : (CollectionCategory) -> (vec Document) query
get_documents_by_institution : (text) -> (vec Document) query
get_documents_by_type : (text) -> (vec Document) query
get_documents_by_quarter_year : (nat8, nat16) -> (vec Document) query
search_documents_by_name : (text) -> (vec Document) query
```

## Data Structures

### Institution
```candid
type Institution = record {
    institution_id : text;
    owner : principal;
    name : text;
    email : text;
    created_at : nat64;
    collections : vec text;
};
```

### Collection Metadata
```candid
type CollectionMetadata = record {
    institution_id : text;
    collection_id : text;
    owner : principal;
    name : text;
    description : text;
    external_url : text;
    created_at : nat64;
    updated_at : nat64;
    category : CollectionCategory;
    documents : vec text;
};
```

### Collection Category
```candid
type CollectionCategory = variant {
    EarningRelease
};
```

## Usage Example

### Frontend Integration

```javascript
// Example of creating an institution
const createInstitution = async (institutionId, name, email) => {
    try {
        const result = await backend.create_institution(institutionId, name, email);
        if (result.Ok) {
            console.log('Institution created successfully');
        } else {
            console.error('Failed to create institution:', result.Err);
        }
    } catch (error) {
        console.error('Error:', error);
    }
};

// Example of uploading a document
const uploadDocument = async (file, metadata) => {
    const fileData = await file.arrayBuffer();
    const fileBytes = Array.from(new Uint8Array(fileData));
    
    const document = {
        institution_id: "INST001",
        collection_id: "COLL001",
        document_id: "",  // Will be set by the canister
        owner: "2vxsx-fae", // Anonymous principal
        name: file.name,
        company_name: "Example Corp",
        description: "Q1 2024 Earning Release",
        document_hash: "",  // Will be calculated by the canister
        document_data: {
            EarningRelease: {
                earning_release_id: "ER001",
                quarter: 1,
                year: 2024,
                consolidated_income_data: {
                    gross_profit: 1000000.0,
                    operating_profit: 800000.0,
                    ebitda: 900000.0,
                    profit_before_tax: 700000.0,
                    net_profit: 500000.0
                },
                consolidated_balance_sheet_data: {
                    total_assets: 5000000.0,
                    total_equity: 3000000.0,
                    total_liabilities: 2000000.0,
                    total_liabilities_and_equity: 5000000.0
                }
            }
        },
        file_size: file.size,
        file_type: file.type,
        file_data: fileBytes
    };

    try {
        const response = await backend.upload_file_and_publish_document(document);
        if (response.success) {
            console.log('Document uploaded successfully:', response.document_id);
        } else {
            console.error('Upload failed:', response.error_message);
        }
    } catch (error) {
        console.error('Error:', error);
    }
};
```

## Validation Rules

1. **File Size**: Maximum 10MB per file
2. **File Types**: Supports images (JPEG, PNG), documents (PDF, text), and Excel files (.xls, .xlsx, .xlsm, .xltm, .xlam, .xlsb)
3. **Institution ID**: 3-50 characters
4. **Institution Name**: 2-100 characters
5. **Collection ID**: 1-100 characters
6. **Collection Name**: 1-200 characters
7. **Email**: Must be valid email format

## Storage

The backend uses stable memory to store:
- Institution metadata and relationships
- Collection metadata and document lists
- Document metadata and file data
- Owner-to-document mappings


## Building and Deploying

```bash
# Build the canister
dfx build backend

# Deploy to local network
dfx deploy backend

# Deploy to mainnet
dfx deploy --network ic backend
``` 