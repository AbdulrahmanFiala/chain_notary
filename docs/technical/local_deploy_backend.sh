#!/bin/bash

# deploy.sh - Automated deployment script for ChainNotary backend

set -e

echo "Starting automated deployment..."

# Deploy backend to local network
echo "Deploying backend to local network..."
dfx deploy backend --network local

echo "Deployment completed successfully!"
echo "Your application is now running."