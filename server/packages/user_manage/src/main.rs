use proto::user_manage::manage_server::ManageServer;
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

use crate::greeter::UserManageGreeter;

mod database;
mod greeter;
mod utils;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:80".parse().unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("postgres").unwrap())
        .await
        .unwrap();
    let greeter = UserManageGreeter::new(pool);

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(ManageServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
