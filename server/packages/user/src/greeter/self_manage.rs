use std::sync::Arc;

use proto::{
    async_trait,
    auth::LoginReply,
    user::{
        self_manage_server::SelfManage, GetUserInfoRequest, UpdatePasswordRequest,
        UpdateUserInfoRequest, UserInfo,
    },
    validation::Validate,
    Request, Response, Status,
};
use utils::{
    database::{users::User, Pool, Postgres},
    errors::grpc::ToStatusResult,
    validation::{
        check_auth::check_user,
        claims::Claims,
        hash::{to_hash, validate_hash},
    },
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
        // 验证用户身份
        let name = check_user(&request.get_ref().auth).await?;
        let user = User::update(&name, &request.get_ref().description, &self.pool).await?;
        Ok(Response::new(user.into()))
    }
    async fn get_user_info(
        &self,
        request: Request<GetUserInfoRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        // 验证用户身份
        let name = check_user(&request.get_ref().auth).await?;
        let user = User::find_one(&name, &self.pool).await?;
        Ok(Response::new(user.into()))
    }
    async fn update_password(
        &self,
        request: Request<UpdatePasswordRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        // 验证
        request.get_ref().validate().to_status()?;
        let UpdatePasswordRequest {
            old_password,
            new_password,
            auth,
        } = request.into_inner();
        // 验证用户身份
        let user = Claims::check_user(auth, &self.pool).await?;
        // 验证旧密码
        if validate_hash(&old_password, &user.password).is_err() {
            return Err(Status::invalid_argument("旧密码错误"));
        }
        // 更新密码
        User::update_password(&user.name, &to_hash(&new_password)?, &self.pool).await?;
        // 生成 token
        let token = Claims::new_token(user.name, new_password)?;
        Ok(Response::new(LoginReply { auth: token }))
    }
}
