#!/bin/bash

echo "🚀 Document Management Test Script"
echo "=================================="

# Check if dfx is running
if ! dfx ping; then
    echo "❌ dfx is not running. Please start it with: dfx start"
    exit 1
fi

# Get canister ID
CANISTER_ID=$(dfx canister id backend)
echo "📦 Canister ID: $CANISTER_ID"

echo ""
echo "🧪 Running Document Tests..."
echo "============================"

# Test 1: Create a test document
echo ""
echo "1️⃣ Testing document creation..."
echo "Creating test document..."
DOC_RESULT=$(dfx canister call backend upload_file_and_publish_document '(
    vec { 255, 216, 255, 224, 0, 16, 74, 70, 73, 70, 0, 1, 1, 1, 0, 72, 0, 72, 0, 0 },
    "image/jpeg",
    record {
        collection_id = "";
        document_id = "";
        owner = principal "2vxsx-fae";
        name = "Test Document";
        description = opt "A test document for testing purposes";
        image_url = opt "https://example.com/test.jpg";
        document_hash = "";
        file_size = 0;
        file_type = "";
        file_data = null;
        recipient = opt record {
            name = "Test User";
            id = opt "test_001";
            email = opt "test@example.com"
        }
    }
)')
echo "   Result: $DOC_RESULT"

# Extract document ID from result for further testing
DOC_ID=$(echo "$DOC_RESULT" | grep -o 'document_id = opt ".*"' | cut -d'"' -f2)
if [ -n "$DOC_ID" ]; then
    echo "   Document ID: $DOC_ID"
    
    # Test 2: Get document metadata
    echo ""
    echo "2️⃣ Testing get document metadata..."
    METADATA_RESULT=$(dfx canister call backend get_document_metadata "(\"$DOC_ID\")")
    echo "   Result: $METADATA_RESULT"
    
    # Test 3: Get document file
    echo ""
    echo "3️⃣ Testing get document file..."
    FILE_RESULT=$(dfx canister call backend get_document_file "(\"$DOC_ID\")")
    echo "   Result: $FILE_RESULT"
    
    # Test 4: Get complete document
    echo ""
    echo "4️⃣ Testing get complete document..."
    COMPLETE_RESULT=$(dfx canister call backend get_complete_document "(\"$DOC_ID\")")
    echo "   Result: $COMPLETE_RESULT"
else
    echo "   ❌ Could not extract document ID from result"
fi

# Test 5: Get document count
echo ""
echo "5️⃣ Testing document count..."
COUNT_RESULT=$(dfx canister call backend get_document_count)
echo "   Result: $COUNT_RESULT"

# Test 6: List all documents
echo ""
echo "6️⃣ Testing list all documents..."
LIST_RESULT=$(dfx canister call backend list_all_documents)
echo "   Result: $LIST_RESULT"

# Test 7: Get documents by owner
echo ""
echo "7️⃣ Testing get documents by owner..."
OWNER_RESULT=$(dfx canister call backend get_documents_by_owner '(principal "2vxsx-fae")')
echo "   Result: $OWNER_RESULT"

echo ""
echo "✅ Document tests completed!"
echo ""
echo "🌐 To access the Candid UI, open:"
echo "   http://localhost:8080/?canisterId=$CANISTER_ID"
echo ""
echo "📋 Manual Test Commands:"
echo "   dfx canister call backend get_document_count"
echo "   dfx canister call backend list_all_documents"
echo "   dfx canister call backend get_document_metadata '(\"document_id_here\")'" 