use alloy::rpc::types::eth::{Log, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChronicleEvent {
    // block number
    // block hash
    // transaction hash
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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

impl From<Transaction> for ChronicleTransaction {
    fn from(tx: Transaction) -> Self {
        Self {
            // hash: tx.hash,
            // nonce: tx.nonce,
            // block_hash: tx.block_hash,
            // block_number: tx.block_number,
            // from: tx.from,
            // to: tx.to,
            // value: tx.value,
            // gas_price: tx.gas_price,
            // gas: tx.gas,
            // data: tx.input,
            // max_fee_per_gas: tx.max_fee_per_gas,
        }
    }
}

impl From<Log> for ChronicleEvent {
    fn from(log: Log) -> Self {
        Self {
            // block_number: log.block_number,
            // block_hash: log.block_hash,
            // transaction_hash: log.transaction_hash,
        }
    }
}
