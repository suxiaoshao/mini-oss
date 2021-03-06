use std::sync::Arc;

use database::request::RequestModal;
use database::time::OffsetDateTime;
use database::{Decimal, Pool, Postgres};
use errors::TonicResult;
use proto::core::{
    CountDurationItem, CountDurationReply, GetBucketWithTimeRequest, GetTimeRequest,
    SizeDurationItem, SizeDurationReply, SizeReply,
};
use proto::user::CountReply;
use proto::{async_trait, core::request_server, Request, Response, Status};
use validation::check_auth::{check_manager, check_user};

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

    async fn get_upload_size(
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
        check_manager(auth).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let size = RequestModal::upload_size(&start_time, &end_time, pool)
            .await?
            .to_string();
        Ok(Response::new(SizeReply { size }))
    }

    async fn get_download_size(
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
        check_manager(auth).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let size = RequestModal::download_size(&start_time, &end_time, pool)
            .await?
            .to_string();
        Ok(Response::new(SizeReply { size }))
    }

    async fn get_count(
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
        check_manager(auth).await?;
        let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
        let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
        let total = RequestModal::count(&start_time, &end_time, pool).await?;
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
        let requests =
            get_chart_request_by_bucket(&bucket_name, start_time, end_time, pool).await?;
        let requests = chart_to_count(requests);
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
        let requests =
            get_chart_request_by_bucket(&bucket_name, start_time, end_time, pool).await?;
        let requests = chart_to_upload(requests);
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
        let requests =
            get_chart_request_by_bucket(&bucket_name, start_time, end_time, pool).await?;
        let requests = chart_to_download(requests);
        Ok(Response::new(SizeDurationReply { data: requests }))
    }

    async fn get_count_duration_by_user(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<CountDurationReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            start_time,
            end_time,
        } = request.into_inner();
        let username = check_user(auth).await?;
        let requests = get_chart_request_by_user(&username, start_time, end_time, pool).await?;
        let requests = chart_to_count(requests);
        Ok(Response::new(CountDurationReply { data: requests }))
    }

    async fn get_upload_duration_by_user(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<SizeDurationReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            start_time,
            end_time,
        } = request.into_inner();
        let username = check_user(auth).await?;
        let requests = get_chart_request_by_user(&username, start_time, end_time, pool).await?;
        let requests = chart_to_upload(requests);
        Ok(Response::new(SizeDurationReply { data: requests }))
    }

    async fn get_download_duration_by_user(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<SizeDurationReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            start_time,
            end_time,
        } = request.into_inner();
        let username = check_user(auth).await?;
        let requests = get_chart_request_by_user(&username, start_time, end_time, pool).await?;
        let requests = chart_to_download(requests);
        Ok(Response::new(SizeDurationReply { data: requests }))
    }

    async fn get_count_duration(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<CountDurationReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            start_time,
            end_time,
        } = request.into_inner();
        check_manager(auth).await?;
        let requests = get_chart_request(start_time, end_time, pool).await?;
        let requests = chart_to_count(requests);
        Ok(Response::new(CountDurationReply { data: requests }))
    }

    async fn get_upload_duration(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<SizeDurationReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            start_time,
            end_time,
        } = request.into_inner();
        check_manager(auth).await?;
        let requests = get_chart_request(start_time, end_time, pool).await?;
        let requests = chart_to_upload(requests);
        Ok(Response::new(SizeDurationReply { data: requests }))
    }

    async fn get_download_duration(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<SizeDurationReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            start_time,
            end_time,
        } = request.into_inner();
        check_manager(auth).await?;
        let requests = get_chart_request(start_time, end_time, pool).await?;
        let requests = chart_to_download(requests);
        Ok(Response::new(SizeDurationReply { data: requests }))
    }
}

type ChartTime<T> = (OffsetDateTime, OffsetDateTime, T);
type ChartTimeRequest = ChartTime<Vec<RequestModal>>;

const SPLIT_FLAG: u16 = 300;

/// 获取 bucket 下 request chart
async fn get_chart_request(
    start_time: i64,
    end_time: i64,
    pool: &Pool<Postgres>,
) -> TonicResult<Vec<ChartTimeRequest>> {
    let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
    let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
    let requests = RequestModal::find_all(&start_time, &end_time, pool).await?;
    let requests = request_to_chart(start_time, end_time, requests).await?;
    Ok(requests)
}

/// 获取 bucket 下 request chart
async fn get_chart_request_by_bucket(
    bucket_name: &str,
    start_time: i64,
    end_time: i64,
    pool: &Pool<Postgres>,
) -> TonicResult<Vec<ChartTimeRequest>> {
    let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
    let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
    let requests =
        RequestModal::find_all_by_time_bucket(bucket_name, &start_time, &end_time, pool).await?;
    let requests = request_to_chart(start_time, end_time, requests).await?;
    Ok(requests)
}

/// 获取用户下 request chart
async fn get_chart_request_by_user(
    username: &str,
    start_time: i64,
    end_time: i64,
    pool: &Pool<Postgres>,
) -> TonicResult<Vec<ChartTimeRequest>> {
    let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
    let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
    let requests =
        RequestModal::find_all_by_time_user(username, &start_time, &end_time, pool).await?;
    let requests = request_to_chart(start_time, end_time, requests).await?;
    Ok(requests)
}

async fn request_to_chart(
    start_time: OffsetDateTime,
    end_time: OffsetDateTime,
    requests: Vec<RequestModal>,
) -> TonicResult<Vec<ChartTimeRequest>> {
    let mut result = vec![];
    // 时间间隔
    let dur = (start_time - end_time) / SPLIT_FLAG;
    for i in 0..=SPLIT_FLAG {
        result.push((start_time + (i * dur), end_time + (i + 1) * dur, vec![]));
    }
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

fn chart_to_count(requests: Vec<ChartTimeRequest>) -> Vec<CountDurationItem> {
    requests
        .into_iter()
        .map(|(s, e, data)| CountDurationItem {
            start_time: (s.unix_timestamp_nanos() / 1000000) as i64,
            end_time: (e.unix_timestamp_nanos() / 1000000) as i64,
            value: data.len() as i64,
        })
        .collect()
}
fn chart_to_upload(requests: Vec<ChartTimeRequest>) -> Vec<SizeDurationItem> {
    requests
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
        .collect()
}
fn chart_to_download(requests: Vec<ChartTimeRequest>) -> Vec<SizeDurationItem> {
    requests
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
        .collect()
}
