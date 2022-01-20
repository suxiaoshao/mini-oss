use proto::{
    async_trait,
    auth::{login_server::Login, LoginReply, LoginRequest},
    validation::Validate,
    Request, Response, Status,
};
use std::sync::Arc;
use utils::{
    database::{Pool, Postgres},
    errors::grpc::ToStatusResult,
    validation::claims::Claims,
};

pub struct LoginGreeter {
    pool: Arc<Pool<Postgres>>,
}

impl LoginGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Login for LoginGreeter {
    async fn user_login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        // 验证
        request.get_ref().validate().to_status()?;
        let request = request.into_inner();
        let name = request.name;
        let password = request.password;
        let token = Claims::user_token(name, password, &self.pool).await?;
        Ok(Response::new(LoginReply { auth: token }))
    }
    async fn manager_login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        // 验证
        request.get_ref().validate().to_status()?;
        let request = request.into_inner();
        let name = request.name;
        let password = request.password;
        let token = Claims::manager_token(name, password)?;
        Ok(Response::new(LoginReply { auth: token }))
    }
}

#[tokio::test]
async fn manager_login() {
    use proto::auth::login_client::LoginClient;
    let mut client = LoginClient::connect("http://localhost:80").await.unwrap();
    let request = Request::new(LoginRequest {
        name: "sushao".to_string(),
        password: "sushao".to_string(),
    });
    let res = client.user_login(request).await.unwrap();
    println!("{}", res.get_ref().auth);
}
