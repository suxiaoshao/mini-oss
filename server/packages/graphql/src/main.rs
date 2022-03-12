use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::{self, Extension};
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use axum::response::{self, IntoResponse};
use axum::routing::{get, post};
use axum::{Router, Server};

use schema::RootSchema;

use crate::schema::root::{MutationRoot, QueryRoot};
use crate::utils::cors::get_cors;
use anyhow::Result;

mod errors;
mod schema;
mod utils;

async fn graphql_handler(
    schema: extract::Extension<RootSchema>,
    req: GraphQLRequest,
    header: HeaderMap,
) -> GraphQLResponse {
    match header
        .get(AUTHORIZATION)
        .and_then(|x| x.to_str().ok())
        .map(|x| x.to_string())
    {
        None => schema.execute(req.into_inner()).await.into(),
        Some(auth) => schema.execute(req.into_inner().data(auth)).await.into(),
    }
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

#[tokio::main]
async fn main() -> Result<()> {
    // 设置跨域
    let cors = get_cors();
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors);

    println!("Playground: http://graphql:80");

    Server::bind(&"0.0.0.0:80".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
