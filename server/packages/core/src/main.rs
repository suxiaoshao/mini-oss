use std::sync::Arc;

use ::utils::database::PgPoolOptions;
use ::utils::mongo::Mongo;
use proto::{
    core::{bucket_server::BucketServer, folder_server::FolderServer, object_server::ObjectServer},
    transport::Server,
};

use crate::greeter::{bucket::BucketGreeter, folder::FolderGreeter, object::ObjectGreeter};

mod greeter;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:80".parse().unwrap();

    // 获取数据库连接池
    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("postgres").unwrap())
            .await
            .unwrap(),
    );
    let mongo = Arc::new(
        Mongo::new(&std::env::var("mongodb").unwrap())
            .await
            .unwrap(),
    );

    let bucket_greeter = BucketGreeter::new(Arc::clone(&pool), Arc::clone(&mongo));
    let folder_greeter = FolderGreeter::new(Arc::clone(&pool), Arc::clone(&mongo));
    let object_greeter = ObjectGreeter::new(Arc::clone(&pool), Arc::clone(&mongo));
    println!("GreeterServer listening on {addr}");

    Server::builder()
        .add_service(BucketServer::new(bucket_greeter))
        .add_service(FolderServer::new(folder_greeter))
        .add_service(ObjectServer::new(object_greeter))
        .serve(addr)
        .await?;

    Ok(())
}
