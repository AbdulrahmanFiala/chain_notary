#!/bin/bash

set -e

echo "Starting automated deployment..."

# Verify GEMINI_API_KEY is available for Rust compilation
if [ -z "$GEMINI_API_KEY" ]; then
    echo "Error: GEMINI_API_KEY environment variable is not set."
    echo "This variable is required for backend compilation."
    exit 1
fi

echo "GEMINI_API_KEY is available (length: ${#GEMINI_API_KEY} characters)"

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

# Deploy the project to mainnet
echo "Deploying project to mainnet..."
dfx deploy --network ic --yes

echo "Mainnet deployment completed successfully!"
echo "Your application is now running."