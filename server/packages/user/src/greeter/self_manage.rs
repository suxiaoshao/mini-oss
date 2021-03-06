use std::sync::Arc;

use database::{users::UserModal, Pool, Postgres};
use errors::TonicError;
use proto::user::Empty;
use proto::{
    async_trait,
    user::{
        self_manage_server::SelfManage, LoginReply, UpdatePasswordRequest, UpdateUserInfoRequest,
        UserInfo,
    },
    Request, Response, Status,
};
use validation::{check_auth::check_user, TonicValidate};

use crate::utils::{
    claims::Claims,
    hash::{password_to_hash, validate_password_hash},
};
pub struct SelfManageGreeter {
    pool: Arc<Pool<Postgres>>,
}

impl SelfManageGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl SelfManage for SelfManageGreeter {
    async fn update_user_info(
        &self,
        request: Request<UpdateUserInfoRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        // 验证用户身份
        let name = check_user(auth).await?;
        let user = UserModal::update(&name, &request.get_ref().description, &self.pool).await?;
        Ok(Response::new(user.into()))
    }
    async fn get_user_info(&self, request: Request<Empty>) -> Result<Response<UserInfo>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        // 验证用户身份
        let name = check_user(auth).await?;
        let user = UserModal::find_one(&name, &self.pool).await?;
        Ok(Response::new(user.into()))
    }
    async fn update_password(
        &self,
        request: Request<UpdatePasswordRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        // 获取 auth
        let auth = request
            .extensions()
            .get::<String>()
            .cloned()
            .ok_or(TonicError::NoneAuth)?;
        // 验证
        request.get_ref().validate()?;
        let UpdatePasswordRequest {
            old_password,
            new_password,
        } = request.into_inner();
        // 验证用户身份
        let user = Claims::check_user(auth, &self.pool).await?;
        // 验证旧密码
        if validate_password_hash(&old_password, &user.password).is_err() {
            return Err(Status::invalid_argument("旧密码错误"));
        }
        // 更新密码
        UserModal::update_password(&user.name, &password_to_hash(&new_password)?, &self.pool)
            .await?;
        // 生成 token
        let token = Claims::new_token(user.name, new_password)?;
        Ok(Response::new(LoginReply { auth: token }))
    }
}
