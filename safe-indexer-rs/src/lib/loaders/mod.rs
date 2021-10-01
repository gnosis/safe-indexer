use async_trait::async_trait;
use crate::rpc::models::{RpcTransaction, Topic};

pub mod in_mem_loader;
pub mod default_event_looper;

#[async_trait]
pub trait EventLoader {
    async fn get_transaction_hashes_for_event(&self, safe_address: &str, from: u64, topic: Topic) -> anyhow::Result<Vec<String>>;

    async fn was_tx_hash_checked(&self, tx_hash: &str) -> bool;

    async fn process_tx_hash(&self, tx_hash: &str) -> anyhow::Result<RpcTransaction>;

    async fn last_available_block(&self) -> anyhow::Result<u64>;
}

#[async_trait]
pub trait EventLooper {
    async fn start(&self, safe_address: &str, event_loader: &(impl EventLoader + Sync)) -> anyhow::Result<()>;
}