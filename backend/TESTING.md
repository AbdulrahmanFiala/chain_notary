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

**Method:** `upload_file_and_create_nft`
**Input:**
```json
{
  "file_data": [255, 216, 255, 224, 0, 16, 74, 70, 73, 70, 0, 1, 1, 1, 0, 72, 0, 72, 0, 0],
  "file_name": "test.jpg",
  "file_type": "image/jpeg",
  "metadata": {
    "name": "Uploaded NFT",
    "description": "NFT created via file upload",
    "image_url": "https://example.com/image.jpg",
    "external_url": null,
    "attributes": [
      {
        "trait_type": "Type",
        "value": "Uploaded",
        "display_type": null
      }
    ],
    "properties": null
  },
  "owner": "2vxsx-fae"
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

### Issue 3: Invalid Principal
```bash
# Solution: Use anonymous principal
dfx canister call backend icrc37_mint '(record { token_ids = vec {"test_001"}; metadata = null })'
```

## 📊 Expected Test Results

### After Minting NFT:
1. `get_nft_count()` should return `1`
2. `list_all_nfts()` should return `["test_001"]`
3. `get_nft_metadata("test_001")` should return NFT info

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
# Try to mint same token twice
dfx canister call backend icrc37_mint '(record { token_ids = vec {"duplicate"}; metadata = null })'
dfx canister call backend icrc37_mint '(record { token_ids = vec {"duplicate"}; metadata = null })'
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
- [ ] Test minting NFT
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