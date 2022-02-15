mod schema;
mod utils;
mod errors;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::{self};
use axum::response::{self, IntoResponse};
use axum::routing::{get, post};
use axum::{AddExtensionLayer, Router, Server};
use schema::RootSchema;

use crate::schema::root::{MutationRoot, QueryRoot};
use crate::utils::cors::get_cors;
use crate::utils::get_local_ip::get_local_ip;

async fn graphql_handler(
    schema: extract::Extension<RootSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

#[tokio::main]
async fn main() {
    // 设置跨域
    let cors = get_cors();
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/graphql", post(graphql_handler))
        .layer(AddExtensionLayer::new(schema))
        .layer(cors);

    // 打印 ip
    let ip = get_local_ip().unwrap();
    println!("Playground: http://{ip}:80");

    Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
