use std::sync::Arc;

use ::utils::database::PgPoolOptions;
use proto::{
    core::{bucket_server::BucketServer, folder_server::FolderServer},
    Server,
};
use utils::mongo::Mongo;

use crate::greeter::{bucket::BucketGreeter, folder::FolderGreeter};

mod greeter;

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
    let folder_greeter = FolderGreeter::new(Arc::clone(&pool));
    println!("GreeterServer listening on {addr}");

    Server::builder()
        .add_service(BucketServer::new(bucket_greeter))
        .add_service(FolderServer::new(folder_greeter))
        .serve(addr)
        .await?;

    Ok(())
}
