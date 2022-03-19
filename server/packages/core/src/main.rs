use std::sync::Arc;

use anyhow::Result;
use database::PgPoolOptions;
use proto::core::request_server::RequestServer;
use proto::middleware::server::{add_auth, interceptor};
use proto::{
    core::{bucket_server::BucketServer, folder_server::FolderServer, object_server::ObjectServer},
    transport::Server,
};
use proto::core::storage_server::StorageServer;

use crate::greeter::request::RequestGreeter;
use crate::{
    greeter::{bucket::BucketGreeter, folder::FolderGreeter, object::ObjectGreeter},
    utils::mongo::Mongo,
};
use crate::greeter::storage::StorageGreeter;

mod greeter;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:80".parse()?;

    // 获取数据库连接池
    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("postgres")?)
            .await?,
    );
    let mongo = Arc::new(Mongo::new(&std::env::var("mongodb")?).await?);

    let bucket_greeter = BucketGreeter::new(Arc::clone(&pool), Arc::clone(&mongo));
    let folder_greeter = FolderGreeter::new(Arc::clone(&pool), Arc::clone(&mongo));
    let object_greeter = ObjectGreeter::new(Arc::clone(&pool), Arc::clone(&mongo));
    let request_greeter = RequestGreeter::new(Arc::clone(&pool));
    let storage_greeter = StorageGreeter::new(Arc::clone(&pool));
    println!("GreeterServer listening on {addr}");

    Server::builder()
        .layer(interceptor(add_auth))
        .add_service(BucketServer::new(bucket_greeter))
        .add_service(FolderServer::new(folder_greeter))
        .add_service(ObjectServer::new(object_greeter))
        .add_service(RequestServer::new(request_greeter))
        .add_service(StorageServer::new(storage_greeter))
        .serve(addr)
        .await?;

    Ok(())
}
