mod database;
mod greeter;
mod utils;
mod validation;

use crate::greeter::MyGreeter;
use proto::auth::login_server::LoginServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:80".parse().unwrap();
    let greeter = MyGreeter::default().await;

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(LoginServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
