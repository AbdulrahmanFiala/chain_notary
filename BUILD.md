# Chain Notary - Local Development Setup

This guide will help you set up and run the Chain Notary project locally on your machine.

## Prerequisites

### 1. Install Developer Tools

You can install the developer tools natively or use Dev Containers.

#### Option 1: Native Installation

> Installing `dfx` natively is currently only supported on macOS and Linux systems. On Windows, it is recommended to use the Dev Containers option.

1. Install `dfx` (Internet Computer SDK):
   ```bash
   sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
   ```

   > On Apple Silicon (e.g., Apple M1 chip), make sure you have Rosetta installed (`softwareupdate --install-rosetta`).

2. Install [NodeJS](https://nodejs.org/en/download/package-manager).

3. Install Rust and required tools:
   - Install [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html#install-rust-and-cargo): 
     ```bash
     curl https://sh.rustup.rs -sSf | sh
     ```
   - Install candid-extractor:
     ```bash
     cargo install candid-extractor
     ```

4. Navigate to your project directory:
   ```bash
   cd /path/to/chain_notary
   ```

#### Option 2: Dev Containers

For the easiest setup, especially on Windows, use Dev Containers:

1. Install the [Dev Container extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) for VS Code
2. Install [Docker](https://docs.docker.com/engine/install/)
3. Make sure Docker is running
4. Open the project in VS Code and select `Dev-Containers: Reopen in Container` from the command palette (F1 or Ctrl/Cmd+Shift+P)

> Note: Local development ports (e.g., the ports used by `dfx` or `vite`) are forwarded from the Dev Container to your local machine. In the VS Code terminal, use Ctrl/Cmd+Click on the displayed local URLs to open them in your browser.

## Setup and Deployment

### 2. Start the Local Development Environment

```bash
dfx start --background
```

### 3. Create a Local Developer Identity (Recommended)

To manage your project's canisters securely, create a local [developer identity](https://internetcomputer.org/docs/building-apps/getting-started/identities):

```bash
dfx identity new IDENTITY_NAME
dfx identity use IDENTITY_NAME
```

Replace `IDENTITY_NAME` with your preferred identity name. The first command will create a new identity and return your identity's seed phrase. **Be sure to save this in a safe, secure location.**

Your identity will have a principal ID associated with it, which is used to identify different entities on the Internet Computer.

### 4. Deploy the Project Locally

Install dependencies and deploy to your local environment:

```bash
npm install
dfx deploy
```

Your project will be hosted on your local machine. The terminal will display the local canister URLs for your project. Open these URLs in your web browser to view the local instance of Chain Notary.

## Production Deployment

### 5. Obtain Cycles

To deploy your project to the mainnet for public access, you'll need [cycles](https://internetcomputer.org/docs/building-apps/getting-started/tokens-and-cycles). Cycles pay for the resources your project uses on the mainnet.

> This follows ICP's [reverse gas model](https://internetcomputer.org/docs/building-apps/essentials/gas-cost), where developers pay for gas fees rather than users. This provides an enhanced user experience as users don't need to hold tokens or sign transactions.

> Estimate your project's costs using the [pricing calculator](https://internetcomputer.org/docs/building-apps/essentials/cost-estimations-and-examples).

Obtain cycles by [converting ICP tokens using `dfx`](https://internetcomputer.org/docs/building-apps/developer-tools/dfx/dfx-cycles#dfx-cycles-convert).

### 6. Deploy to Mainnet

Once you have cycles, deploy to the Internet Computer mainnet:

```bash
dfx deploy --network ic
```

After deployment, your project will continuously require cycles to pay for resources. You'll need to [top up](https://internetcomputer.org/docs/building-apps/canister-management/topping-up) your canisters or set up automatic cycles management through services like [CycleOps](https://cycleops.dev/).

> **Important:** If your project's canisters run out of cycles, they will be removed from the network.

## Development Workflow

- **Local Development:** Use `dfx deploy` to deploy changes locally
- **Testing:** Test your changes on the local network before mainnet deployment
- **Production:** Use `dfx deploy --network ic` to deploy to mainnet

## Troubleshooting

- If you encounter permission issues, ensure your user owns the project directory
- For build errors, verify all dependencies are installed correctly
- Check the [DFINITY documentation](https://internetcomputer.org/docs) for detailed guides

## Additional Resources

- [Internet Computer Documentation](https://internetcomputer.org/docs)
- [DFINITY Examples Repository](https://github.com/dfinity/examples)
- [Candid Language Reference](https://internetcomputer.org/docs/current/developer-docs/build/candid/candid-intro)
