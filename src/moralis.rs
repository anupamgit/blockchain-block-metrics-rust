//! This file defines the MoralisClient struct which provides the client connection to the Moralis API.
//! It includes the fetch_latest_block function to retrieve the latest block information from the specified blockchain.
//! Moralis doc: https://deep-index.moralis.io/api-docs-2.2/#/

use chrono::Utc;

use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct Block {
    pub timestamp: String,
    pub number: String,
    pub hash: String,
    pub parent_hash: String,
    pub nonce: String,
    pub sha3_uncles: String,
    pub logs_bloom: String,
    pub transactions_root: String,
    pub state_root: String,
    pub receipts_root: String,
    pub miner: String,
    pub difficulty: String,
    pub total_difficulty: String,
    pub size: String,
    pub extra_data: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub transaction_count: String,
    pub base_fee_per_gas: String,
    pub transactions: Vec<Transaction>,
}

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    pub hash: String,
    pub nonce: String,
    pub transaction_index: String,
    pub from_address: String,
    pub from_address_label: Option<String>,
    pub to_address: String,
    pub to_address_label: Option<String>,
    pub value: String,
    pub gas: String,
    pub gas_price: String,
    pub input: String,
    pub receipt_cumulative_gas_used: String,
    pub receipt_gas_used: String,
    pub receipt_contract_address: Option<String>,
    pub receipt_root: Option<String>,
    pub receipt_status: String,
    pub block_timestamp: String,
    pub block_number: String,
    pub block_hash: String,
    pub transfer_index: Vec<i64>,
    pub logs: Vec<Log>,
    pub transaction_fee: String,
}

#[derive(Deserialize, Serialize)]
pub struct Log {
    pub log_index: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub address: String,
    pub data: String,
    pub topic0: String,
    pub topic1: Option<String>,
    pub topic2: Option<String>,
    pub topic3: Option<String>,
    pub block_timestamp: String,
    pub block_number: String,
    pub block_hash: String,
    pub transfer_index: Vec<i64>,
    pub transaction_value: String,
}

#[derive(Deserialize)]
struct DateToBlockResponse {
    block: i32,
}

#[derive(Clone)]
pub struct MoralisClient {
    api_key: String,
    client: Client,
}

impl MoralisClient {
    pub fn new() -> Self {
        let api_key = env::var("MORALIS_API_KEY").expect("MORALIS_API_KEY must be set");
        MoralisClient {
            api_key: api_key,
            client: Client::new(),
        }
    }

    pub async fn fetch_latest_block(&self, chain: &str) -> Result<Block, String> {
        let block_number = self.date_to_block_number(&chain).await?;
        info!("Block number for chain {} is {}", chain, block_number);

        self.get_block_details(&chain, block_number).await
    }

    async fn date_to_block_number(&self, chain: &str) -> Result<i32, String> {
        let date = Utc::now().timestamp_millis();

        let url = format!(
            "https://deep-index.moralis.io/api/v2.2/dateToBlock?chain={}&date={}",
            chain, date
        );
        match self.invoke_client(&url).await {
            Ok(res) => {
                let response: DateToBlockResponse = res
                    .json()
                    .await
                    .map_err(|e| format!("JSON parse error: {}", e))?;
                Ok(response.block)
            }
            Err(e) => Err(e),
        }
    }

    async fn get_block_details(&self, chain: &str, block_number: i32) -> Result<Block, String> {
        let url = format!(
            "https://deep-index.moralis.io/api/v2.2/block/{}?chain={}",
            block_number, chain
        );
        match self.invoke_client(&url).await {
            Ok(res) => {
                let block: Block = res
                    .json()
                    .await
                    .map_err(|e| format!("JSON parse error: {}", e))?;
                Ok(block)
            }
            Err(e) => Err(e),
        }
    }

    // invoke_client calls the Moralis API with the given URL
    async fn invoke_client(&self, url: &str) -> Result<reqwest::Response, String> {
        match self
            .client
            .get(url)
            .header("X-API-Key", self.api_key.clone())
            .send()
            .await
        {
            Ok(res) => match res.status().is_success() {
                true => Ok(res),
                false => {
                    let error_text = res
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    error!("Request failed: {}", error_text);
                    Err(error_text)
                }
            },
            Err(e) => {
                error!("Request error: {}", e);
                Err(format!("Request error: {}", e))
            }
        }
    }
}

// test a mock moralis client
#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_fetch_latest_block() {
        dotenv().ok();
        let client = MoralisClient::new();
        let block = client.fetch_latest_block("eth").await.unwrap();
        assert_eq!(block.number != "0", true);
    }
}
