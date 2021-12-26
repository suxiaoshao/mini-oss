use proto::user_manage::{
    manage_server::Manage, Empty, UserCreateInfo, UserDeleteInfo, UserInfo, UserUpdateInfo,
};
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::utils::check_manager;

pub struct UserManageGreeter {
    pool: Pool<Postgres>,
}

impl UserManageGreeter {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }
}
#[tonic::async_trait]
impl Manage for UserManageGreeter {
    async fn user_create(
        &self,
        request: Request<UserCreateInfo>,
    ) -> Result<Response<UserInfo>, Status> {
        // 验证管理员身份
        check_manager(&request.get_ref().auth).await?;
        todo!()
    }
    async fn user_update(
        &self,
        request: Request<UserUpdateInfo>,
    ) -> Result<Response<UserInfo>, Status> {
        // 验证管理员身份
        check_manager(&request.get_ref().auth).await?;
        todo!()
    }
    async fn user_delete(
        &self,
        request: Request<UserDeleteInfo>,
    ) -> Result<Response<Empty>, Status> {
        // 验证管理员身份
        check_manager(&request.get_ref().auth).await?;
        todo!()
    }
}
