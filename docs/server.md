# Chronicle-GraphQL
-----------------------------
This is the interface of the Chronicle, it is responsible for rendering the indexed data in a GraphQL interface. This is referred to as `chronicle-server`.
This crate is built with [Async GraphQL](https://github.com/async-graphql/async-graphql), it reads from the Postgres DB and renders the data in a GraphQL interface, using the GraphQL schema defined in the `chronicle-server` crate.

This core function responsible for rendering the data in a GraphQL interface is the `graphql` function, it is located in the `src/lib.rs` file.

```rust
/// This function is used to run the chronicle server.
/// `[DB]` This is a generic type, which is used to store the database.
/// `[Query]` This is a gaint Query entity, for all the Events enitities and all the tx enitities.
pub async fn run_chronicle_server<Query>(
    config: ServerConfig,
    query: Query,
) -> Result<(), anyhow::Error>
where
    Query: ObjectType + 'static,
{
    let url = config.server_url.clone();
    let schema = Schema::build(query, EmptyMutation, EmptySubscription)
        .data(config)
        .finish();
    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    tracing::info!(url);
    axum::serve(TcpListener::bind(url).await.unwrap(), app)
        .await
        .unwrap();

    Ok(())
}

```

This returns a `Future` that is Encapsulated into a task and the task manager the binary crate `Chronicle` is responsible for running the `chronicle-server` concurrently with the `chronicle-indexer`. These tasks also listen for the `control-c` key to gracefully shutdown the Chronicle.


An interesting part of the `chronicle-server` is the `graphql` `Query` entity, this entity is a giant Query entity, for all the Events entities and all the tx entities. This entity is responsible for running the `chronicle-server` and it is located in the `src/query.rs` file.

```rust
#[Object]
impl ChronicleQuery {
    async fn get_all_events<'a>(
        &self,
        cxt: &Context<'a>,
        name: String,
    ) -> Vec<DisplayChronicleEvent> {
        let config = cxt.data_unchecked::<ServerConfig>();
        let mut db_client = create_db_instance(&config.db_url)
            .await
            .expect("Could not connect to the db");
        let events = get_all_events(&mut db_client, &name)
            .await
            .expect("Could not get events from db");

        events
    }

    async fn get_events_by_tx_hash<'a>(
        &self,
        cxt: &Context<'a>,
        name: String,
        transaction_hash: String,
    ) -> Vec<DisplayChronicleEvent> {
        let config = cxt.data_unchecked::<ServerConfig>();
        let mut db_client = create_db_instance(&config.db_url)
            .await
            .expect("Could not connect to the db");
        let events = get_events_by_tx_hash(&mut db_client, &name, transaction_hash)
            .await
            .expect("Could not get events from db");

        events
    }

    async fn get_events_by_block_number<'a>(
        &self,
        cxt: &Context<'a>,
        name: String,
        block_number: String,
    ) -> Vec<DisplayChronicleEvent> {
        let config = cxt.data_unchecked::<ServerConfig>();
        let mut db_client = create_db_instance(&config.db_url)
            .await
            .expect("Could not connect to the db");
        let events = get_events_by_block_number(&mut db_client, &name, block_number)
            .await
            .expect("Could not get events from db");

        events
    }
}


```
