use postgres::Client;

use crate::indexer::{ChronicleEvent, DisplayChronicleEvent};




/// This function would be used to store the event to the db
/// params:
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;]
/// name: &str - The name of the table
pub fn create_new_db_table(db_client: &mut Client, name: &str) -> Result<(), anyhow::Error> {
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
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;]
/// name: &str - The name of the table
pub fn store_event_to_db(event: &ChronicleEvent, db_client: &mut Client, name: &str) -> Result<(), anyhow::Error> {
    let executable = format!("
        INSERT INTO {name} (address, block_number, transaction_hash, topics, data)
        VALUES ($1, $2, $3, $4, $5)
    ");
    let stringified_topics: Vec<String> = event.topics.iter().map(|topic| topic.to_string()).collect();
    let block_number = event.block_number as i64;
    db_client.execute(&executable, &[
        &event.address.to_string(),
        &block_number,
        &event.transaction_hash.to_string(),
        &stringified_topics,
        &event.data.to_vec(),
    ])?;

    Ok(())
}

/// This function would be used to get the event from the db with an filter
/// params:
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;]
/// name: &str - The name of the table
pub fn get_all_events(db_client: &mut Client, name: &str) -> Result<Vec<DisplayChronicleEvent>, anyhow::Error> {
    let mut events = Vec::new();
    let executable = format!("
        SELECT * FROM {name}
    ");
    let rows = db_client.query(&executable, &[])?;
    for row in rows {
        let address: String = row.get(0);
        let block_number: i64 = row.get(1);
        let transaction_hash: String = row.get(2);
        let topics: Vec<String> = row.get(3);
        let data: Vec<u8> = row.get(4);

        events.push(DisplayChronicleEvent::new(address, block_number, transaction_hash, topics, data));
    }

    Ok(events)
}

/// This function would be used to get the event from the db with an filter
/// params:
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls).unwrap();]
/// name: &str - The name of the table
/// filter: Vec<String> - The filter to be used [address, block_number, transaction_hash]
pub fn get_all_events_with_filter(db_client: &mut Client, name: &str, filter: Vec<String>) {
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

    todo!();
}

/// This function would be used to get the event from the db with a filter: the event hash
/// params:
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls).unwrap();]
/// name: &str - The name of the table
/// transaction_hash: String - The transaction hash
pub fn get_events_by_tx_hash(db_client: &mut Client, name: &str, transaction_hash: String) -> Result<Vec<DisplayChronicleEvent>, anyhow::Error> {
    let mut events = Vec::new();
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

        events.push(DisplayChronicleEvent::new(address, block_number, transaction_hash, topics, data));
    }

    Ok(events)
}

/// This function would be used to get the event from the db with a filter: the block number
/// params:
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls).unwrap();]
/// name: &str - The name of the table
/// block_number: i64 - The block number
pub fn get_events_by_block_number(db_client: &mut Client, name: &str, block_number: i64) -> Result<Vec<DisplayChronicleEvent>, anyhow::Error> {
    let mut events = Vec::new();
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

        events.push(DisplayChronicleEvent::new(address, block_number, transaction_hash, topics, data));
    }

    Ok(events)
}
