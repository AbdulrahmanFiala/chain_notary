#!/bin/bash

set -e

# Get the script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
ENV_FILE="$PROJECT_ROOT/.env"

# Load environment variables from .env file
if [ -f "$ENV_FILE" ]; then
    echo "Loading environment variables from .env file..."
    export $(grep -v '^#' "$ENV_FILE" | xargs)
    echo "Environment variables loaded successfully"
    
    # Specifically export GEMINI_API_KEY for the build process
    if [ -n "$GEMINI_API_KEY" ]; then
        export GEMINI_API_KEY="$GEMINI_API_KEY"
        echo "GEMINI_API_KEY exported successfully"
    else
        echo "Warning: GEMINI_API_KEY not found in .env file"
    fi
else
    echo "Warning: .env file not found at $ENV_FILE"
    echo "Make sure to set GEMINI_API_KEY environment variable manually."
fi

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

# Verify GEMINI_API_KEY is set
if [ -z "$GEMINI_API_KEY" ]; then
    echo "Error: GEMINI_API_KEY environment variable is not set."
    echo "Please create a .env file in the project root with GEMINI_API_KEY=your_api_key_here"
    exit 1
fi

echo "GEMINI_API_KEY is set (length: ${#GEMINI_API_KEY} characters)"

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
