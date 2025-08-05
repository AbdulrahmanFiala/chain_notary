#!/bin/bash

# NFT Backend Test Script
# This script helps test the backend functionality

echo "🚀 NFT Backend Test Script"
echo "=========================="

# Check if dfx is running
if ! dfx ping; then
    echo "❌ dfx is not running. Please start it with: dfx start"
    exit 1
fi

# Get canister ID
CANISTER_ID=$(dfx canister id backend)
echo "📦 Canister ID: $CANISTER_ID"

echo ""
echo "🧪 Running Tests..."
echo "==================="

# Test 1: Greeting
echo "1️⃣ Testing greeting function..."
GREETING_RESULT=$(dfx canister call backend greet '("World")')
echo "   Result: $GREETING_RESULT"

# Test 2: Metadata
echo ""
echo "2️⃣ Testing ICRC-37 metadata..."
METADATA_RESULT=$(dfx canister call backend icrc37_metadata)
echo "   Result: $METADATA_RESULT"

# Test 3: Mint NFT
echo ""
echo "3️⃣ Testing NFT minting..."
MINT_RESULT=$(dfx canister call backend icrc37_mint '(record { token_ids = vec {"test_001"}; metadata = null })')
echo "   Result: $MINT_RESULT"

# Test 4: Get NFT count
echo ""
echo "4️⃣ Testing NFT count..."
COUNT_RESULT=$(dfx canister call backend get_nft_count)
echo "   Result: $COUNT_RESULT"

# Test 5: List all NFTs
echo ""
echo "5️⃣ Testing list all NFTs..."
LIST_RESULT=$(dfx canister call backend list_all_nfts)
echo "   Result: $LIST_RESULT"

# Test 6: Get NFT metadata
echo ""
echo "6️⃣ Testing get NFT metadata..."
METADATA_RESULT=$(dfx canister call backend get_nft_metadata '("test_001")')
echo "   Result: $METADATA_RESULT"

echo ""
echo "✅ Tests completed!"
echo ""
echo "🌐 To access the Candid UI, open:"
echo "   http://localhost:8080/?canisterId=$CANISTER_ID"
echo ""
echo "📋 Manual Test Commands:"
echo "   dfx canister call backend greet '(\"Your Name\")'"
echo "   dfx canister call backend icrc37_metadata"
echo "   dfx canister call backend get_nft_count"
echo "   dfx canister call backend list_all_nfts" 