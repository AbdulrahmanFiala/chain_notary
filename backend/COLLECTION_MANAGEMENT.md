# Collection Management System

## Overview

The Chain Notary backend now properly implements a **one-to-many relationship** between collections and documents. Collections are created separately with their own metadata, and documents reference the collections they belong to.

## Architecture

### 1. **Collections as First-Class Entities**
- Collections are stored separately in `COLLECTIONS` storage
- Each collection has its own metadata and document list
- Collections can exist without documents

### 2. **Document-Collection Relationship**
- Documents reference collections via `collection_id` field
- Documents can exist without belonging to a collection
- When a document is added to a collection, it's automatically added to the collection's `documents` list

## Storage Structure

```rust
// Collections stored separately
pub static COLLECTIONS: RefCell<StableBTreeMap<String, Vec<u8>, Memory>>

// Documents with collection references
pub struct Document {
    pub collection_id: String,  // References a collection
    // ... other fields
}
```

## Collection Management Functions

### **Create Collection**
```rust
create_collection(
    collection_id: String,
    name: String,
    description: Option<String>,
    image_url: Option<String>,
    external_url: Option<String>,
    category: Option<CollectionCategory>,
) -> Result<(), String>
```

### **Update Collection**
```rust
update_collection(
    collection_id: String,
    name: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    external_url: Option<String>,
    category: Option<CollectionCategory>,
) -> Result<(), String>
```

### **Delete Collection**
```rust
delete_collection(collection_id: String) -> Result<(), String>
```
*Note: Can only delete collections with no documents*

### **Add Document to Collection**
```rust
add_document_to_collection(collection_id: String, document_id: String) -> Result<(), String>
```

### **Remove Document from Collection**
```rust
remove_document_from_collection(collection_id: String, document_id: String) -> Result<(), String>
```

## Query Functions

### **Collection Queries**
- `get_collection(collection_id)` - Get specific collection
- `list_all_collections()` - List all collections
- `get_collections_by_owner(owner)` - Get collections by owner
- `get_collection_count()` - Get total collection count

### **Document Queries**
- `get_documents_by_collection(collection_id)` - Get documents in a collection
- `get_documents_by_category(category)` - Get documents by collection category

## Workflow Examples

### 1. **Creating a Collection and Adding Documents**

```bash
# 1. Create collection
dfx canister call backend create_collection '(
    "graduation_certs",
    "University Graduation Certificates",
    opt "Collection of university diplomas",
    null,
    null,
    opt variant { UniversityGraduationCertificate }
)'

# 2. Upload document to collection
dfx canister call backend upload_file_and_create_nft '(
    file_data,
    "application/pdf",
    document_metadata_with_collection_id
)'
```

### 2. **Managing Collection Membership**

```bash
# Add existing document to collection
dfx canister call backend add_document_to_collection '("graduation_certs", "doc_123")'

# Remove document from collection
dfx canister call backend remove_document_from_collection '("graduation_certs", "doc_123")'
```

## Validation Rules

### **Collection Creation**
- Collection ID must be 1-100 characters
- Collection name must be 1-200 characters
- Collection ID must be unique

### **Document-Collection Relationship**
- Document can only reference existing collections
- Collection must exist before document can reference it
- Document can be removed from collection without deleting the document

### **Collection Deletion**
- Collection can only be deleted if it has no documents
- Documents are not automatically deleted when collection is deleted

## Benefits of New System

1. **Proper Separation of Concerns**: Collections and documents are separate entities
2. **Flexible Relationships**: Documents can exist without collections, collections without documents
3. **Efficient Queries**: Direct collection access instead of building from documents
4. **Better Metadata Management**: Collection-level metadata independent of documents
5. **Scalability**: Collections can handle large numbers of documents efficiently

## Migration from Old System

The old system automatically created collections when documents referenced them. The new system requires:

1. **Explicit collection creation** before referencing in documents
2. **Collection validation** when creating documents
3. **Proper collection management** for adding/removing documents

## Testing

Use the provided test script to verify functionality:

```bash
./test_collections.sh
```

This will test:
- Collection creation, retrieval, and updates
- Document creation with collection references
- Collection-document relationship management
- Error handling for invalid operations
