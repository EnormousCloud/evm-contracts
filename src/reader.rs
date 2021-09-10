use tracing::debug;
use web3::api::Eth;
use web3::transports::{Either, Http, Ipc};
use web3::types::{FilterBuilder, Log, H160, H256};
use web3::{Transport, Web3};

pub async fn get_transport(source: &str) -> Either<Http, Ipc> {
    if source.contains(".ipc") {
        let transport = Ipc::new(source)
            .await
            .expect("Failed to connect to IPC file");
        debug!("Connected to {:?}", source);
        Either::Right(transport)
    } else {
        let transport = Http::new(source).expect("Invalid RPC HTTP endpoint");
        debug!("Connecting to {:?}", source);
        Either::Left(transport)
    }
}

#[derive(Clone, Debug)]
pub struct BlockBatch {
    pub from: u64,
    pub to: u64,
}

pub async fn get_batches<T: Transport>(
    eth: Eth<T>,
    genesis: u64,
    max: Option<u64>,
    batch_size: u64,
) -> Vec<BlockBatch> {
    let max_block: u64 = match max {
        Some(x) => x,
        None => eth
            .block_number()
            .await
            .expect("max block height failure")
            .as_u64(),
    };
    let mut from = genesis;
    let mut res = vec![];
    while from <= max_block {
        let to = if from + batch_size > max_block {
            max_block
        } else {
            from + batch_size - 1
        };
        res.push(BlockBatch { from, to });
        from = from + batch_size
    }
    res
}

#[derive(Debug, Clone)]
pub struct Scanner {
    chain_id: u64,
    genesis_block: u64,
    max_block: Option<u64>,
    batch_size: u64,
}

impl Scanner {

    pub fn new(
        chain_id: u64,
        genesis_block: u64,
        max_block: Option<u64>,
        batch_size: u64,
    ) -> Self {
        Self {
            chain_id,
            genesis_block,
            max_block,
            batch_size,
        }
    }

    pub async fn scan_for_topics<T>(
        &mut self,
        web3: &Web3<T>,
        topic1: H256,
    ) -> anyhow::Result<u64>
    where
        T: Transport,
    {
        let chain_id = self.chain_id;
        let mut last_block = self.genesis_block;
        let mut contracts: Vec<H160> = vec![];
        for b in get_batches(
            web3.eth(),
            self.genesis_block,
            self.max_block,
            self.batch_size,
        )
        .await
        {
            println!("BATCH {:?}/{}", b, chain_id);
            let filter = FilterBuilder::default()
                .from_block(b.from.into())
                .to_block(b.to.into())
                .topics(Some(vec![topic1]), None, None, None)
                .build();
            let logs: Vec<Log> = web3.eth().logs(filter).await?;
            if logs.len() > 0 {
                for l in logs {
                    println!("FOUND AT {:#?}", l.address);
                    if ! contracts.contains(&l.address) {
                        contracts.push(l.address);
                    }
                }
            }
            last_block = b.to;
        }

        println!("CONTRACTS {:#?} on chain {}", contracts, chain_id);
        Ok(last_block)
    }
}