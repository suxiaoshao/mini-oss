use tonic::{transport::Server, Request, Response, Status};

use proto::login_server::{Login, LoginServer};
use proto::{LoginReply, LoginRequest};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Login for MyGreeter {
    async fn user_login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        let request = request.into_inner();
        let name = request.name;
        let password = request.password;
        if let (Ok(n), Ok(p)) = (
            std::env::var("manager_name"),
            std::env::var("manager_password"),
        ) {
            if name == n && password == p {}
            todo!()
        } else {
            Err(Status::new(
                tonic::Code::FailedPrecondition,
                "管理员配置缺失",
            ))
        }
    }
    async fn manager_login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:80".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(LoginServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
