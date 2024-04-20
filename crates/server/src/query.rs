use async_graphql::{Object, Context};
use chronicle_primitives::{db::{get_all_events, get_events_by_block_number, get_events_by_tx_hash}, indexer::DisplayChronicleEvent, ServerConfig};
use postgres::{Client, NoTls};





pub struct ChronicleQuery;




#[Object]
impl ChronicleQuery {
    async fn get_all_events<'a>(&self, cxt: &Context<'a>, name: String) -> Vec<DisplayChronicleEvent> {
        let config = cxt.data_unchecked::<ServerConfig>();
        let mut db_client = Client::connect(&config.db_url, NoTls).expect("Could not connect to the db");
        let (client, connection) =
                tokio_postgres::connect("host=localhost user=postgres", NoTls).await.expect("Could not connect to the db");
        // let events = get_all_events(&mut db_client, &name).expect("Could not get events from db");
        let events = vec![DisplayChronicleEvent::default()];



        events
    }

    // async fn get_events_by_tx_hash<'a>(&self, cxt: &Context<'a>, name: String, transaction_hash: String) -> Vec<DisplayChronicleEvent> {
    //     let config = cxt.data_unchecked::<ServerConfig>();
    //     let mut db_client = Client::connect(&config.db_url, NoTls).expect("Could not connect to the db");
    //     let events = get_events_by_tx_hash(&mut db_client, &name, transaction_hash).expect("Could not get events from db");

    //     events
    // }

    // async fn get_events_by_block_number<'a>(&self, cxt: &Context<'a>, name: String, block_number: String) -> Vec<DisplayChronicleEvent> {
    //     let config = cxt.data_unchecked::<ServerConfig>();
    //     let mut db_client = Client::connect(&config.db_url, NoTls).expect("Could not connect to the db");
    //     let events = get_events_by_block_number(&mut db_client, &name, block_number).expect("Could not get events from db");

    //     events
    // }
}
