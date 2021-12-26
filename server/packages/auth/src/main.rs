mod greeter;
mod utils;
mod validation;

use crate::greeter::{check::CheckGreeter, login::LoginGreeter};
use database::PgPoolOptions;
use proto::{
    auth::{check_server::CheckServer, login_server::LoginServer},
    Server,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:80".parse().unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("postgres").unwrap())
        .await
        .unwrap();
    let login_greeter = LoginGreeter::new(pool.clone());
    let check_greeter = CheckGreeter::new(pool);

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(LoginServer::new(login_greeter))
        .add_service(CheckServer::new(check_greeter))
        .serve(addr)
        .await?;

    Ok(())
}
