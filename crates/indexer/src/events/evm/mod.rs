pub mod utils;


use alloy::{
    primitives::{Address, B256},
    providers::RootProvider,
    pubsub::PubSubFrontend,
    rpc::types::eth::BlockNumberOrTag,
};
use async_trait::async_trait;
use chronicle_primitives::{indexer::ChronicleEvent, interfaces::ChronicleEventIndexer};
use self::utils::{query_events, subscribe_to_events};




pub struct EvmEventIndexer {}



#[async_trait]
impl ChronicleEventIndexer for EvmEventIndexer {
    type SubProvider = RootProvider<PubSubFrontend>;
    type ContractAddress = Address;
    type EventSignature = B256;
    type BlockNumber = BlockNumberOrTag;

    async fn query_events(
        &self,
        provider: Self::SubProvider,
        addr: Self::ContractAddress,
        event_sig: Self::EventSignature,
        block_number: Self::BlockNumber,
    ) -> Result<Vec<ChronicleEvent>, anyhow::Error> {
        query_events(provider, addr, event_sig, block_number).await
    }

    async fn subscribe_to_events<F>(
        &self,
        provider: Self::SubProvider,
        addr: Vec<Self::ContractAddress>,
        event_sig: Self::EventSignature,
        callback: F,
    ) where
        F: FnMut(ChronicleEvent) + Send,
    {
        subscribe_to_events(provider, addr, event_sig, callback).await;
    }
}
