use std::sync::Arc;

use database::{users::User, Pool, Postgres};
use proto::{
    async_trait,
    user::{self_manage_server::SelfManage, GetUserInfoRequest, UpdateUserInfoRequest, UserInfo},
    Request, Response, Status,
};

use crate::utils::check_user;

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
        // 验证管理员身份
        let name = check_user(&request.get_ref().auth).await?;
        let user = User::update(&name, &request.get_ref().description, &self.pool).await?;
        Ok(Response::new(user.into()))
    }
    async fn get_user_info(
        &self,
        request: Request<GetUserInfoRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        // 验证管理员身份
        let name = check_user(&request.get_ref().auth).await?;
        let user = User::find_one(&name, &self.pool).await?;
        Ok(Response::new(user.into()))
    }
}
