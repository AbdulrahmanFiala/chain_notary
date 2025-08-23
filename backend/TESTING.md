# Chain Notary Backend Testing Guide

This guide shows you how to test the Chain Notary backend using the browser and Candid interface.

## 🚀 Quick Start

### 1. Build and Deploy

```bash
# Build the canister
dfx build backend

# Deploy to local network
dfx deploy backend

# Get the canister ID
dfx canister id backend
```

### 2. Access the Candid UI

Open your browser and navigate to:
```
http://localhost:8080/?canisterId=<CANISTER_ID>
```

Replace `<CANISTER_ID>` with your actual canister ID.

## 🧪 Testing Methods

### Method 1: Browser Candid UI (Recommended)

1. **Open the Candid UI** in your browser
2. **Test each method** using the interactive interface
3. **Copy/paste the test data** below

### Method 2: Command Line

```bash
# Test document count
dfx canister call backend get_document_count
```

## 📋 Test Cases

### 1. Institution Management Tests

#### Create Institution
**Method:** `create_institution`
**Input:**
```json
{
  "institution_id": "INST001",
  "name": "Test Financial Corp",
  "email": "test@financialcorp.com"
}
```
**Expected Output:** `{ "Ok" }`

#### Get Institution Metadata
**Method:** `get_institution_metadata`
**Input:** `"INST001"`
**Expected Output:** Institution record with the provided data

#### Get Institution Count
**Method:** `get_institution_count`
**Input:** `()`
**Expected Output:** `1` (after creating institution)

### 2. Collection Management Tests

#### Create Collection
**Method:** `create_collection`
**Input:**
```json
{
  "collection_id": "COLL001",
  "name": "Q1 2024 Earnings",
  "description": "First quarter earnings releases for 2024",
  "external_url": "https://example.com/earnings",
  "category": { "EarningRelease" },
  "institution_id": "INST001"
}
```
**Expected Output:** `{ "Ok" }`

#### Get Collection Metadata
**Method:** `get_collection_metadata`
**Input:** `"COLL001"`
**Expected Output:** Collection record with the provided data

#### Get Collections by Institution
**Method:** `get_collections_by_institution`
**Input:** `"INST001"`
**Expected Output:** Array containing the created collection

### 3. Document Upload Test

#### Upload Earning Release Document
**Method:** `upload_file_and_publish_document`
**Input:**
```json
{
  "institution_id": "INST001",
  "collection_id": "COLL001",
  "document_id": "",
  "owner": "2vxsx-fae",
  "name": "Q1 2024 Earning Release",
  "company_name": "Test Financial Corp",
  "description": "First quarter 2024 financial results",
  "document_hash": "calculated_hash_here",
  "document_data": {
    "EarningRelease": {
      "earning_release_id": "ER001",
      "quarter": 1,
      "year": 2024,
      "consolidated_income_data": {
        "gross_profit": 1000000.0,
        "operating_profit": 800000.0,
        "ebitda": 900000.0,
        "profit_before_tax": 700000.0,
        "net_profit": 500000.0
      },
      "consolidated_balance_sheet_data": {
        "total_assets": 5000000.0,
        "total_equity": 3000000.0,
        "total_liabilities": 2000000.0,
        "total_liabilities_and_equity": 5000000.0
      }
    }
  },
  "file_size": 20,
  "file_type": "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
  "file_data": [80, 75, 3, 4, 20, 0, 6, 0, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0]
}
```
**Expected Output:**
```json
{
  "success": true,
  "document_id": "generated_id_here",
  "error_message": "",
  "document_hash": "calculated_hash_here"
}
```

### 4. Query Tests

#### Document Count
**Method:** `get_document_count`
**Input:** `()`
**Expected Output:** `1` (after uploading document)

#### Get Document Metadata
**Method:** `get_document_metadata`
**Input:** `"generated_document_id"`
**Expected Output:** Document record without file data

#### Get Document File
**Method:** `get_document_file`
**Input:** `"generated_document_id"`
**Expected Output:** File data as blob

#### Get Complete Document
**Method:** `get_complete_document`
**Input:** `"generated_document_id"`
**Expected Output:** Tuple of document metadata and file data

#### Get Documents by Collection
**Method:** `get_documents_by_collection`
**Input:** `"COLL001"`
**Expected Output:** Array containing the uploaded document

