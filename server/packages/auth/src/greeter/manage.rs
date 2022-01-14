use std::sync::Arc;

use database::{users::User, Pool, Postgres};
use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Pbkdf2,
};
use proto::{
    async_trait,
    auth::{manage_server::Manage, Empty, LoginReply, UpdatePasswordRequest},
    Request, Response, Status,
};
use utils::errors::grpc::ToStatusResult;

use crate::validation::Claims;

pub struct ManageGreeter {
    pool: Arc<Pool<Postgres>>,
}

impl ManageGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl Manage for ManageGreeter {
    async fn update_password(
        &self,
        request: Request<UpdatePasswordRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        let UpdatePasswordRequest {
            old_password,
            new_password,
            auth,
        } = request.into_inner();
        // 验证管理员身份
        let name = Claims::check_user(auth, &self.pool).await?;
        // 验证旧密码
        let user = User::find_one(&name, &self.pool).await?;
        let parsed_hash = PasswordHash::new(&user.password).to_status()?;
        if Pbkdf2
            .verify_password(old_password.as_bytes(), &parsed_hash)
            .is_err()
        {
            return Err(Status::invalid_argument("旧密码错误"));
        }
        User::update_password(&name, &new_password, &self.pool).await?;
        let token = Claims::user_token(name, new_password, &self.pool).await?;
        Ok(Response::new(LoginReply { auth: token }))
    }
}
