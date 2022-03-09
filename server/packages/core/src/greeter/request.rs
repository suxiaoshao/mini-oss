use std::sync::Arc;

use database::request::RequestModal;
use database::time::OffsetDateTime;
use database::{Pool, Postgres};
use proto::core::{CountReply, GetTimeRequest, SizeReply};
use proto::{async_trait, core::request_server, Request, Response, Status};

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
        request: Request<GetTimeRequest>,
    ) -> Result<Response<SizeReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            bucket_name,
            start_time,
            end_time,
        } = request.into_inner();
        // 判断权限
        check_bucket(auth, &bucket_name, pool).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let size =
            RequestModal::total_upload_size_by_time(&bucket_name, &start_time, &end_time, pool)
                .await?
                .to_string();
        Ok(Response::new(SizeReply { size }))
    }

    async fn get_download_size_by_bucket(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<SizeReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            bucket_name,
            start_time,
            end_time,
        } = request.into_inner();
        // 判断权限
        check_bucket(auth, &bucket_name, pool).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let size =
            RequestModal::total_download_size_by_time(&bucket_name, &start_time, &end_time, pool)
                .await?
                .to_string();
        Ok(Response::new(SizeReply { size }))
    }

    async fn get_count_by_bucket(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<CountReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            bucket_name,
            start_time,
            end_time,
        } = request.into_inner();
        // 判断权限
        check_bucket(auth, &bucket_name, pool).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let total = RequestModal::count_by_time(&bucket_name, &start_time, &end_time, pool).await?;
        Ok(Response::new(CountReply { total }))
    }
}
