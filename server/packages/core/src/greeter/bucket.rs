use std::sync::Arc;

use proto::core::GetBucketRequest;
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
        object::ObjectModal,
        Pool, Postgres,
    },
    errors::grpc::ToStatusResult,
    mongo::Mongo,
    validation::check_auth::{check_manager, check_user},
};

use crate::utils::check::check_bucket;
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
    async fn delete_bucket(
        &self,
        request: Request<DeleteBucketRequest>,
    ) -> Result<Response<Empty>, Status> {
        let pool = &self.pool;
        let DeleteBucketRequest { name, auth } = request.into_inner();
        // 判断该存储桶是否存在和权限
        check_bucket(&auth, &name, pool).await?;
        // 数据库删除
        futures::try_join!(
            BucketModal::delete(&name, pool),
            FolderModal::delete_by_bucket(&name, pool),
            ObjectModal::delete_by_bucket(&name, pool),
            self.mongo.drop_self(name.clone())
        )?;
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
        // 删除 folder
        let mut folder_delete = vec![];
        let mut mongo_delete = vec![];
        let mut object_delete = vec![];
        // 在 mongo 中删除
        for BucketModal { name, .. } in &buckets {
            folder_delete.push(FolderModal::delete_by_bucket(name, pool));
            mongo_delete.push(self.mongo.drop_self(name.clone()));
            object_delete.push(ObjectModal::delete_by_bucket(name, pool));
        }
        let mongo_delete = futures::future::try_join_all(mongo_delete);
        let folder_delete = futures::future::try_join_all(folder_delete);
        let object_delete = futures::future::try_join_all(object_delete);
        // sql中删除
        let sql_delete = BucketModal::delete_by_user(&username, pool);
        // 验证结果
        futures::try_join!(mongo_delete, sql_delete, folder_delete, object_delete)?;
        Ok(Response::new(Empty {}))
    }
    async fn update_bucket(
        &self,
        request: Request<UpdateBucketRequest>,
    ) -> Result<Response<BucketInfo>, Status> {
        let pool = &self.pool;
        let access = request.get_ref().access();
        let UpdateBucketRequest { name, auth, .. } = request.into_inner();
        // 判断该存储桶是否存在和权限
        check_bucket(&auth, &name, pool).await?;
        let access: bucket::BucketAccess = bucket::BucketAccess::from(access);
        let updated = BucketModal::update(&name, &access, &self.pool).await?;
        Ok(Response::new(updated.into()))
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

    async fn get_bucket(
        &self,
        request: Request<GetBucketRequest>,
    ) -> Result<Response<BucketInfo>, Status> {
        let pool = &self.pool;
        let GetBucketRequest { auth, bucket_name } = request.into_inner();
        // 判断权限
        check_bucket(&auth, &bucket_name, pool).await?;
        let bucket = BucketModal::find_one(&bucket_name, pool).await?;
        Ok(Response::new(bucket.into()))
    }
}
