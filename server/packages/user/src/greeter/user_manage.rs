use std::sync::Arc;

use database::{users::UserModal, Pool, Postgres};

use errors::{TonicError, TonicResult};
use proto::{
    async_trait,
    auth::Empty,
    core::{bucket_client::BucketClient, DeleteBucketsRequest},
    user::{
        user_manage_server::UserManage, CreateUserRequest, DeleteUserRequest, GetListRequest,
        GetUserListReply, GetUserRequest, UpdateUserRequest, UserInfo,
    },
    Request, Response, Status,
};
use validation::{check_auth::check_manager, hash::password_to_hash, validate};

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
        // 验证
        validate(request.get_ref())?;
        let CreateUserRequest {
            name,
            password,
            description,
            auth,
        } = request.into_inner();
        // 验证管理员身份
        check_manager(&auth).await?;
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
        let UpdateUserRequest {
            name,
            description,
            auth,
        } = request.into_inner();
        // 验证管理员身份
        check_manager(&auth).await?;
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
        let DeleteUserRequest { name, auth } = request.into_inner();
        // 判断该用户是否存在
        UserModal::exist(&name, &self.pool)
            .await
            .map_err(|_| TonicError::UserNotFound)?;
        // 验证管理员身份
        check_manager(&auth).await?;
        // 删除 bucket
        delete_buckets(auth, name.to_string()).await?;
        UserModal::delete(&name, &self.pool).await?;
        Ok(Response::new(Empty {}))
    }
    async fn get_user_list(
        &self,
        request: Request<GetListRequest>,
    ) -> Result<Response<GetUserListReply>, Status> {
        let GetListRequest {
            limit,
            offset,
            auth,
        } = request.into_inner();
        // 验证管理员身份
        check_manager(&auth).await?;
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
        let GetUserRequest { name, auth } = request.into_inner();
        // 验证管理员身份
        check_manager(&auth).await?;
        let user = UserModal::find_one(&name, &self.pool).await?;
        Ok(Response::new(user.into()))
    }
}

async fn delete_buckets(auth: String, name: String) -> TonicResult<()> {
    let mut client = BucketClient::connect("http://core:80").await?;
    let request = Request::new(DeleteBucketsRequest {
        auth,
        username: name.clone(),
    });
    client.delete_buckets(request).await?;
    Ok(())
}
