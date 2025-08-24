#!/bin/bash

# deploy.sh - Automated deployment script for Chain Notary
# This script automatically sets the VITE_PRINCIPAL_ID and deploys the project

set -e

echo "ðŸš€ Starting automated deployment..."

# Get the current dfx identity (principal ID)
echo "ðŸ“‹ Getting principal ID..."
PRINCIPAL_ID=$(dfx identity get-principal)

if [ -z "$PRINCIPAL_ID" ]; then
    echo "âŒ Error: Could not get principal ID. Make sure dfx is installed and you're logged in."
    exit 1
fi

echo "âœ… Principal ID: $PRINCIPAL_ID"

# Export the environment variable
export VITE_PRINCIPAL_ID="$PRINCIPAL_ID"
echo "ðŸ”§ Set VITE_PRINCIPAL_ID=$VITE_PRINCIPAL_ID"

# Deploy the project
echo "ðŸŽ¯ Deploying project..."
dfx deploy

echo "âœ… Deployment completed successfully!"
echo "ðŸŒ Your application is now running."
