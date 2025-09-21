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

echo "Starting automated backend deployment..."

# Create canister if it doesn't exist
echo "Creating canister if needed..."
dfx canister create backend --network local || echo "Canister already exists"

# Verify GEMINI_API_KEY is set
if [ -z "$GEMINI_API_KEY" ]; then
    echo "Error: GEMINI_API_KEY environment variable is not set."
    echo "Please create a .env file in the project root with GEMINI_API_KEY=your_api_key_here"
    exit 1
fi

echo "GEMINI_API_KEY is set (length: ${#GEMINI_API_KEY} characters)"

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