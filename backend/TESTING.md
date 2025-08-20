# NFT Backend Testing Guide

This guide shows you how to test the NFT backend using the browser and Candid interface.

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
# Test metadata
dfx canister call backend icrc37_metadata
```

## 📋 Test Cases



### 2. ICRC-37 Metadata Test

**Method:** `icrc37_metadata`
**Input:** `()`
**Expected Output:**
```json
{
  "name": "Chain Notary NFTs",
  "symbol": "CNFT",
  "description": ["Chain Notary NFTs"],
  "logo": null,
  "url": null,
  "created_at": 1234567890,
  "updated_at": 1234567890
}
```

### 3. Mint NFT Test

**Method:** `icrc37_mint`
**Input:**
```json
{
  "token_ids": ["nft_test_001"],
  "metadata": {
    "name": "Test NFT",
    "description": "A test NFT",
    "image": "https://example.com/image.jpg",
    "external_url": null,
    "attributes": [
      {
        "trait_type": "Color",
        "value": "Blue",
        "display_type": null
      }
    ]
  }
}
```

### 4. File Upload Test

**Method:** `upload_file_and_publish_document`
**Input:**
```json
{
  "metadata": {
    "institution_id": null,
    "collection_id": null,
    "document_id": "",
    "owner": "2vxsx-fae",
    "name": "Test Excel Document",
    "description": "A test Excel spreadsheet for testing purposes",
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
}
```

### 5. Query Tests

**Method:** `get_nft_count`
**Input:** `()`
**Expected Output:** `0` (initially)

**Method:** `list_all_nfts`
**Input:** `()`
**Expected Output:** `[]` (initially)

## 🔧 Test Data Generator

### Generate Test Principal
```bash
# Generate a test principal
dfx identity get-principal
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

### Generate Test Excel Data
```javascript
// In browser console, generate a small test Excel file header
// This represents the first 20 bytes of an Excel .xlsx file (ZIP format)
const excelHeader = [80, 75, 3, 4, 20, 0, 6, 0, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0];
console.log(excelHeader); // Copy this array for Excel file testing
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
dfx canister call backend icrc37_metadata
```



## 📊 Expected Test Results



### After File Upload:
1. `get_nft_count()` should return `1`
2. `get_nft_file(token_id)` should return file data
3. `get_nfts_by_owner(owner)` should return token IDs

## 🔍 Debugging Tips

### Check Canister Logs
```bash
# Check canister status and logs
dfx canister status backend
```

### Verify Candid Interface
```bash
dfx canister call backend icrc37_metadata
```

### Test Error Handling
```bash

```

## 🎯 Success Criteria

✅ **All methods respond without errors**
✅ **NFTs can be minted and retrieved**
✅ **File uploads work correctly**
✅ **Metadata is stored and retrieved**
✅ **Error handling works properly**

## 📝 Test Checklist

- [ ] Deploy canister successfully
- [ ] Access Candid UI in browser

- [ ] Test ICRC-37 metadata

- [ ] Test file upload
- [ ] Test query methods
- [ ] Test error conditions
- [ ] Verify data persistence

## 🚨 Important Notes

1. **Local Network**: Tests run on local network (`dfx start`)
2. **Principal IDs**: Use `2vxsx-fae` for anonymous calls
3. **File Size**: Keep test files under 2MB
4. **Image Types**: Only JPEG, PNG, GIF, WebP supported
5. **Persistence**: Data persists between calls in local network 