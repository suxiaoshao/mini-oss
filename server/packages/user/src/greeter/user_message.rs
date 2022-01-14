use std::sync::Arc;

use proto::{
    async_trait,
    user::{
        user_message_server::UserMessage, GetUserRequest, ListRequest, ListUserReply, UserInfo,
    },
    Request, Response, Status,
};
use utils::database::{users::User, Pool, Postgres};

use crate::utils::check_manager;

pub struct UserMessageGreeter {
    pool: Arc<Pool<Postgres>>,
}

impl UserMessageGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl UserMessage for UserMessageGreeter {
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
