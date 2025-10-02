# ChainNotary - AI Coding Agent Guide

## Project Overview
ChainNotary is a blockchain-based document notarization platform built on the Internet Computer (ICP) for the Egyptian financial market. It enables notarization and verification of financial documents with AI-powered analytics.

## Tech Stack
- **Backend**: Rust (DFINITY canister)
- **Frontend**: React 19 + TypeScript + Vite
- **Styling**: Tailwind CSS + Ant Design
- **State Management**: Redux Toolkit
- **Testing**: Cypress
- **Blockchain**: Internet Computer Protocol (ICP)

## Directory Structure
```
/home/joeafify/chain_notary/
├── backend/                 # Rust canister code
│   ├── src/
│   │   ├── functions/       # Update and query functions
│   │   ├── types/           # Data models
│   │   ├── storage/         # Data persistence
│   │   ├── logging/         # Logging utilities
│   │   └── lib.rs           # Main entry point
│   └── Cargo.toml
├── frontend/                # React application
│   ├── src/
│   │   ├── components/      # Reusable UI components
│   │   ├── pages/           # Route components
│   │   ├── services/        # API service functions
│   │   ├── store/           # Redux store and slices
│   │   ├── utils/           # Utility functions
│   │   └── App.tsx          # Root component
│   └── package.json
├── docs/                    # Documentation
└── dfx.json                 # DFINITY configuration
```

## Coding Conventions

### Backend (Rust)
- Use modules for organization (functions, types, storage, etc.)
- Follow Rust naming conventions
- Use `ic_cdk` for canister development
- Export Candid interface with `ic_cdk::export_candid!()`

### Frontend (TypeScript/React)
- Use functional components with hooks
- Import paths: `@/*` maps to `src/*`
- Strict TypeScript configuration
- Redux Toolkit for state management
- Ant Design for UI components
- Tailwind for styling

### General
- No comments in code unless necessary
- Use ESLint and Prettier for code formatting
- Follow existing patterns in the codebase

## Key Files
- `backend/src/lib.rs`: Main canister entry point
- `frontend/src/App.tsx`: Root React component
- `dfx.json`: Canister configuration
- `frontend/package.json`: Frontend dependencies and scripts

## Development Workflow
1. Start local network: `dfx start --background`
2. Install dependencies: `npm install`
3. Deploy locally: `./docs/technical/local_deploy.sh`
4. Frontend dev server: `npm run dev` (in frontend/)
5. Build: `npm run build`

## Environment Variables
- `VITE_PRINCIPAL_ID`: User's principal ID (set automatically by deploy script)
- `GEMINI_API_KEY`: Required for AI analytics (set in .env file)

## Testing
- Component tests: `npm run test` (Cypress)
- E2E tests: `npm run cy:run`

## Deployment
- Local: `./docs/technical/local_deploy.sh`
- Mainnet: `./docs/technical/mainnet_deploy.sh`