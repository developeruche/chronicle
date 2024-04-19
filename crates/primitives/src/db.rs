use crate::indexer::{ChronicleEvent, DisplayChronicleEvent};
use postgres::Client;





/// This function would be used to store the event to the db
/// params:
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;]
/// name: &str - The name of the table
pub fn create_new_event_db_table(db_client: &mut Client, name: &str) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            CREATE TABLE IF NOT EXISTS {name} (
                id              SERIAL PRIMARY KEY,
                address         VARCHAR NULL,
                block_number    VARCHAR NULL,
                transaction_hash VARCHAR NULL,
                topics          VARCHAR NULL,
                data            BYTEA NULL
            )
        "
    );
    db_client.batch_execute(&executable)?;

    Ok(())
}

/// This function would be used to store the event to the db
/// params:
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;]
/// name: &str - The name of the table
pub fn store_event_to_db(
    event: &ChronicleEvent,
    db_client: &mut Client,
    name: &str,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            INSERT INTO {name} (address, block_number, transaction_hash, topics, data)
            VALUES ($1, $2, $3, $4, $5)
        "
    );
    let stringified_topics: String =
        event.topics.iter().map(|topic| topic.to_string()).collect::<Vec<String>>().join(", ");
    db_client.execute(
        &executable,
        &[
            &event.address.to_string(),
            &event.block_number.to_string(),
            &event.transaction_hash.to_string(),
            &stringified_topics,
            &event.data.to_vec(),
        ],
    )?;

    Ok(())
}

/// This function would be used to get the event from the db with an filter
/// params:
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;]
/// name: &str - The name of the table
pub fn get_all_events(
    db_client: &mut Client,
    name: &str,
) -> Result<Vec<DisplayChronicleEvent>, anyhow::Error> {
    let mut events = Vec::new();
    let executable = format!(
        "
            SELECT * FROM {name}
        "
    );
    let rows = db_client.query(&executable, &[])?;
    for row in rows {
        let address: String = row.get(1);
        let block_number: String = row.get(2);
        let transaction_hash: String = row.get(3);
        let topics: String = row.get(4);
        let topics: Vec<String> = topics.split(',').map(String::from).collect();
        let data: Vec<u8> = row.get(5);

        events.push(DisplayChronicleEvent::new(
            address,
            block_number,
            transaction_hash,
            topics,
            data,
        ));
    }

    Ok(events)
}

/// This function would be used to get the event from the db with an filter
/// params:
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls);]
/// name: &str - The name of the table
/// filter: Vec<String> - The filter to be used [address, block_number, transaction_hash]
pub fn get_all_events_with_filter(db_client: &mut Client, name: &str, filter: Vec<String>) -> Result<(), anyhow::Error> {
    let filter_decoded = filter.join(", ");
    let executable = format!(
        "
            SELECT {filter_decoded} FROM {name}
        "
    );
    let rows = db_client.query(&executable, &[])?;
    for row in rows {
        let address: String = row.get(0);
        let block_number: String = row.get(1);
        let transaction_hash: String = row.get(2);
        let topics: String = row.get(3);
        let topics: Vec<String> = topics.split(',').map(String::from).collect();
        let data: Vec<u8> = row.get(4);

        println!(
            "address: {}, block_number: {}, transaction_hash: {}, topics: {:?}, data: {:?}",
            address, block_number, transaction_hash, topics, data
        );
    }

    // todo!();
    Ok(())
}

/// This function would be used to get the event from the db with a filter: the event hash
/// params:
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls);]
/// name: &str - The name of the table
/// transaction_hash: String - The transaction hash
pub fn get_events_by_tx_hash(
    db_client: &mut Client,
    name: &str,
    transaction_hash: String,
) -> Result<Vec<DisplayChronicleEvent>, anyhow::Error> {
    let mut events = Vec::new();
    let executable = format!(
        "
        SELECT * FROM {name} WHERE transaction_hash = $1
    "
    );
    let rows = db_client.query(&executable, &[&transaction_hash])?;
    for row in rows {
        let address: String = row.get(1);
        let block_number: String = row.get(2);
        let transaction_hash: String = row.get(3);
        let topics: String = row.get(4);
        let topics: Vec<String> = topics.split(',').map(String::from).collect();
        let data: Vec<u8> = row.get(5);

        events.push(DisplayChronicleEvent::new(
            address,
            block_number,
            transaction_hash,
            topics,
            data,
        ));
    }

    Ok(events)
}

