# Query Module Reorganization

## Overview
The query functions have been reorganized from a single large `query.rs` file into specialized modules for better organization, maintainability, and separation of concerns.

## New Module Structure

### 1. `document_queries.rs`
Contains all document-related query functions:
- `get_document_metadata()` - Get document metadata by ID
- `get_document_file()` - Get document file data by ID
- `get_complete_document()` - Get complete document (metadata + file)
- `list_all_documents()` - List all document IDs
- `get_documents_by_owner()` - Get documents owned by a principal
- `get_document_count()` - Get total document count
- `get_documents_by_collection()` - Get documents by collection ID
- `get_documents_by_collection_category()` - Get documents by category
- `get_documents_by_recipient_email()` - Get documents by recipient email
- `get_documents_by_recipient_id()` - Get documents by recipient ID
- `get_documents_by_institution()` - Get documents by institution

### 2. `collection_queries.rs`
Contains all collection-related query functions:
- `get_collection_metadata()` - Get collection metadata by ID
- `get_all_collection_ids()` - Get all collection IDs
- `get_all_collections()` - Get all collections with full metadata
- `get_collections_by_owner()` - Get collections by owner
- `get_collections_by_institution()` - Get collections by institution
- `get_collection_count()` - Get total collection count

### 3. `institution_queries.rs`
Contains all institution-related query functions:
- `get_institution_metadata()` - Get institution metadata by ID
- `get_all_institutions()` - Get all institutions with full metadata
- `get_institutions_by_owner()` - Get institutions by owner
- `get_institution_count()` - Get total institution count

### 4. `search_queries.rs`
Contains all search and discovery functions:
- `search_documents_by_name()` - Search documents by name
- `search_collections_by_name()` - Search collections by name
- `search_institutions_by_name()` - Search institutions by name

### 5. `query.rs` (Coordinator Module)
The main `query.rs` file now serves as a coordinator that:
- Re-exports all functions from the specialized modules
- Maintains the same public API for backward compatibility
- Provides clear documentation about the module organization

## Benefits of This Organization

1. **Better Separation of Concerns**: Each module focuses on a specific domain
2. **Easier Maintenance**: Developers can work on specific query types without affecting others
3. **Improved Testing**: Each module can be tested independently
4. **Clearer Code Structure**: Related functions are grouped together
5. **Reduced File Size**: Each file is more manageable and focused
6. **Better Code Navigation**: Easier to find specific query functions

## Usage

The public API remains exactly the same. All existing code will continue to work without changes:

```rust
// These calls work exactly as before
use crate::functions::query::*;

let doc = get_document_metadata("doc123".to_string());
let collections = get_collections_by_owner(owner);
let institutions = search_institutions_by_name("Bank".to_string());
```

## Module Dependencies

- `document_queries.rs` depends on `collection_queries.rs` for cross-references
- All modules are independent and can be modified without affecting others
- The coordinator module (`query.rs`) handles all re-exports

## Future Improvements

With this new structure, it's easier to:
- Add new query types to appropriate modules
- Implement caching strategies per domain
- Add pagination support to specific query types
- Optimize storage access patterns per module
- Add domain-specific error handling
