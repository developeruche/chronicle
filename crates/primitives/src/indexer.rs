use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct ChronicleEvent {
    // block number
    // block hash
    // transaction hash
}


pub struct ChronicleBlock {
    //fields like db, etc
}


/// This index triat would be shared across all supported chains
#[async_trait]
pub trait ChronicleEventIndexer {
    type SubProvider;
    type ContractAddress;
    type EventSignature;
    type BlockNumber;


    fn query_events(&self, provider: Self::SubProvider, addr: Self::ContractAddress, event_sig: Self::EventSignature, block_nuber: Self::BlockNumber) -> Result<Vec<ChronicleEvent>, anyhow::Error>; )
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChronicleIndexingMode {
    Transaction,
    Event
}
