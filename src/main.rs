pub mod reader;
use std::str::FromStr;
use web3::types::H256;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone)]
pub struct State {
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct LogResponse {
    pub id: u64,
    pub result: Vec<web3::types::Log>,
}

use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_ansi(false)
        .with_level(false)
        .with_target(false);
    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .or_else(|_| tracing_subscriber::EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    let topic_str = match std::env::var("TOPIC") {
        Ok(x) => x,
        Err(e) => panic!("please provide TOPIC {}", e),
    };
    let topic: H256 = H256::from_str(&topic_str).expect("invalid TOPIC");

    let endpoint = match std::env::var("RPC_ENDPOINT") {
        Ok(x) => x,
        Err(e) => panic!("please provide RPC_ENDPOINT {}", e),
    };
    let genesis_block = match std::env::var("MIN_BLOCK") {
        Ok(x) => match x.parse::<u64>() {
            Ok(x) => x,
            Err(e) => panic!("failed parsing MIN_BLOCK {}", e),
        },
        Err(e) => panic!("please provide MIN_BLOCK {}", e),
    };
    let transport = reader::get_transport(&endpoint).await;

    let web3 = web3::Web3::new(transport);
    let chain_id = web3.eth().chain_id().await?.as_u64();
    let batch_size = 10000u64;

    let mut scanner = reader::Scanner::new(chain_id, genesis_block,None, batch_size);
    let _ = scanner.scan_for_topics(&web3, topic).await;

    Ok(())
}
