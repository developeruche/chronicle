use alloy::{
    primitives::{Address, Bytes, B256, U256},
    rpc::types::eth::{Log, Transaction},
};
use postgres::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChronicleEvent {
    pub address: Address,
    pub block_number: u64,
    pub transaction_hash: B256,
    pub topics: Vec<B256>,
    pub data: Bytes,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChronicleTransaction {
    pub hash: B256,
    pub nonce: u64,
    pub block_hash: B256,
    pub block_number: u64,
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub gas_price: u128,
    pub gas: u128,
    pub max_fee_per_gas: u128,
    pub data: Bytes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChronicleIndexingMode {
    Transaction,
    Event,
}

impl From<Transaction> for ChronicleTransaction {
    fn from(tx: Transaction) -> Self {
        Self {
            hash: tx.hash,
            nonce: tx.nonce,
            block_hash: tx.block_hash.unwrap_or(B256::default()),
            block_number: tx.block_number.unwrap_or(0),
            from: tx.from,
            to: tx.to.unwrap_or(Address::ZERO),
            value: tx.value,
            gas_price: tx.gas_price.unwrap_or(0),
            gas: tx.gas,
            data: tx.input,
            max_fee_per_gas: tx.max_fee_per_gas.unwrap_or(0),
        }
    }
}

impl From<Log> for ChronicleEvent {
    fn from(log: Log) -> Self {
        Self {
            address: log.address(),
            block_number: log.block_number.unwrap_or(0),
            transaction_hash: log.transaction_hash.unwrap_or(B256::default()),
            topics: log.data().clone().topics().to_vec(),
            data: log.inner.data.data,
        }
    }
}



impl ChronicleEvent {
    /// This function would be used to store the event to the db
    /// params:
    /// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls).unwrap();]
    /// name: &str - The name of the table
    pub fn create_new_db_table(&self, db_client: &mut Client, name: &str) -> Result<(), anyhow::Error> {
        let executable = format!("
            CREATE TABLE IF NOT EXISTS {name} (
                id              SERIAL PRIMARY KEY,
                address         VARCHAR NULL,
                block_number    BIGINT NULL,
                transaction_hash VARCHAR NULL,
                topics          VARCHAR[] NULL,
                data            BYTEA NULL
            )
        ");
        db_client.batch_execute(&executable)?;

        Ok(())
    }

    /// This function would be used to store the event to the db
    /// params:
    /// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls).unwrap();]
    /// name: &str - The name of the table
    pub fn store_event_to_db(&self, db_client: &mut Client, name: &str) -> Result<(), anyhow::Error> {
        let executable = format!("
            INSERT INTO {name} (address, block_number, transaction_hash, topics, data)
            VALUES ($1, $2, $3, $4, $5)
        ");
        let stringified_topics: Vec<String> = self.topics.iter().map(|topic| topic.to_string()).collect();
        let block_number = self.block_number as i64;
        db_client.execute(&executable, &[
            &self.address.to_string(),
            &block_number,
            &self.transaction_hash.to_string(),
            &stringified_topics,
            &self.data.to_vec(),
        ])?;

        Ok(())
    }

    /// This function would be used to get the event from the db with an filter
    /// params:
    /// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls).unwrap();]
    /// name: &str - The name of the table
    pub fn get_all_events(&self, db_client: &mut Client, name: &str) -> Result<Vec<>, anyhow::Error> {
        let executable = format!("
            SELECT * FROM {name}
        ");
        let rows = db_client.query(&executable, &[]).unwrap();
        for row in rows {
            let address: String = row.get(0);
            let block_number: i64 = row.get(1);
            let transaction_hash: String = row.get(2);
            let topics: Vec<String> = row.get(3);
            let data: Vec<u8> = row.get(4);
            println!("address: {}, block_number: {}, transaction_hash: {}, topics: {:?}, data: {:?}", address, block_number, transaction_hash, topics, data);
        }
    }

    /// This function would be used to get the event from the db with an filter
    /// params:
    /// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls).unwrap();]
    /// name: &str - The name of the table
    /// filter: Vec<String> - The filter to be used
    pub fn get_all_events_with_filter(&self, db_client: &mut Client, name: &str, filter: Vec<String>) {
        let filter_decoded = filter.join(", ");
        let executable = format!("
            SELECT {filter_decoded} FROM {name}
        ");
        let rows = db_client.query(&executable, &[]).unwrap();
        for row in rows {
            let address: String = row.get(0);
            let block_number: i64 = row.get(1);
            let transaction_hash: String = row.get(2);
            let topics: Vec<String> = row.get(3);
            let data: Vec<u8> = row.get(4);
            println!("address: {}, block_number: {}, transaction_hash: {}, topics: {:?}, data: {:?}", address, block_number, transaction_hash, topics, data);
        }
    }

    pub fn get_events_by_tx_hash(&self, db_client: &mut Client, name: &str, transaction_hash: String) {
        let executable = format!("
            SELECT * FROM {name} WHERE transaction_hash = $1
        ");
        let rows = db_client.query(&executable, &[&transaction_hash]).unwrap();
        for row in rows {
            let address: String = row.get(0);
            let block_number: i64 = row.get(1);
            let transaction_hash: String = row.get(2);
            let topics: Vec<String> = row.get(3);
            let data: Vec<u8> = row.get(4);
            println!("address: {}, block_number: {}, transaction_hash: {}, topics: {:?}, data: {:?}", address, block_number, transaction_hash, topics, data);
        }
    }

    pub fn get_events_by_block_number(&self, db_client: &mut Client, name: &str, block_number: i64) {
        let executable = format!("
            SELECT * FROM {name} WHERE block_number = $1
        ");
        let rows = db_client.query(&executable, &[&block_number]).unwrap();
        for row in rows {
            let address: String = row.get(0);
            let block_number: i64 = row.get(1);
            let transaction_hash: String = row.get(2);
            let topics: Vec<String> = row.get(3);
            let data: Vec<u8> = row.get(4);
            println!("address: {}, block_number: {}, transaction_hash: {}, topics: {:?}, data: {:?}", address, block_number, transaction_hash, topics, data);
        }
    }
}
