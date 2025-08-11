#!/bin/bash

echo "🧪 Testing Institution Management System"
echo "========================================"

# Get the canister ID
CANISTER_ID=$(dfx canister id backend)

echo "📋 Creating a test institution..."
dfx canister call backend create_institution '(
    "cairo_university",
    "Cairo University",
    "admin@cairo.edu"
)'

echo ""
echo "📋 Creating another institution..."
dfx canister call backend create_institution '(
    "alexandria_university", 
    "Alexandria University",
    "admin@alexandria.edu"
)'

echo ""
echo "📋 Listing all institutions..."
dfx canister call backend list_all_institutions

echo ""
echo "📋 Getting specific institution..."
dfx canister call backend get_institution '("cairo_university")'

echo ""
echo "📋 Getting institution count..."
dfx canister call backend get_institution_count

echo ""
echo "📋 Creating a collection linked to Cairo University..."
dfx canister call backend create_collection '(
    "cairo_graduation_certs",
    "Cairo University Graduation Certificates",
    opt "Official graduation certificates from Cairo University",
    opt "https://example.com/cairo.jpg",
    opt "https://example.com/cairo",
    opt variant { UniversityGraduationCertificate },
    opt "cairo_university"
)'

echo ""
echo "📋 Getting collections by institution..."
dfx canister call backend get_collections_by_institution '("cairo_university")'

echo ""
echo "📋 Updating institution name..."
dfx canister call backend update_institution '(
    "cairo_university",
    opt "Cairo University - Faculty of Engineering",
    null
)'

echo ""
echo "📋 Getting updated institution..."
dfx canister call backend get_institution '("cairo_university")'

echo ""
echo "✅ Institution management tests completed!"
