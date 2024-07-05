//! This file defines the Metrics struct and provides functionality
//! to transform blockchain block data into a metrics format. It includes
//! serialization support and unit tests for the transformation function.

use crate::moralis::Block;
use serde::{Deserialize, Serialize};

/// Represents various metrics for a blockchain block.
#[derive(Clone, Deserialize, Serialize)]
pub struct Metrics {
    pub blockchain: String,
    pub block_number: String,
    pub timestamp: String,
    pub gas_used: String, // in units of gas
    pub transaction_count: String,
    pub block_size: String,       // in bytes
    pub transaction_fees: String, // in Ether/Token
    pub base_fee_per_gas: String, // in Wei
}

/// Transforms a Block into a Metrics struct.
pub fn transform_block_to_metrics(blockchain: &str, block: &Block) -> Metrics {
    let transaction_fees: f64 = block
        .transactions
        .iter()
        .map(|tx| tx.transaction_fee.parse::<f64>().unwrap_or(0.0))
        .sum();

    Metrics {
        blockchain: blockchain.to_string(),
        block_number: block.number.clone(),
        timestamp: block.timestamp.clone(),
        gas_used: format!("{} gas", block.gas_used),
        transaction_count: block.transaction_count.clone(),
        block_size: format!("{} bytes", block.size),
        transaction_fees: format!("{:.6} ETH", transaction_fees), // Assuming fees in Ether for Ethereum
        base_fee_per_gas: format!("{} Wei", block.base_fee_per_gas),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_block_to_metrics() {
        let block = Block {
            timestamp: "1635734400".to_string(),
            number: "1337".to_string(),
            hash: "0xdeadbeef".to_string(),
            parent_hash: "0xdeadbeef".to_string(),
            nonce: "0xdeadbeef".to_string(),
            sha3_uncles: "0xdeadbeef".to_string(),
            logs_bloom: "0xdeadbeef".to_string(),
            transactions_root: "0xdeadbeef".to_string(),
            state_root: "0xdeadbeef".to_string(),
            receipts_root: "0xdeadbeef".to_string(),
            miner: "0xdeadbeef".to_string(),
            difficulty: "0xdeadbeef".to_string(),
            total_difficulty: "0xdeadbeef".to_string(),
            size: "1337".to_string(),
            extra_data: "0xdeadbeef".to_string(),
            gas_limit: "1337".to_string(),
            gas_used: "1337".to_string(),
            transaction_count: "1337".to_string(),
            base_fee_per_gas: "1337".to_string(),
            transactions: vec![],
        };
        let metrics = transform_block_to_metrics("Ethereum", &block);
        assert_eq!(metrics.blockchain, "Ethereum");
        assert_eq!(metrics.transaction_count, "1337");
    }
}
