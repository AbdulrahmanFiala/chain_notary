#!/bin/bash

echo "🧪 Testing Collection Management in Chain Notary Backend"
echo "======================================================"

# Get the canister ID
CANISTER_ID=$(dfx canister id backend)
echo "📦 Backend Canister ID: $CANISTER_ID"

echo ""
echo "1️⃣ Testing Collection Creation..."
echo "--------------------------------"

# Test creating a collection
echo "Creating 'graduation_certificates' collection..."
dfx canister call backend create_collection '(
    "graduation_certificates",
    "University Graduation Certificates",
    opt "Collection of university graduation certificates",
    opt "https://example.com/graduation.jpg",
    opt "https://example.com/graduation",
    opt variant { UniversityGraduationCertificate },
    null
)'

echo ""
echo "2️⃣ Testing Collection Listing..."
echo "--------------------------------"

# List all collections
echo "Listing all collections..."
dfx canister call backend list_all_collections

echo ""
echo "3️⃣ Testing Collection Retrieval..."
echo "----------------------------------"

# Get specific collection
echo "Getting 'graduation_certificates' collection..."
dfx canister call backend get_collection '("graduation_certificates")'

echo ""
echo "4️⃣ Testing Collection Count..."
echo "-------------------------------"

# Get collection count
echo "Getting total collection count..."
dfx canister call backend get_collection_count

echo ""
echo "5️⃣ Testing Document Creation with Collection..."
echo "-----------------------------------------------"

# Create a document that references the collection
echo "Creating a document in the collection..."
dfx canister call backend upload_file_and_publish_document '(
    vec { 255, 216, 255, 224, 0, 16, 74, 70, 73, 70, 0, 1, 1, 1, 0, 72, 0, 72, 0, 0 },
    "image/jpeg",
    record {
        collection_id = "graduation_certificates";
        document_id = "";
        owner = principal "2vxsx-fae";
        name = "John Doe Graduation Certificate";
        description = opt "Bachelor of Science in Computer Science";
        image_url = opt "https://example.com/certificate.jpg";
        document_hash = "";
        file_size = 0;
        file_type = "";
        file_data = null;
        
        recipient = opt record {
            name = "John Doe";
            id = opt "12345";
            email = opt "john.doe@university.edu"
        }
    }
)'

echo ""
echo "6️⃣ Testing Collection with Documents..."
echo "---------------------------------------"

# Get collection again to see if document was added
echo "Getting updated collection..."
dfx canister call backend get_collection '("graduation_certificates")'

echo ""
echo "7️⃣ Testing Collection Update..."
echo "--------------------------------"

# Update collection description
echo "Updating collection description..."
dfx canister call backend update_collection '(
    "graduation_certificates",
    null,
    opt "Updated: Collection of university graduation certificates and diplomas",
    null,
    null,
    null
)'

echo ""
echo "8️⃣ Testing Collection After Update..."
echo "-------------------------------------"

# Get updated collection
echo "Getting updated collection..."
dfx canister call backend get_collection '("graduation_certificates")'

echo ""
echo "✅ Collection Management Test Complete!"
echo "======================================"
echo ""
echo "📋 Summary of what was tested:"
echo "   • Collection creation"
echo "   • Collection listing"
echo "   • Collection retrieval"
echo "   • Document creation with collection reference"
echo "   • Collection update"
echo "   • Collection-document relationship"
echo ""
echo "🌐 You can also test these functions in the Candid UI:"
echo "   http://localhost:8080/?canisterId=$CANISTER_ID"