#### Get Documents by Quarter/Year
**Method:** `get_documents_by_quarter_year`
**Input:** `(1, 2024)`
**Expected Output:** Array containing documents from Q1 2024

### 5. Search Tests

#### Search Documents by Name
**Method:** `search_documents_by_name`
**Input:** `"Q1 2024"`
**Expected Output:** Array containing documents with matching names

#### Search Collections by Name
**Method:** `search_collections_by_name`
**Input:** `"Earnings"`
**Expected Output:** Array containing collections with matching names

#### Search Institutions by Name
**Method:** `search_institutions_by_name`
**Input:** `"Financial"`
**Expected Output:** Array containing institutions with matching names

## 🔧 Test Data Generator

### Generate Test Principal
```bash
# Generate a test principal
dfx identity get-principal
```

### Generate Test Excel Data
```javascript
// In browser console, generate a small test Excel file header
// This represents the first 20 bytes of an Excel .xlsx file (ZIP format)
const excelHeader = [80, 75, 3, 4, 20, 0, 6, 0, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0];
console.log(excelHeader); // Copy this array for Excel file testing
```

### Generate Test Image Data
```javascript
// In browser console, generate a small test image
const canvas = document.createElement('canvas');
canvas.width = 10;
canvas.height = 10;
const ctx = canvas.getContext('2d');
ctx.fillStyle = 'red';
ctx.fillRect(0, 0, 10, 10);
const blob = await new Promise(resolve => canvas.toBlob(resolve, 'image/jpeg'));
const arrayBuffer = await blob.arrayBuffer();
const uint8Array = new Uint8Array(arrayBuffer);
console.log(Array.from(uint8Array)); // Copy this array
```

## 🐛 Common Issues & Solutions

### Issue 1: Canister Not Found
```bash
# Solution: Check if canister is deployed
dfx canister status backend
```

### Issue 2: Method Not Found
```bash
# Solution: Check candid interface
dfx canister call backend get_document_count
```

### Issue 3: Validation Errors
- **Institution ID**: Must be 3-50 characters
- **Collection ID**: Must be 1-100 characters
- **File Size**: Maximum 10MB
- **File Type**: Must be supported MIME type

## 📊 Expected Test Results

### After Institution Creation:
1. `get_institution_count()` should return `1`
2. `get_institution_metadata("INST001")` should return institution data

### After Collection Creation:
1. `get_collection_count()` should return `1`
2. `get_collections_by_institution("INST001")` should return collection data

### After Document Upload:
1. `get_document_count()` should return `1`
2. `get_document_file(document_id)` should return file data
3. `get_documents_by_owner(owner)` should return document IDs
4. `get_documents_by_collection("COLL001")` should return document data

## 🔍 Debugging Tips

### Check Canister Logs
```bash
# Check canister status and logs
dfx canister status backend
```

### Verify Candid Interface
```bash
# Check available methods
dfx canister call backend get_document_count
```

### Test Error Handling
```bash
# Try invalid inputs to test validation
dfx canister call backend create_institution '("", "Name", "email@test.com")'
```

## 🎯 Success Criteria

✅ **All methods respond without errors**
✅ **Institutions can be created and managed**
✅ **Collections can be created and managed**
✅ **Documents can be uploaded and retrieved**
✅ **Search and query functions work correctly**
✅ **Error handling works properly**
✅ **Data persistence between calls**

## 📝 Test Checklist

- [ ] Deploy canister successfully
- [ ] Access Candid UI in browser
- [ ] Test institution creation and management
- [ ] Test collection creation and management
- [ ] Test document upload and retrieval
- [ ] Test query methods
- [ ] Test search functionality
- [ ] Test error conditions
- [ ] Verify data persistence

## 🚨 Important Notes

1. **Local Network**: Tests run on local network (`dfx start`)
2. **Principal IDs**: Use `2vxsx-fae` for anonymous calls
3. **File Size**: Keep test files under 10MB
4. **File Types**: Supports images (JPEG, PNG), documents (PDF, text), and Excel files
5. **Persistence**: Data persists between calls in local network
6. **Validation**: All inputs are validated for format and length
7. **Ownership**: Only owners can modify their institutions, collections, and documents 