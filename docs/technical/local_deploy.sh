#!/bin/bash

# deploy.sh - Automated deployment script for ChainNotary
# This script automatically sets the VITE_PRINCIPAL_ID and deploys the project

set -e

echo "Starting automated deployment..."

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

# Deploy the project
echo "Deploying project..."
dfx deploy --network local

echo "Deployment completed successfully!"
echo "Your application is now running."
