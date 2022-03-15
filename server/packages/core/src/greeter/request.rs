use std::sync::Arc;

use database::request::RequestModal;
use database::time::OffsetDateTime;
use database::{Pool, Postgres};
use proto::core::{CountReply, GetBucketWithTimeRequest, GetTimeRequest, SizeReply};
use proto::{async_trait, core::request_server, Request, Response, Status};
use validation::check_auth::check_user;

use crate::utils::check::check_bucket;

#[derive(Clone)]
pub struct RequestGreeter {
    pool: Arc<Pool<Postgres>>,
}

impl RequestGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl request_server::Request for RequestGreeter {
    async fn get_upload_size_by_bucket(
        &self,
        request: Request<GetBucketWithTimeRequest>,
    ) -> Result<Response<SizeReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetBucketWithTimeRequest {
            bucket_name,
            start_time,
            end_time,
        } = request.into_inner();
        // 判断权限
        check_bucket(auth, &bucket_name, pool).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let size = RequestModal::upload_by_time_bucket(&bucket_name, &start_time, &end_time, pool)
            .await?
            .to_string();
        Ok(Response::new(SizeReply { size }))
    }

    async fn get_download_size_by_bucket(
        &self,
        request: Request<GetBucketWithTimeRequest>,
    ) -> Result<Response<SizeReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetBucketWithTimeRequest {
            bucket_name,
            start_time,
            end_time,
        } = request.into_inner();
        // 判断权限
        check_bucket(auth, &bucket_name, pool).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let size =
            RequestModal::download_by_time_bucket(&bucket_name, &start_time, &end_time, pool)
                .await?
                .to_string();
        Ok(Response::new(SizeReply { size }))
    }

    async fn get_count_by_bucket(
        &self,
        request: Request<GetBucketWithTimeRequest>,
    ) -> Result<Response<CountReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetBucketWithTimeRequest {
            bucket_name,
            start_time,
            end_time,
        } = request.into_inner();
        // 判断权限
        check_bucket(auth, &bucket_name, pool).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let total =
            RequestModal::count_by_time_bucket(&bucket_name, &start_time, &end_time, pool).await?;
        Ok(Response::new(CountReply { total }))
    }

    async fn get_upload_size_by_user(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<SizeReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            start_time,
            end_time,
        } = request.into_inner();
        let username = check_user(auth).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let size = RequestModal::upload_by_time_user(&username, &start_time, &end_time, pool)
            .await?
            .to_string();
        Ok(Response::new(SizeReply { size }))
    }

    async fn get_download_size_by_user(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<SizeReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            start_time,
            end_time,
        } = request.into_inner();
        let username = check_user(auth).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let size = RequestModal::download_by_time_user(&username, &start_time, &end_time, pool)
            .await?
            .to_string();
        Ok(Response::new(SizeReply { size }))
    }

    async fn get_count_by_user(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<CountReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            start_time,
            end_time,
        } = request.into_inner();
        let username = check_user(auth).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let total =
            RequestModal::count_by_time_bucket(&username, &start_time, &end_time, pool).await?;
        Ok(Response::new(CountReply { total }))
    }
}
