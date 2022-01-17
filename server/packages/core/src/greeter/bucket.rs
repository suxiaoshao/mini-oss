use std::sync::Arc;

use proto::{
    async_trait,
    auth::Empty,
    core::{
        bucket_server::Bucket, Access, BucketInfo, CreateBucketRequest, DeleteBucketRequest,
        UpdateBucketRequest,
    },
    Request, Response, Status,
};
use utils::{
    database::{
        bucket::{self},
        Pool, Postgres,
    },
    validation::check_auth::check_user,
};

pub struct BucketGreeter {
    pool: Arc<Pool<Postgres>>,
}

impl BucketGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl Bucket for BucketGreeter {
    async fn create_bucket(
        &self,
        request: Request<CreateBucketRequest>,
    ) -> Result<Response<BucketInfo>, Status> {
        let CreateBucketRequest { name, access, auth } = request.into_inner();
        let user_name = check_user(&auth).await?;
        let name = format!("{name}-{user_name}");
        // 判断该存储桶是否存在
        bucket::Bucket::exist(&name, &self.pool)
            .await
            .map_err(|_| Status::already_exists("存储桶名重复"))?;
        let access: Access = Access::try_from(access)?;
        let bucket = bucket::Bucket::create(&name, access, &user_name, &self.pool).await?;
        Ok(Response::new(bucket.into()))
    }
    async fn update_bucket(
        &self,
        request: Request<UpdateBucketRequest>,
    ) -> Result<Response<BucketInfo>, Status> {
        let UpdateBucketRequest { name, access, auth } = request.into_inner();
        let user_name = check_user(&auth).await?;
        // 判断该存储桶是否存在
        bucket::Bucket::exist(&name, &self.pool)
            .await
            .map_err(|_| Status::already_exists("该存储桶不存在"))?;
        // 判断用户是否一致
        let bucket::Bucket {
            user_name: user_name_,
            ..
        } = bucket::Bucket::find_one(&name, &self.pool).await?;
        if user_name != user_name_ {
            return Err(Status::permission_denied("没有权限操作不属于你的存储桶"));
        }
        let access: bucket::Access = bucket::Access::try_from(access)?;
        let updated = bucket::Bucket::update(&name, &access, &self.pool).await?;
        Ok(Response::new(updated.into()))
    }
    async fn delete_bucket(
        &self,
        request: Request<DeleteBucketRequest>,
    ) -> Result<Response<Empty>, Status> {
        let DeleteBucketRequest { name, auth } = request.into_inner();
        let user_name = check_user(&auth).await?;
        // 判断该存储桶是否存在
        bucket::Bucket::exist(&name, &self.pool)
            .await
            .map_err(|_| Status::already_exists("该存储桶不存在"))?;
        // 判断用户是否一致
        let bucket::Bucket {
            user_name: user_name_,
            ..
        } = bucket::Bucket::find_one(&name, &self.pool).await?;
        if user_name != user_name_ {
            return Err(Status::permission_denied("没有权限操作不属于你的存储桶"));
        }
        bucket::Bucket::delete(&name, &self.pool).await?;
        Ok(Response::new(Empty {}))
    }
}
