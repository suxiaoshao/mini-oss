use std::sync::Arc;

use database::PgPoolOptions;
use proto::{
    user::{
        self_manage_server::SelfManageServer, user_manage_server::UserManageServer,
        user_message_server::UserMessageServer,
    },
    Server,
};

use crate::greeter::{
    self_manage::SelfManageGreeter, user_manage::UserManageGreeter,
    user_message::UserMessageGreeter,
};

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

    let user_manage_greeter = UserManageGreeter::new(Arc::clone(&pool));
    let self_manage_greeter = SelfManageGreeter::new(Arc::clone(&pool));
    let user_message_greeter = UserMessageGreeter::new(Arc::clone(&pool));
    println!("GreeterServer listening on {addr}");

    Server::builder()
        .add_service(UserManageServer::new(user_manage_greeter))
        .add_service(UserMessageServer::new(user_message_greeter))
        .add_service(SelfManageServer::new(self_manage_greeter))
        .serve(addr)
        .await?;

    Ok(())
}
