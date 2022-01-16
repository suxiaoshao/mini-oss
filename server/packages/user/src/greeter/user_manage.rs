use std::sync::Arc;

use proto::{
    async_trait,
    auth::Empty,
    user::{
        user_manage_server::UserManage, CreateUserRequest, DeleteUserRequest, GetUserRequest,
        ListRequest, ListUserReply, UpdateUserRequest, UserInfo,
    },
    Request, Response, Status,
};
use utils::{
    database::{users::User, Pool, Postgres},
    validation::{check_auth::check_manager, hash::to_hash},
};

pub struct UserManageGreeter {
    pool: Arc<Pool<Postgres>>,
}

impl UserManageGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
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
    async fn list_user(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<ListUserReply>, Status> {
        // 验证管理员身份
        check_manager(&request.get_ref().auth).await?;
        let limit = &request.get_ref().limit;
        let limit = if limit > &50 { &50 } else { limit };
        let offset = &request.get_ref().offset;
        let users = User::find_many(*limit, *offset, &self.pool).await?;
        let count = User::count(&self.pool).await?;
        Ok(Response::new(ListUserReply {
            data: users.into_iter().map(|x| x.into()).collect(),
            total: count,
        }))
    }
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        // 验证管理员身份
        check_manager(&request.get_ref().auth).await?;
        let user = User::find_one(&request.get_ref().name, &self.pool).await?;
        Ok(Response::new(user.into()))
    }
}
