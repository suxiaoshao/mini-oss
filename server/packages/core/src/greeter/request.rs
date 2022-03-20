use std::sync::Arc;

use database::request::RequestModal;
use database::time::OffsetDateTime;
use database::{Decimal, Pool, Postgres};
use errors::TonicResult;
use proto::core::{
    CountDurationItem, CountDurationReply, CountReply, GetBucketWithTimeRequest, GetTimeRequest,
    SizeDurationItem, SizeDurationReply, SizeReply,
};
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

    async fn get_count_duration_by_bucket(
        &self,
        request: Request<GetBucketWithTimeRequest>,
    ) -> Result<Response<CountDurationReply>, Status> {
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
        let requests = get_chart_request(&bucket_name, start_time, end_time, pool).await?;
        let requests = requests
            .into_iter()
            .map(|(s, e, data)| CountDurationItem {
                start_time: (s.unix_timestamp_nanos() / 1000000) as i64,
                end_time: (e.unix_timestamp_nanos() / 1000000) as i64,
                value: data.len() as i64,
            })
            .collect::<Vec<_>>();
        Ok(Response::new(CountDurationReply { data: requests }))
    }

    async fn get_upload_duration_by_bucket(
        &self,
        request: Request<GetBucketWithTimeRequest>,
    ) -> Result<Response<SizeDurationReply>, Status> {
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
        let requests = get_chart_request(&bucket_name, start_time, end_time, pool).await?;
        let requests = requests
            .into_iter()
            .map(|(s, e, data)| SizeDurationItem {
                start_time: (s.unix_timestamp_nanos() / 1000000) as i64,
                end_time: (e.unix_timestamp_nanos() / 1000000) as i64,
                value: data
                    .iter()
                    .fold(
                        Decimal::default(),
                        |acc, RequestModal { upload_size, .. }| acc + upload_size,
                    )
                    .to_string(),
            })
            .collect::<Vec<_>>();
        Ok(Response::new(SizeDurationReply { data: requests }))
    }

    async fn get_download_duration_by_bucket(
        &self,
        request: Request<GetBucketWithTimeRequest>,
    ) -> Result<Response<SizeDurationReply>, Status> {
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
        let requests = get_chart_request(&bucket_name, start_time, end_time, pool).await?;
        let requests = requests
            .into_iter()
            .map(|(s, e, data)| SizeDurationItem {
                start_time: (s.unix_timestamp_nanos() / 1000000) as i64,
                end_time: (e.unix_timestamp_nanos() / 1000000) as i64,
                value: data
                    .iter()
                    .fold(
                        Decimal::default(),
                        |acc, RequestModal { download_size, .. }| acc + download_size,
                    )
                    .to_string(),
            })
            .collect::<Vec<_>>();
        Ok(Response::new(SizeDurationReply { data: requests }))
    }
}

type ChartTime<T> = (OffsetDateTime, OffsetDateTime, T);
type ChartTimeRequest = ChartTime<Vec<RequestModal>>;

const SPLIT_FLAG: u16 = 300;

async fn get_chart_request(
    bucket_name: &str,
    start_time: i64,
    end_time: i64,
    pool: &Pool<Postgres>,
) -> TonicResult<Vec<ChartTimeRequest>> {
    let mut result = vec![];
    let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
    let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
    // 时间间隔
    let dur = (start_time - end_time) / SPLIT_FLAG;
    for i in 0..=SPLIT_FLAG {
        result.push((start_time + (i * dur), end_time + (i + 1) * dur, vec![]));
    }
    let requests =
        RequestModal::find_all_by_time_bucket(bucket_name, &start_time, &end_time, pool).await?;
    for request in requests {
        if let Some((_, _, data)) = result
            .iter_mut()
            .find(|(s, e, _)| s < &request.time && e < &request.time)
        {
            data.push(request);
        }
    }
    Ok(result)
}
