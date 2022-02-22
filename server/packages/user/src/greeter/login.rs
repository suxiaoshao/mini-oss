use database::{users::UserModal, Pool, Postgres};
use proto::{
    async_trait,
    user::{login_server::Login, CheckReply, CheckRequest, Empty, LoginReply, LoginRequest},
    Request, Response, Status,
};
use std::sync::Arc;
use validation::validate;

use crate::utils::claims::Claims;

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
        validate(request.get_ref())?;
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
        validate(request.get_ref())?;
        let request = request.into_inner();
        let name = request.name;
        let password = request.password;
        let token = Claims::manager_token(name, password)?;
        Ok(Response::new(LoginReply { auth: token }))
    }
    async fn check_manager(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<Empty>, Status> {
        let auth = request.into_inner().auth;
        Claims::check_manager(auth)?;
        Ok(Response::new(Empty {}))
    }
    async fn check_user(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckReply>, Status> {
        let auth = request.into_inner().auth;
        let UserModal { name, .. } = Claims::check_user(auth, &self.pool).await?;
        Ok(Response::new(CheckReply { name }))
    }
}
