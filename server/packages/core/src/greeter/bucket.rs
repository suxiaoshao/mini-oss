use std::sync::Arc;

use proto::{
    async_trait,
    auth::Empty,
    core::{
        bucket_server::Bucket, BucketInfo, CreateBucketRequest, DeleteBucketRequest,
        DeleteBucketsRequest, GetBucketListReply, UpdateBucketRequest,
    },
    user::GetListRequest,
    validation::Validate,
    Request, Response, Status,
};
use utils::{
    database::{
        bucket::{self, BucketModal},
        folder::{FolderModal, ObjectAccess},
        Pool, Postgres,
    },
    errors::grpc::ToStatusResult,
    mongo::Mongo,
    validation::check_auth::{check_manager, check_user},
};
#[derive(Clone)]
pub struct BucketGreeter {
    pool: Arc<Pool<Postgres>>,
    mongo: Arc<Mongo>,
}

impl BucketGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>, mongo: Arc<Mongo>) -> Self {
        Self { pool, mongo }
    }
}
#[async_trait]
impl Bucket for BucketGreeter {
    async fn create_bucket(
        &self,
        request: Request<CreateBucketRequest>,
    ) -> Result<Response<BucketInfo>, Status> {
        // 验证
        request.get_ref().validate().to_status()?;
        let access = request.get_ref().access();
        let CreateBucketRequest { name, auth, .. } = request.into_inner();
        let username = check_user(&auth).await?;
        let name = format!("{name}-{username}");
        // 判断该存储桶是否存在
        if BucketModal::exist(&name, &self.pool).await.is_ok() {
            return Err(Status::already_exists("存储桶名重复"));
        }
        // 创建存储桶
        let bucket = BucketModal::create(&name, access, &username, &self.pool).await?;
        // 创建存储桶下第一个文件夹
        FolderModal::create("/", ObjectAccess::Bucket, &bucket.name, "", &self.pool).await?;
        Ok(Response::new(bucket.into()))
    }
    async fn update_bucket(
        &self,
        request: Request<UpdateBucketRequest>,
    ) -> Result<Response<BucketInfo>, Status> {
        let access = request.get_ref().access();
        let UpdateBucketRequest { name, auth, .. } = request.into_inner();
        let username = check_user(&auth).await?;
        // 判断该存储桶是否存在
        BucketModal::exist(&name, &self.pool)
            .await
            .map_err(|_| Status::not_found("该存储桶不存在"))?;
        // 判断用户是否一致
        let BucketModal {
            username: username_,
            ..
        } = BucketModal::find_one(&name, &self.pool).await?;
        if username != username_ {
            return Err(Status::permission_denied("没有权限操作不属于你的存储桶"));
        }
        let access: bucket::BucketAccess = bucket::BucketAccess::from(access);
        let updated = BucketModal::update(&name, &access, &self.pool).await?;
        Ok(Response::new(updated.into()))
    }
    async fn delete_bucket(
        &self,
        request: Request<DeleteBucketRequest>,
    ) -> Result<Response<Empty>, Status> {
        let DeleteBucketRequest { name, auth } = request.into_inner();
        let username = check_user(&auth).await?;
        // 判断该存储桶是否存在
        BucketModal::exist(&name, &self.pool)
            .await
            .map_err(|_| Status::not_found("该存储桶不存在"))?;
        // 判断用户是否一致
        let BucketModal {
            username: username_,
            ..
        } = BucketModal::find_one(&name, &self.pool).await?;
        if username != username_ {
            return Err(Status::permission_denied("没有权限操作不属于你的存储桶"));
        }
        // 数据库中删除
        BucketModal::delete(&name, &self.pool).await?;
        self.mongo.drop_self(name).await?;
        // todo folder 删除
        Ok(Response::new(Empty {}))
    }
    async fn delete_buckets(
        &self,
        request: Request<DeleteBucketsRequest>,
    ) -> Result<Response<Empty>, Status> {
        let pool = &self.pool;
        let DeleteBucketsRequest { username, auth } = request.into_inner();
        check_manager(&auth).await?;
        // 获取所有 buckets
        let buckets = BucketModal::find_total_by_user(&username, pool).await?;
        // 在 mongo 中删除
        let mongo_delete = buckets
            .into_iter()
            .map(|BucketModal { name, .. }| self.mongo.drop_self(name))
            .collect::<Vec<_>>();
        let mongo_delete = futures::future::join_all(mongo_delete);
        // sql中删除
        let sql_delete = BucketModal::delete_by_user(&username, pool);
        // 验证结果
        let (mongo_delete, sql_delete) = futures::join!(mongo_delete, sql_delete);
        sql_delete?;
        for mongo in mongo_delete {
            mongo?;
        }
        // todo 删除 folder
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
        let username = check_user(&auth).await?;
        let pool = &self.pool;
        let (count, data) = tokio::join!(
            BucketModal::count_by_name(&username, pool),
            BucketModal::find_many_by_user(*limit, *offset, &username, pool)
        );
        Ok(Response::new(GetBucketListReply {
            data: data?.into_iter().map(|x| x.into()).collect(),
            total: count?,
        }))
    }
}
