use std::sync::Arc;

use proto::{
    async_trait,
    auth::Empty,
    core::{
        bucket_server::Bucket, BucketInfo, CreateBucketRequest, DeleteBucketRequest,
        UpdateBucketRequest,
    },
    Request, Response, Status,
};
use utils::{
    database::{Pool, Postgres},
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
        todo!()
    }
    async fn update_bucket(
        &self,
        request: Request<UpdateBucketRequest>,
    ) -> Result<Response<BucketInfo>, Status> {
        todo!()
    }
    async fn delete_bucket(
        &self,
        request: Request<DeleteBucketRequest>,
    ) -> Result<Response<Empty>, Status> {
        todo!()
    }
}
