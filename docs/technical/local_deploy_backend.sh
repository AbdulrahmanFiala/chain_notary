#!/bin/bash

set -e

# Set up identity to avoid passphrase prompts
echo "Setting up identity..."
chmod 600 ~/.config/dfx/identity/default/identity.pem
dfx identity use default

echo "Starting automated backend deployment..."

# Create canister if it doesn't exist
echo "Creating canister if needed..."
dfx canister create backend --network local || echo "Canister already exists"

# Build the backend first
echo "Building backend..."
dfx build backend --network local

# Generate Candid interface file
echo "Generating Candid interface..."
candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > ./backend/backend.did

# Deploy backend to local network
echo "Deploying backend to local network..."
dfx deploy backend --network local --yes

echo "Backend deployment completed successfully!"
echo "Your backend is now running."