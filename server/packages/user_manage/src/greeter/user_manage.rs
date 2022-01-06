use database::{users::User, Pool, Postgres};
use proto::{
    async_trait,
    auth::Empty,
    user_manage::{
        user_manage_server::UserManage, CreateUserRequest, DeleteUserRequest, UpdateUserRequest,
        UserInfo,
    },
    Request, Response, Status,
};

use crate::utils::{check_manager, to_hash};

pub struct UserManageGreeter {
    pool: Pool<Postgres>,
}

impl UserManageGreeter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl UserManage for UserManageGreeter {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        // 验证管理员身份
        check_manager(&request.get_ref().auth).await?;
        let user = User::create(
            &request.get_ref().name,
            &to_hash(&request.get_ref().password)?,
            &request.get_ref().description,
            &self.pool,
        )
        .await?;
        Ok(Response::new(user.into()))
    }
    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        // 验证管理员身份
        check_manager(&request.get_ref().auth).await?;
        let user = User::update(
            &request.get_ref().name,
            &request.get_ref().description,
            &self.pool,
        )
        .await?;
        Ok(Response::new(user.into()))
    }
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<Empty>, Status> {
        // 验证管理员身份
        check_manager(&request.get_ref().auth).await?;
        User::delete(&request.get_ref().name, &self.pool).await?;
        Ok(Response::new(Empty {}))
    }
}
