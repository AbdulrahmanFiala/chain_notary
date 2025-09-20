#!/bin/bash


set -e

# Set up identity to avoid passphrase prompts
echo "Setting up identity..."
chmod 600 ~/.config/dfx/identity/default/identity.pem
dfx identity use default

echo "Starting automated deployment..."

# Create canisters if they don't exist
echo "Creating canisters if needed..."
dfx canister create --all --network local || echo "Canisters already exist"

# Get the current dfx identity (principal ID)
echo "Getting principal ID..."
PRINCIPAL_ID=$(dfx identity get-principal)

if [ -z "$PRINCIPAL_ID" ]; then
    echo "Error: Could not get principal ID. Make sure dfx is installed and you are logged in."
    exit 1
fi

echo "Principal ID: $PRINCIPAL_ID"

# Export the environment variable
export VITE_PRINCIPAL_ID="$PRINCIPAL_ID"
echo "Set VITE_PRINCIPAL_ID=$VITE_PRINCIPAL_ID"

# Build the project first
echo "Building project..."
dfx build --network local

# Generate Candid interface file (for local development)
echo "Generating Candid interface..."
candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > ./backend/backend.did

# Deploy the project
echo "Deploying project..."
dfx deploy --network local --yes

echo "Deployment completed successfully!"
echo "Your application is now running."
