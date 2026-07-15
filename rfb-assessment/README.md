# Rust for Bitcoin Assessment

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![Bitcoin Core](https://img.shields.io/badge/Bitcoin%20Core-24.0.1-orange.svg)](https://bitcoincore.org/)
[![Polar](https://img.shields.io/badge/Polar-4.0.0-green.svg)](https://github.com/jamaljsr/polar)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

##  Overview

A command-line application written in Rust that communicates with a local Bitcoin Core node running on **Regtest** using **Polar**. This project demonstrates:

- ✅ Rust programming and idiomatic code
- ✅ Bitcoin Core JSON-RPC interface
- ✅ Command-line application development
- ✅ Error handling and graceful degradation
- ✅ Clean, modular project structure
- ✅ Comprehensive documentation

##  Features

### Part 1: Bitcoin Core Setup with Polar
- [x] Automated connection to Polar-managed Bitcoin Core node
- [x] Flexible configuration (env vars, config file, CLI flags)
- [x] No source code modification required

### Part 2: CLI Commands
- [x] `blockchain-info` - Display blockchain status
- [x] `wallet-info` - Display wallet details
- [x] `balance` - Show wallet balance
- [x] `new-address` - Generate new receiving address

### Part 3: Generic RPC Command
- [x] Execute arbitrary Bitcoin Core RPC methods
- [x] Dynamic parameter handling
- [x] JSON response formatting

## Prerequisites

| Requirement | Version | Installation |
|-------------|---------|--------------|
| **Rust** | 1.70+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` |
| **Polar** | 4.0.0 | Download from [releases](https://github.com/jamaljsr/polar/releases) |
| **Docker** | Latest | `sudo apt install docker.io docker-compose` |
| **Git** | Latest | `sudo apt install git` |

##  Setting Up Bitcoin Core with Polar

### Step 1: Install and Launch Polar

```bash
# Download Polar (Linux)
wget https://github.com/jamaljsr/polar/releases/download/v4.0.0/polar-linux-x86_64-v4.0.0.AppImage
chmod +x polar-linux-x86_64-v4.0.0.AppImage
./polar-linux-x86_64-v4.0.0.AppImage

# Or install via .deb package
sudo dpkg -i polar-linux-amd64-v4.0.0.deb
polar

RPC URL: http://localhost:18443
RPC User: polar
RPC Password: [auto-generated]

Application Configuration

export RPC_URL="http://localhost:18443"
export RPC_USER="polar"
export RPC_PASS="your-password-from-polar"

cargo run -- blockchain-info

Usage Examples

1. Display Blockchain Information

$ cargo run -- blockchain-info

┌─────────────────────────────────────────┐
│        BLOCKCHAIN INFORMATION          │
├─────────────────────────────────────────┤
│ Chain:               regtest            │
│ Blocks:              307                │
│ Headers:             307                │
│ Difficulty:          4.656542373906925e-10 │
│ Verification Progress: 1                │
└─────────────────────────────────────────┘


2. Display Wallet Information

$ cargo run -- wallet-info Miner

┌─────────────────────────────────────────┐
│           WALLET INFORMATION           │
├─────────────────────────────────────────┤
│ Wallet:              Miner              │
│ Balance:             29.99997180 BTC    │
│ Unconfirmed Balance: 0.00000000 BTC     │
│ Transactions:        103                │
└─────────────────────────────────────────┘

3. Check Wallet Balance

$ cargo run -- balance Miner

Balance: 29.99997180 BTC

4. Generate New Address

$ cargo run -- new-address Miner

New address:
  bcrt1qveagz7586262xfugslw07x33ncqyxxy7zxeze6
  Wallet: Miner

5. Generic RPC Commands

# Get block count
$ cargo run -- rpc getblockcount
307

# Get block hash
$ cargo run -- rpc getblockhash 0
"0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206"

# Get wallet info with wallet context
$ cargo run -- rpc --wallet Miner getwalletinfo
{
  "walletname": "Miner",
  "walletversion": 169900,
  "txcount": 103,
  ...
}


PROJECT STRUCTURE

rfb-assessment/
├── src/
│   ├── main.rs              # Application entry point
│   ├── cli.rs               # Command-line interface definitions
│   ├── config.rs            # Configuration management
│   ├── errors.rs            # Error handling
│   ├── rpc/
│   │   └── client.rs        # Bitcoin Core RPC client
│   └── commands/
│       ├── mod.rs           # Command module exports
│       ├── blockchain.rs    # blockchain-info command
│       ├── wallet.rs        # wallet-info and balance commands
│       └── address.rs       # new-address command
├── Cargo.toml               # Dependencies and metadata
├── Cargo.lock               # Locked dependencies
├── config.example.json      # Example configuration file
├── .env.example             # Example environment variables
├── .gitignore               # Git ignore rules
└── README.md                # Documentation

Error Handling

The application gracefully handles:

Error Type		Example		    Response

Invalid Credentials	Wrong password	    Error: RPC Error: Authorization failed
Connection Failure	Node not running	Error: HTTP Error: Connection refused
Invalid Method	Unknown RPC method	    Error: RPC Error: Method not found
Invalid Parameters	Wrong number of args	Error: Invalid Argument: expected 1 parameter
Missing Wallet	        Wallet doesn't exist	Error: RPC Error: Wallet not found



Testing

Running Tests

# Run all tests
cargo test

# Run specific test module
cargo test --test integration_test

# Run with logging
RUST_LOG=debug cargo test

Example Test Cases

# Run all tests
cargo test

# Run specific test module
cargo test --test integration_test

# Run with logging
RUST_LOG=debug cargo test


Development workflow


Code Quality

# Format code
cargo fmt

# Check for issues
cargo clippy -- -D warnings

# Build for release
cargo build --release

# Run tests
cargo test

Final Checks

# Format everything
cargo fmt

# Check Clippy
cargo clippy -- -D warnings

# Build release
cargo build --release

# Test all commands
cargo run -- blockchain-info
cargo run -- wallet-info Miner
cargo run -- balance Miner
cargo run -- new-address Miner
cargo run -- rpc getblockcount






