# Institution Management System

## Overview

The institution management system provides a hierarchical structure for organizing educational and organizational entities:

**Institution → Collections → Documents**

This creates a proper organizational hierarchy where:
- **Institutions** represent universities, schools, or organizations
- **Collections** belong to institutions and group related documents
- **Documents** belong to collections and represent individual certificates/credentials

## Architecture

### Storage Structure
- **INSTITUTIONS**: `MemoryId::new(2)` - Stores institution metadata
- **COLLECTIONS**: `MemoryId::new(1)` - Stores collection metadata with institution links
- **DOCUMENTS**: `MemoryId::new(0)` - Stores document data with collection links
- **OWNER_TOKENS**: `MemoryId::new(3)` - Maps owners to their documents

### Data Relationships
```
Institution
├── institution_id (unique identifier)
├── owner (Principal who controls the institution)
├── name (institution name)
├── email (contact email)
├── created_at (timestamp)
└── collections (Vec<String> of collection IDs)

Collection
├── institution_id (links to institution)
├── collection_id (unique identifier)
├── owner (Principal who controls the collection)
├── name, description, image_url, external_url
├── category (CollectionCategory enum)
├── created_at, updated_at
└── documents (Vec<String> of document IDs)

Document
├── collection_id (links to collection)
├── document_id (unique identifier)
├── owner (Principal who owns the document)
├── name, description, image_url
├── document_hash, file_size, file_type
├── file_data (optional binary data)
├── minted_at (timestamp)
└── recipient (optional recipient info)
```

## Functions

### Institution Management

#### `create_institution(institution_id, name, email)`
- Creates a new institution
- Validates input lengths (ID: 3-50 chars, name: 2-100 chars, email: 5-100 chars)
- Sets caller as owner
- Returns error if institution ID already exists

#### `update_institution(institution_id, name, email)`
- Updates institution metadata (only owner can update)
- Optional parameters for partial updates
- Validates input lengths
- Returns error if caller is not owner

#### `delete_institution(institution_id)`
- Deletes institution (only owner can delete)
- Prevents deletion if institution has collections
- Returns error if caller is not owner or collections exist

#### `get_institution(institution_id)`
- Retrieves institution metadata by ID
- Returns `Option<Institution>`

#### `list_all_institutions()`
- Lists all institutions in the system
- Returns `Vec<Institution>`

#### `get_institutions_by_owner(owner)`
- Lists institutions owned by a specific principal
- Returns `Vec<Institution>`

#### `get_institution_count()`
- Returns total number of institutions
- Returns `u64`

### Collection-Institution Linking

#### `add_collection_to_institution(institution_id, collection_id)`
- Links an existing collection to an institution
- Only institution owner can perform this operation
- Updates both institution and collection records
- Returns error if collection doesn't exist or is already linked

#### `remove_collection_from_institution(institution_id, collection_id)`
- Removes collection link from institution
- Only institution owner can perform this operation
- Updates both institution and collection records
- Returns error if collection is not linked to institution

#### `get_collections_by_institution(institution_id)`
- Retrieves all collections belonging to an institution
- Returns `Vec<CollectionMetadata>`

### Enhanced Collection Creation

#### `create_collection(..., institution_id)`
- Now accepts optional `institution_id` parameter
- Validates institution exists if provided
- Automatically links collection to institution
- Updates institution's collections list

## Workflow Examples

### 1. Complete Institution Setup
```bash
# 1. Create institution
dfx canister call backend create_institution '(
    "cairo_university",
    "Cairo University", 
    "admin@cairo.edu"
)'

# 2. Create collection linked to institution
dfx canister call backend create_collection '(
    "graduation_certs",
    "Graduation Certificates",
    opt "Official graduation certificates",
    null, null, null,
    opt "cairo_university"
)'

# 3. Upload document to collection
dfx canister call backend upload_file_and_create_document '(
    [file_data],
    "application/pdf",
    {
        collection_id: "graduation_certs",
        document_id: "",
        owner: caller,
        name: "John Doe Certificate",
        description: null,
        image_url: null,
        document_hash: "",
        file_size: 0,
        file_type: "",
        file_data: null,
        minted_at: 0,
        recipient: null
    }
)'
```

### 2. Institution Management
```bash
# List all institutions
dfx canister call backend list_all_institutions

# Get specific institution
dfx canister call backend get_institution '("cairo_university")'

# Update institution
dfx canister call backend update_institution '(
    "cairo_university",
    opt "Cairo University - Faculty of Engineering",
    null
)'

# Get collections by institution
dfx canister call backend get_collections_by_institution '("cairo_university")'
```

## Validation Rules

### Institution Validation
- **ID**: 3-50 characters, must be unique
- **Name**: 2-100 characters
- **Email**: 5-100 characters
- **Ownership**: Only owner can update/delete

### Collection Validation
- **ID**: 1-100 characters, must be unique
- **Name**: 1-200 characters
- **Institution**: If provided, must exist
- **Ownership**: Only owner can update/delete

### Relationship Validation
- **Institution → Collections**: One-to-many
- **Collection → Documents**: One-to-many
- **Deletion Protection**: Cannot delete institution with collections
- **Automatic Linking**: Collections automatically update institution records

## Benefits

1. **Organizational Structure**: Clear hierarchy for educational institutions
2. **Ownership Control**: Proper access control at each level
3. **Data Integrity**: Validation prevents orphaned records
4. **Scalability**: Easy to add new institutions and collections
5. **Query Efficiency**: Direct access to institution-collection relationships
6. **Flexibility**: Collections can exist without institutions if needed

## Migration Notes

- Existing collections will have empty `institution_id` by default
- Collections can be linked to institutions after creation using `add_collection_to_institution`
- No data loss during migration
- Backward compatible with existing document uploads

## Testing

Use the provided test script:
```bash
./backend/test_institutions.sh
```

This script tests:
- Institution creation and management
- Collection linking to institutions
- Query operations
- Update operations
- Relationship validation
