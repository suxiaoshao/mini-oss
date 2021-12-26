use database::{users::User, Pool, Postgres};
use proto::{
    async_trait,
    auth::Empty,
    user_manage::{
        manage_server::Manage, UserCreateInfo, UserDeleteInfo, UserInfo, UserUpdateInfo,
    },
    Request, Response, Status,
};

use crate::utils::check_manager;

pub struct UserManageGreeter {
    pool: Pool<Postgres>,
}

impl UserManageGreeter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl Manage for UserManageGreeter {
    async fn user_create(
        &self,
        request: Request<UserCreateInfo>,
    ) -> Result<Response<UserInfo>, Status> {
        // 验证管理员身份
        check_manager(&request.get_ref().auth).await?;
        let user = User::create(
            &request.get_ref().name,
            &request.get_ref().password,
            &request.get_ref().description,
            &self.pool,
        )
        .await?;
        Ok(Response::new(user.into()))
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
