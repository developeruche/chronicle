use crate::Task;
use async_trait::async_trait;
use chronicle_indexer::events::evm::EvmEventIndexer;
use chronicle_primitives::{IndexerConfig, StateMachine, interfaces::ChronicleEventIndexer};
use postgres::{Client, NoTls};
use tokio::select;
use tokio_util::sync::CancellationToken;
use alloy::{
    providers::ProviderBuilder,
    rpc::client::WsConnect,
};
use tracing::info;

#[derive(Debug)]
pub struct IndexerTask {
    pub config: IndexerConfig
}

#[async_trait]
impl Task for IndexerTask {
    async fn run(mut self: Box<Self>, shutdown_token: CancellationToken) -> anyhow::Result<()> {
        let mut client = Client::connect(&self.config.db_url, NoTls)?;
        let ws = WsConnect::new(self.config.rpc_url.clone());
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();
        match self.config.state_machine.clone().into() {
            StateMachine::EVM => {
                let evm_event_indexer = EvmEventIndexer::new(self.config.event_name.clone());

                // This queries events that have happened from this block number and stores them in the database
                // It also subscribes to new events and stores them in the database
                tokio::spawn(async move {
                    loop {
                        select! {
                            event_n_sub = evm_event_indexer.query_events(
                                provider.clone(),
                                self.config.address.clone().parse().expect("CONFIG address could not be parsed"),
                                self.config.event_signature.clone().parse().expect("CONFIG event signature is missing"),
                                self.config.block_number.into(),
                                &mut client,
                            ) => {
                                // Want this indexing to halt before
                                event_n_sub.unwrap();
                            }
                            _ = shutdown_token.cancelled() => {
                                info!("Shutting down chain watcher");
                                break;
                            }
                        }
                    }
                });

            },
            StateMachine::PARACHAIN => {}
        }
        Ok(())
    }
}

impl IndexerTask {
    pub fn new(config: IndexerConfig) -> Self {
        Self { config }
    }

    /// Converts the task into a boxed trait object.
    pub fn boxed(self) -> Box<dyn Task> {
        Box::new(self)
    }
}
