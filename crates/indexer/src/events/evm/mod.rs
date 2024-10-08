pub mod utils;

use self::utils::{query_events, subscribe_to_events};
use alloy::{
    primitives::{Address, B256},
    providers::RootProvider,
    pubsub::PubSubFrontend,
    rpc::types::eth::BlockNumberOrTag,
};
use async_trait::async_trait;
use chronicle_primitives::{
    db::{create_new_event_db_table, store_event_to_db},
    interfaces::ChronicleEventIndexer,
};

pub struct EvmEventIndexer {
    /// This is the name if this indexer instance, this is used for the DB table name
    name: String,
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

    // TODO: This should be renamed to query then subscribe events
    async fn query_events(
        &self,
        provider: Self::SubProvider,
        addr: Self::ContractAddress,
        event_sig: Self::EventSignature,
        block_number: Self::BlockNumber,
        db_client: &mut tokio_postgres::Client,
    ) -> Result<(), anyhow::Error> {
        create_new_event_db_table(db_client, &self.name).await?;
        // Query existing events from the specified block number
        let events = query_events(provider.clone(), addr, event_sig, block_number).await?;

        // Store all this event is the database
        for event in events {
            store_event_to_db(&event, db_client, &self.name).await?;
        }

        // Now subsbribing the events
        self.subscribe_to_events(provider, vec![addr], event_sig, db_client)
            .await?;

        Ok(())
    }

    async fn subscribe_to_events(
        &self,
        provider: Self::SubProvider,
        addr: Vec<Self::ContractAddress>,
        event_sig: Self::EventSignature,
        db_client: &mut tokio_postgres::Client,
    ) -> Result<(), anyhow::Error> {
        subscribe_to_events(provider, addr, event_sig, db_client, &self.name).await;

        Ok(())
    }
}
