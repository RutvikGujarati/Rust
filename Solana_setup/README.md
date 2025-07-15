# Hello World Solana Program

This is a simple "Hello World" Solana program that logs messages to the blockchain.

## Prerequisites

1. **Solana CLI** - Install from https://docs.solana.com/cli/install-solana-cli-tools
2. **Rust** - Install from https://rustup.rs/
3. **Node.js** - For the client (optional)

## Setup Instructions

### 1. Configure Solana CLI
```bash
# Set to testnet
solana config set --url https://api.testnet.solana.com

# Check your wallet
solana address

# Check balance
solana balance
```

### 2. Build the Program
```bash
# Build the program
cargo build-bpf

# Or if you have newer Solana CLI
cargo build-sbf
```

### 3. Deploy to Testnet
```bash
# Deploy the program
solana program deploy target/deploy/hello_world.so

# Save the program ID that gets returned
# Example: Program Id: 7KqpRwzkkeweW5jQoETyLzhvs9rcCj9dVQ1MnzudirsM
```

### 4. Test the Program
```bash
# Call the program (replace PROGRAM_ID with your actual program ID)
solana program call PROGRAM_ID --keypair ~/.config/solana/id.json
```

### 5. View Program Logs
```bash
# Get the transaction signature from the previous step
# Then view the logs
solana confirm -v SIGNATURE
```

## JavaScript Client

If you want to use the JavaScript client:

```bash
# Install dependencies
npm install

# Edit client.js to add your program ID and keypair
# Then run:
node client.js
```

## Program Structure

- `src/lib.rs` - The main program logic
- `Cargo.toml` - Rust dependencies and configuration
- `client.js` - JavaScript client for interacting with the program
- `package.json` - Node.js dependencies

## What the Program Does

When called, this program:
1. Logs "Hello, World! This is my first Solana program!"
2. Logs the program ID
3. Logs the number of accounts passed to it
4. Logs the length of instruction data

## Next Steps

After successfully deploying this program, you can:
1. Create more complex programs with state management
2. Add more instructions to your program
3. Create a proper client application
4. Deploy to mainnet (when ready)

## Troubleshooting

- **Build errors**: Make sure you have the latest Solana CLI and Rust
- **Deployment errors**: Check your SOL balance and network connection
- **Permission errors**: Make sure your keypair file is accessible 