/// This function would be used to get the event from the db with a filter: the block number
/// params:
/// db_client: &mut Client - The db client [let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls);]
/// name: &str - The name of the table
/// block_number: i64 - The block number
pub fn get_events_by_block_number(
    db_client: &mut Client,
    name: &str,
    block_number: String,
) -> Result<Vec<DisplayChronicleEvent>, anyhow::Error> {
    let mut events = Vec::new();
    let executable = format!(
        "
            SELECT * FROM {name} WHERE block_number = $1
        "
    );
    let rows = db_client.query(&executable, &[&block_number])?;
    for row in rows {
        let address: String = row.get(1);
        let block_number: String = row.get(2);
        let transaction_hash: String = row.get(3);
        let topics: String = row.get(4);
        let topics: Vec<String> = topics.split(',').map(String::from).collect();
        let data: Vec<u8> = row.get(5);

        events.push(DisplayChronicleEvent::new(
            address,
            block_number,
            transaction_hash,
            topics,
            data,
        ));
    }

    Ok(events)
}





#[cfg(test)]
pub mod tests {
    use alloy::primitives::{address, b256, Bytes};
    use postgres::NoTls;
    use super::*;

    const DB_URL: &str = "postgresql://postgres:postgres@localhost:5432/user";
    const NAME: &str = "events_2";


    #[test]
    #[ignore]
    pub fn test_can_create_db_table_for_event() {
        let mut client = Client::connect(DB_URL, NoTls)
            .expect("Could not connect to the db");

        let result = create_new_event_db_table(&mut client, NAME);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore]
    pub fn test_should_store_event_to_db() {
        let mut client = Client::connect(DB_URL, NoTls)
            .expect("Could not connect to the db");

        let demo_event = ChronicleEvent {
            address: address!("88da6bf26964af9d7eed9e03e53415d37aa96045"),
            block_number: 5,
            transaction_hash:  b256!("000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045"),
            topics: vec![b256!("000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045")],
            data: Bytes::from_static(&[0x69])
        };

        let store_event_result = store_event_to_db(&demo_event, &mut client, NAME);

        assert!(store_event_result.is_ok());
    }

    #[test]
    #[ignore]
    pub fn test_should_successfully_read_from_db() {
        let mut client = Client::connect(DB_URL, NoTls)
            .expect("Could not connect to the db");
        let get_event_result = get_all_events(&mut client, NAME).unwrap();

        for row in get_event_result {
            println!("Working: {:?}", row)
        }
    }

    #[test]
    #[ignore]
    pub fn test_should_successfully_read_from_db_with_filter() {
        let mut client = Client::connect(DB_URL, NoTls)
            .expect("Could not connect to the db");
        let filter = vec!["address".to_string(), "block_number".to_string(), "transaction_hash".to_string(), "topics".to_string(), "data".to_string()];
        get_all_events_with_filter(&mut client, NAME, filter).unwrap();
    }

    #[test]
    #[ignore]
    pub fn test_should_successfully_read_from_db_with_filter_by_tx_hash() {
        let mut client = Client::connect(DB_URL, NoTls)
            .expect("Could not connect to the db");
        let filter = "0x000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045".to_string();
        let result = get_events_by_tx_hash(&mut client, NAME, filter).unwrap();

        for row in result {
            println!("Working: {:?}", row)
        }
    }


    #[test]
    pub fn test_should_successfully_read_from_db_with_filter_by_block_number() {
        let mut client = Client::connect(DB_URL, NoTls)
            .expect("Could not connect to the db");
        let filter = "5".to_string();
        let result = get_events_by_block_number(&mut client, NAME, filter).unwrap();

        for row in result {
            println!("Working: {:?}", row)
        }
    }
}
