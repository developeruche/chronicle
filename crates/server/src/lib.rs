use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, ObjectType, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;

/// This function is used to serve the graphQL server and GraphiQL IDE.
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

/// This function is used to run the chronicle server.
/// `[DB]` This is a generic type, which is used to store the database.
/// `[Query]` This is a gaint Query entity, for all the Events enitities and all the tx enitities.
pub async fn run_chronicle_server<DB, Query>(
    url: String,
    db: DB,
    query: Query,
) -> Result<(), anyhow::Error>
where
    Query: ObjectType + 'static,
    DB: Send + Sync + 'static,
{
    let schema = Schema::build(query, EmptyMutation, EmptySubscription)
        .data(db)
        .finish();
    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    tracing::info!(url);
    axum::serve(TcpListener::bind(url).await.unwrap(), app)
        .await
        .unwrap();

    Ok(())
}
