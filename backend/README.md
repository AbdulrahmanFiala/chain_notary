# Chain Notary Backend API

This backend provides a comprehensive document management system for the Internet Computer (IC) blockchain. It allows institutions to create collections and publish documents with structured metadata, with specialized support for financial documents like earning releases and advanced analytics capabilities.

## 🚀 Quick Start

```bash
# Build and deploy
dfx build backend
dfx deploy backend

# Access Candid UI
# Open: http://localhost:8080/?canisterId=<CANISTER_ID>
```

## 📋 Overview

The system follows a hierarchical structure: **Institution → Collections → Documents**

- **Institutions** represent universities, schools, or organizations
- **Collections** belong to institutions and group related documents  
- **Documents** belong to collections and represent individual certificates/credentials

**Auto-Generated IDs**: All entities (institutions, collections, documents) now have automatically generated unique identifiers for seamless operation.

## ✨ Features

- **Institution Management**: Create and manage financial institutions
- **Collection Management**: Organize documents into themed collections
- **Document Publishing**: Upload and publish documents with structured metadata
- **Financial Data Support**: Specialized support for earning release documents with structured financial data
- **Advanced Analytics**: AI-powered document analysis using Gemini API for PDF and financial data
- **Search & Query**: Comprehensive querying across institutions, collections, and documents
- **Memory Management**: Efficient stable memory storage with separate storage areas for different data types

## 🔧 API Endpoints

### **Core Management Functions**

#### Institution Management
```candid
create_institution : (text, text) -> (Result text)
update_institution : (text, text, text) -> (Result)
delete_institution : (text) -> (Result)
add_collection_to_institution : (text, text) -> (Result)
remove_collection_from_institution : (text, text) -> (Result)
```

**Parameters for create_institution:**
- `name : text` - Institution name (2-100 characters)
- `email : text` - Institution email (5-100 characters)

**Returns:** Generated institution ID (e.g., "INST_1234567890")

#### Collection Management
```candid
create_collection : (text, text, text, CollectionCategory, text) -> (Result text)
update_collection : (text, text, text, text, CollectionCategory) -> (Result)
delete_collection : (text) -> (Result)
add_document_to_collection : (text, text) -> (Result)
remove_document_from_collection : (text, text) -> (Result)
```

**Parameters for create_collection:**
- `name : text` - Collection name (1-200 characters)
- `description : text` - Collection description
- `external_url : text` - External URL for the collection
- `category : CollectionCategory` - Collection category (currently supports EarningRelease)
- `institution_id : text` - Institution ID (can be empty for standalone collections, must exist if provided)

**Returns:** Generated collection ID (e.g., "COLL_1234567890")

#### Document Management
```candid
upload_file_and_publish_document : (Document) -> (DocumentResponse)
```
**Note:** This function automatically generates a unique document ID and calculates the file hash for integrity verification.

**Document Structure:**
```candid
type Document = record {
    institution_id : text;       // Institution ID (can be empty, must exist if provided)
    collection_id : text;        // Collection ID (can be empty, must exist if provided)
    document_id : text;          // Will be set by the canister
    owner : principal;           // Owner's principal ID
    name : text;                 // Document name
    company_name : text;         // Company name
    description : text;          // Document description
    document_hash : text;        // Will be calculated by the canister
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

**Supported File Types:**
- **Images**: JPEG, PNG
- **Documents**: PDF, text files
- **Excel Files**: .xls, .xlsx, .xlsm, .xltm, .xlam, .xlsb

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

### **Query & Search Functions**

#### Data Retrieval
```candid
# Institution Queries
get_institution_metadata : (text) -> (opt Institution) query
get_all_institutions : () -> (vec Institution) query
get_institutions_by_owner : (principal) -> (vec Institution) query
get_institution_count : () -> (nat64) query

# Collection Queries
get_collection_metadata : (text) -> (opt CollectionMetadata) query
get_all_collections : () -> (vec CollectionMetadata) query
get_collections_by_institution : (text) -> (vec CollectionMetadata) query
get_collections_by_owner : (principal) -> (vec CollectionMetadata) query
get_collection_count : () -> (nat64) query

