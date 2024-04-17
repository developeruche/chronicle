use serde::{Deserialize, Serialize};

pub struct ChronicleEvent {
    // block number
    // block hash
    // transaction hash
}

pub struct ChronicleTransaction {
    // hash
    // nonce
    // block hash
    // block number
    // from
    // to
    // vaule
    // gas price
    // gas
    // data (alloy::input)
    // max fee per gas
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChronicleIndexingMode {
    Transaction,
    Event,
}
