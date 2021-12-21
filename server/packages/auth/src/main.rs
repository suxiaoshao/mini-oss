mod validation;
use tonic::{transport::Server, Request, Response, Status};

use proto::login_server::{Login, LoginServer};
use proto::{LoginReply, LoginRequest};
use validation::Claims;

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
        let token = Claims::user_token(name, password)?;
        Ok(Response::new(LoginReply { auth: token }))
    }
    async fn manager_login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        let request = request.into_inner();
        let name = request.name;
        let password = request.password;
        let token = Claims::manager_token(name, password)?;
        Ok(Response::new(LoginReply { auth: token }))
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
