use std::sync::Arc;

use proto::{
    async_trait,
    auth::Empty,
    core::{
        bucket_server::Bucket, Access, BucketInfo, CreateBucketRequest, DeleteBucketRequest,
        GetBucketListReply, UpdateBucketRequest,
    },
    user::GetListRequest,
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
        if bucket::Bucket::exist(&name, &self.pool).await.is_ok() {
            return Err(Status::already_exists("存储桶名重复"));
        }
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
            .map_err(|_| Status::not_found("该存储桶不存在"))?;
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
            .map_err(|_| Status::not_found("该存储桶不存在"))?;
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
    async fn get_bucket_list(
        &self,
        request: Request<GetListRequest>,
    ) -> Result<Response<GetBucketListReply>, Status> {
        let GetListRequest {
            limit,
            offset,
            auth,
        } = request.into_inner();
        let limit = &limit;
        let limit = if limit > &50 { &50 } else { limit };
        let offset = &offset;
        let user_name = check_user(&auth).await?;
        let pool = &self.pool;
        let (count, data) = tokio::join!(
            bucket::Bucket::count_by_name(&user_name, pool),
            bucket::Bucket::find_many_by_user(*limit, *offset, &user_name, pool)
        );
        Ok(Response::new(GetBucketListReply {
            data: data?.into_iter().map(|x| x.into()).collect(),
            total: count?,
        }))
    }
}
