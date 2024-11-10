# Clawbote

A simple Ethereum blockchain interaction tool built in Rust using the ethers-rs library.

## Overview

Clawbote is a command-line application that connects to a local Ethereum node (like Ganache) and demonstrates basic blockchain interactions. Currently, it retrieves and displays the latest block number.

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


## Features Implemented

- **Account Balance Query**: Retrieve the ETH balance of any Ethereum address
  - Connects to a local Ethereum node (Ganache)
  - Displays balance in ETH with 2 decimal precision
  - Uses ethers-rs for reliable blockchain interaction

- **Modular Architecture**:
  - Controller-Service pattern for clean separation of concerns
  - Error handling using `eyre` for robust operation
  - Asynchronous operations with `tokio` runtime

- **Transfer Infrastructure**: Basic structure in place for future ETH transfer functionality

