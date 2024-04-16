pub mod errors;
use serde::{Deserialize, Serialize};
pub mod indexer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// This is the name of the chronicle server
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexerConfig;

impl Config {
    /// This returns an instance of the server related section of the config
    pub fn into_server(&self) -> ServerConfig {
        ServerConfig
    }

    /// This returns the index related section of the config
    pub fn into_indexer(&self) -> IndexerConfig {
        IndexerConfig
    }
}
