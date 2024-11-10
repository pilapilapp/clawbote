# Clawbote

A simple Ethereum blockchain interaction tool built in Rust using the ethers-rs library.

## Overview

Clawbote is a command-line application that connects to a local Ethereum node (like Ganache) and provides basic blockchain interaction capabilities.

## Prerequisites

- Rust and Cargo installed
- Ganache (local Ethereum blockchain)

## Installation

### Installing Ganache

1. Install Ganache:
   - Using npm (Node Package Manager):
     ```bash
     npm install -g ganache
     ```
   - Or download the Ganache GUI application from the official website:
     https://trufflesuite.com/ganache/

2. Running Ganache:
   - Using CLI:
     ```bash
     ganache
     ```
   - Or launch the Ganache GUI application
   
   By default, Ganache will run on `http://localhost:8545` with 10 pre-funded accounts

### Building Clawbote

```bash
# Build debug version
cargo build

# Build release version
cargo build --release
```

## Usage

```bash
clawbote <command> [arguments...]

Available commands:
  balance <address>     - Get account balance in ETH
  nonce <address>       - Get account transaction count (nonce)
  code-size <address>   - Get contract code size (0 for EOA)
  help                  - Show this help message
```

## Features Implemented

- **Command Line Interface**:
  - Help command for usage information
  - Structured command parsing
  - Clear error messages and usage examples

- **Account Queries**:
  - Get ETH balance of any address
  - Get transaction count (nonce) of any address
  - Get contract code size (useful for distinguishing contracts from EOAs)

- **Modular Architecture**:
  - Controller-Service pattern for clean separation of concerns
  - Error handling using `eyre` for robust operation
  - Asynchronous operations with `tokio` runtime

- **Infrastructure**:
  - Local node (Ganache) connectivity
  - Basic blockchain interaction framework
  - Foundation for future feature additions

