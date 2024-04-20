pub mod utils;

use self::utils::{query_events, subscribe_to_events};
use alloy::{
    primitives::{Address, B256},
    providers::RootProvider,
    pubsub::PubSubFrontend,
    rpc::types::eth::BlockNumberOrTag,
};
use async_trait::async_trait;
use chronicle_primitives::{db::store_event_to_db, indexer::ChronicleEvent, interfaces::ChronicleEventIndexer};
use postgres::Client;

pub struct EvmEventIndexer {
    name: String
}

impl EvmEventIndexer {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

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
        db_client: &mut Client,
    ) -> Result<(), anyhow::Error> {
        let events = query_events(provider, addr, event_sig, block_number).await?;

        for event in events {
            store_event_to_db(&event, db_client, &self.name)?;
        }

        Ok(())
    }

    async fn subscribe_to_events<F>(
        &self,
        provider: Self::SubProvider,
        addr: Vec<Self::ContractAddress>,
        event_sig: Self::EventSignature,
        db_client: &mut Client,
    ) -> Result<(), anyhow::Error>
    {
        let callback = |log: ChronicleEvent| {
            store_event_to_db(&log, db_client, &self.name).unwrap();
        };

        subscribe_to_events(provider, addr, event_sig, callback).await;

        Ok(())
    }
}
