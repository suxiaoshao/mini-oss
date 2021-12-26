use proto::{
    auth::Empty,
    user_manage::{
        manage_server::Manage, UserCreateInfo, UserDeleteInfo, UserInfo, UserUpdateInfo,
    },
};
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::{database::User, utils::check_manager};

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
        let User {
            name,
            password,
            create_time,
            update_time,
            description,
        } = User::create(
            &request.get_ref().name,
            &request.get_ref().password,
            &request.get_ref().description,
            &self.pool,
        )
        .await?;
        let create_time = create_time.assume_utc().unix_timestamp();
        let update_time = update_time.assume_utc().unix_timestamp();
        Ok(Response::new(UserInfo {
            name,
            password,
            description,
            create_time,
            update_time,
        }))
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
