use async_graphql::{
    http::GraphiQLSource, EmptyMutation, EmptySubscription, ObjectType, Schema,
};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;





async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}


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
