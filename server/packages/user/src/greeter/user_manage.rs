use std::sync::Arc;

use database::{users::UserModal, Pool, Postgres};
use errors::{TonicError, TonicResult};
use proto::middleware::client::bucket_client;
use proto::user::CountReply;
use proto::{
    async_trait,
    core::DeleteBucketsRequest,
    user::Empty,
    user::{
        user_manage_server::UserManage, CreateUserRequest, DeleteUserRequest, GetListRequest,
        GetUserListReply, GetUserRequest, UpdateUserRequest, UserInfo,
    },
    Request, Response, Status,
};
use validation::{check_auth::check_manager, TonicValidate};

use crate::utils::hash::password_to_hash;

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
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        // 验证
        request.get_ref().validate()?;
        let CreateUserRequest {
            name,
            password,
            description,
        } = request.into_inner();

        // 验证管理员身份
        check_manager(auth).await?;
        // 判断该用户是否存在
        if UserModal::exist(&name, &self.pool).await.is_ok() {
            return Err(Status::already_exists("用户名重复"));
        }
        let user = UserModal::create(
            &name,
            &password_to_hash(&password)?,
            &description,
            &self.pool,
        )
        .await?;
        Ok(Response::new(user.into()))
    }
    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let UpdateUserRequest { name, description } = request.into_inner();
        // 验证管理员身份
        check_manager(auth).await?;
        // 判断该用户是否存在
        UserModal::exist(&name, &self.pool)
            .await
            .map_err(|_| TonicError::UserNotFound)?;
        let user = UserModal::update(&name, &description, &self.pool).await?;
        Ok(Response::new(user.into()))
    }
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<Empty>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let DeleteUserRequest { name } = request.into_inner();
        // 判断该用户是否存在
        UserModal::exist(&name, &self.pool)
            .await
            .map_err(|_| TonicError::UserNotFound)?;
        // 验证管理员身份
        check_manager(auth.clone()).await?;
        // 删除 bucket
        delete_buckets(auth, name.to_string()).await?;
        UserModal::delete(&name, &self.pool).await?;
        Ok(Response::new(Empty {}))
    }
    async fn get_user_list(
        &self,
        request: Request<GetListRequest>,
    ) -> Result<Response<GetUserListReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let GetListRequest { limit, offset } = request.into_inner();
        // 验证管理员身份
        check_manager(auth).await?;
        let limit = &limit;
        let limit = if limit > &50 { &50 } else { limit };
        let offset = &offset;
        let (users, count) = tokio::join!(
            UserModal::find_many(*limit, *offset, &self.pool),
            UserModal::count(&self.pool)
        );
        Ok(Response::new(GetUserListReply {
            data: users?.into_iter().map(|x| x.into()).collect(),
            total: count?,
        }))
    }
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let GetUserRequest { name } = request.into_inner();
        // 验证管理员身份
        check_manager(auth).await?;
        let user = UserModal::find_one(&name, &self.pool).await?;
        Ok(Response::new(user.into()))
    }

    async fn get_count(&self, request: Request<Empty>) -> Result<Response<CountReply>, Status> {
        let pool = &self.pool;
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        // 验证管理员身份
        check_manager(auth).await?;
        let total = UserModal::count(pool).await?;
        Ok(Response::new(CountReply { total }))
    }
}

async fn delete_buckets(auth: Option<String>, name: String) -> TonicResult<()> {
    let mut client = bucket_client(auth).await?;
    let request = Request::new(DeleteBucketsRequest {
        username: name.clone(),
    });
    client.delete_buckets(request).await?;
    Ok(())
}
