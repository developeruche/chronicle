pub mod db;
pub mod errors;
pub mod indexer;
pub mod interfaces;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StateMachine {
    EVM,
    PARACHAIN,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// This is the name of the chronicle server
    pub name: Option<String>,
    /// This is a list of all the indexer Config
    pub indexer: Vec<IndexerConfig>,
    /// Server config
    pub server: ServerConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    /// This is the URL of the database
    pub db_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexerConfig {
    /// this is the name of the event, this is the name that the DB table is going to be named
    pub event_name: String,
    /// This is represents the statemachaine to be indexed
    pub state_machine: String,
    /// This is the RPC url of the state machine
    pub rpc_url: String,
    /// This is the address of the contract that is to be indexed
    pub address: String,
    /// This is the event signature of the event that is to be indexed
    pub event_signature: String,
    /// This is the block number to start indexing from
    pub block_number: u64,
    /// This is the URL of the database
    pub db_url: String,
}

impl From<String> for StateMachine {
    fn from(s: String) -> Self {
        let s = s.as_str();
        match s {
            "EVM" => Self::EVM,
            "PARACHAIN" => Self::PARACHAIN,
            _ => panic!("Invalid state machine"),
        }
    }
}