# Document Queries
get_document_metadata : (text) -> (opt Document) query
get_document_file : (text) -> (opt blob) query
get_all_document_ids : () -> (vec text) query
get_documents_by_owner : (principal) -> (vec text) query
get_document_count : () -> (nat64) query
get_documents_by_collection : (text) -> (vec Document) query
get_documents_by_collection_category : (CollectionCategory) -> (vec Document) query
get_documents_by_institution : (text) -> (vec Document) query
get_documents_by_type : (text) -> (vec Document) query
get_documents_by_quarter_year : (nat8, nat16) -> (vec Document) query
```

#### Search Functions
```candid
search_institutions_by_name : (text) -> (vec Institution) query
search_collections_by_name : (text) -> (vec CollectionMetadata) query
search_documents_by_name : (text) -> (vec Document) query
```

### Analytics Endpoints

#### Document Analysis
```candid
analyze_document_data : (AnalyticsRequest) -> (AnalyticsResponse)
```

**AnalyticsRequest Structure:**
```candid
type AnalyticsRequest = record {
    document_id : opt text;        // Document ID to analyze (optional)
    input_data : opt text;         // Raw data to analyze (optional)
    analysis_focus : text;         // Analysis focus area (e.g., "financial_summary", "risk_assessment")
};
```

**AnalyticsResponse Structure:**
```candid
type AnalyticsResponse = record {
    success : bool;                // Operation success status
    analysis : text;               // AI-generated analysis content
    error_message : text;          // Error message if failed
    analysis_type : text;          // Type of analysis performed
};
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
const createInstitution = async (name, email) => {
    try {
        const result = await backend.create_institution(name, email);
        if (result.Ok) {
            console.log('Institution created successfully with ID:', result.Ok);
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
        institution_id: "INST_1234567890",  // Use generated institution ID
        collection_id: "COLL_1234567890",   // Use generated collection ID
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

## 📋 Data Structures & Validation

### **Validation Rules**

#### File Validation
- **File Size**: Maximum 10MB per file
- **File Types**: 
  - **Images**: JPEG, PNG
  - **Documents**: PDF, text files
  - **Excel Files**: .xls, .xlsx, .xlsm, .xltm, .xlam, .xlsb

#### Data Validation
- **Institution ID**: Auto-generated (e.g., "INST_1234567890"), must be unique
- **Institution Name**: 2-100 characters
- **Collection ID**: Auto-generated (e.g., "COLL_1234567890"), must be unique
- **Collection Name**: 1-200 characters
- **Email**: Must be valid email format (5-100 characters)

#### Relationship Validation
- **Institution → Collection**: Collection can only reference existing institutions
- **Collection → Document**: Document can only reference existing collections
- **Ownership**: Only owners can update/delete their resources
- **Deletion Protection**: Cannot delete institutions with collections or collections with documents

## Storage & Architecture

### **Memory Management System**

The backend uses **stable memory** with separate storage areas for optimal performance:

- **`MemoryId::new(0)` - DOCUMENTS**: Complete documents (metadata + file data) in single storage
- **`MemoryId::new(1)` - COLLECTIONS**: Collection metadata and document lists
- **`MemoryId::new(2)` - INSTITUTIONS**: Institution metadata and collection relationships
- **`MemoryId::new(3)` - OWNER_TOKENS**: Owner-to-document mappings for efficient queries

### **System Architecture**

The system follows a hierarchical structure: **Institution → Collections → Documents**

- **Institutions** represent universities, schools, or organizations
- **Collections** belong to institutions and group related documents  
- **Documents** belong to collections and represent individual certificates/credentials

### **Key Relationships**
- **Institution → Collections**: One-to-many relationship
- **Collection → Documents**: One-to-many relationship
- **Ownership**: Each level has proper access control
- **Validation**: Prevents orphaned records and maintains data integrity

### **Storage Benefits**
- **Efficient Queries**: Separate storage areas for different data types
- **Memory Optimization**: Stable memory prevents data loss during upgrades
- **Scalability**: Each storage area can be optimized independently
- **Data Integrity**: Automatic validation and relationship management


## Query System Organization

The query functions are organized into specialized modules for better maintainability:
- **`document_queries.rs`** - Document-related queries
- **`collection_queries.rs`** - Collection-related queries  
- **`institution_queries.rs`** - Institution-related queries
- **`search_queries.rs`** - Search and discovery functions

All functions maintain the same public API for backward compatibility.

## Building and Deploying

For detailed build and deployment instructions, see the main [BUILD.md](../BUILD.md) file.

**Quick Commands:**
```bash
# Build the canister
dfx build backend

# Deploy to local network
dfx deploy backend

# Deploy to mainnet
dfx deploy --network ic backend
``` 