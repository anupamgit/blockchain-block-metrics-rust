## Table of Contents

- [Multi-Blockchain Block Metrics Comparison](#multi-blockchain-block-metrics-comparison)
  - [Features](#features)
  - [Setup](#setup)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
    - [View the metrics](#view-the-metrics)
    - [Metrics Explained](#metrics-explained)
    - [Example Table](#example-table)

# Multi-Blockchain Block Metrics Comparison

This project provides a simple web interface to compare the latest block metrics from different blockchains. It fetches and displays various metrics for the most recent block of each supported blockchain, allowing for easy comparison of their performance and efficiency. This is not intended for production use.

## Features

- Fetches the latest block information from multiple blockchains.
- Displays key metrics in a clean and readable table format.
- Provides a legend explaining each metric.
- Automatically refreshes data on page reload.

## Setup

### Prerequisites

- Rust and Cargo installed. You can install Rust [here](https://www.rust-lang.org/tools/install).
- A Moralis API key. You can sign up and get your API key [here](https://moralis.io/).

### Installation

1. **Clone the repository**:
2. **Create a .env file**:
   ```sh
   touch .env
3. **Add the following environment variables to the .env file**:
   ```sh
   MORALIS_API_KEY=your_moralis_api_key
   PORT=3030
4. **Build and run the project**:
   ```sh
   cargo build
   cargo run
   cargo test
5. **Access the web interface**:
   ```sh
   Open your browser and navigate to http://127.0.0.1:3030/metrics to see the latest block metrics.

### View the metrics
- Open your browser and navigate to http://127.0.0.1:3030/metrics.

### Metrics Explained
- Gas Used: Total gas used per block (in units of gas).
- Transaction Count: Number of transactions in the block.
- Block Size: Size of the block (in bytes).
- Transaction Fees: Total fees for transactions in the block (in Ether/Token).
- Base Fee Per Gas: Minimum gas price per unit of gas (in Wei).

### Example Table

| Blockchain          | Block Number | Timestamp           | Gas Used    | Transactions Count | Block Size | Transaction Fees | Base Fee Per Gas |
| ------------------- | ------------ | ------------------- | ----------- | ------------------ | ---------- | ---------------- | ---------------- |
| Ethereum            | 20237431     | 2024-07-05T02:50:23 | 1794283 gas | 25                 | 7984 bytes | 0.001831 ETH     | 5832356099 Wei   |
| Binance Smart Chain | 12345678     | 2024-07-05T03:00:00 | 2500000 gas | 30                 | 1024 bytes | 0.002000 BNB     | 7000000000 Wei   |
