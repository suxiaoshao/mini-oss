use std::sync::Arc;

use database::storage::StorageModal;
use database::time::OffsetDateTime;
use database::{Pool, Postgres};
use errors::TonicResult;
use proto::core::{
    storage_server, CountChartItem, CountChartReply, GetBucketWithTimeRequest, GetTimeRequest,
    SizeChartItem, SizeChartReply,
};
use proto::{async_trait, Request, Response, Status};
use validation::check_auth::check_user;

use crate::utils::check::check_bucket;

#[derive(Clone)]
pub struct StorageGreeter {
    pool: Arc<Pool<Postgres>>,
}

impl StorageGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl storage_server::Storage for StorageGreeter {
    async fn get_count_chart_by_bucket(
        &self,
        request: Request<GetBucketWithTimeRequest>,
    ) -> Result<Response<CountChartReply>, Status> {
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
        let storages = get_chart_storage_bucket(&bucket_name, start_time, end_time, pool).await?;
        let storages = chart_to_count(storages);
        Ok(Response::new(CountChartReply { data: storages }))
    }

    async fn get_size_chart_by_bucket(
        &self,
        request: Request<GetBucketWithTimeRequest>,
    ) -> Result<Response<SizeChartReply>, Status> {
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
        let storages = get_chart_storage_bucket(&bucket_name, start_time, end_time, pool).await?;
        let storages = chart_to_size(storages);
        Ok(Response::new(SizeChartReply { data: storages }))
    }

    async fn get_count_chart_by_user(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<CountChartReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            start_time,
            end_time,
        } = request.into_inner();
        let username = check_user(auth).await?;
        let storages = get_chart_storage_user(&username, start_time, end_time, pool).await?;
        let storages = chart_to_count(storages);
        Ok(Response::new(CountChartReply { data: storages }))
    }

    async fn get_size_chart_by_user(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<SizeChartReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetTimeRequest {
            start_time,
            end_time,
        } = request.into_inner();
        let username = check_user(auth).await?;
        let storages = get_chart_storage_user(&username, start_time, end_time, pool).await?;
        let storages = chart_to_size(storages);
        Ok(Response::new(SizeChartReply { data: storages }))
    }
}
const SPLIT_FLAG: usize = 300;

async fn get_chart_storage_bucket(
    bucket_name: &str,
    start_time: i64,
    end_time: i64,
    pool: &Pool<Postgres>,
) -> TonicResult<Vec<StorageModal>> {
    let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
    let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
    let storages =
        StorageModal::find_all_by_time_bucket(bucket_name, &start_time, &end_time, pool).await?;
    let storages = if storages.len() <= SPLIT_FLAG {
        storages
    } else {
        vec_filter_by_proportion(SPLIT_FLAG, storages)
    };
    Ok(storages)
}

async fn get_chart_storage_user(
    bucket_name: &str,
    start_time: i64,
    end_time: i64,
    pool: &Pool<Postgres>,
) -> TonicResult<Vec<StorageModal>> {
    let start_time = OffsetDateTime::from_unix_timestamp_nanos(start_time as i128 * 1000000);
    let end_time = OffsetDateTime::from_unix_timestamp_nanos(end_time as i128 * 1000000);
    let storages =
        StorageModal::find_all_by_time_user(bucket_name, &start_time, &end_time, pool).await?;
    let storages = if storages.len() <= SPLIT_FLAG {
        storages
    } else {
        vec_filter_by_proportion(SPLIT_FLAG, storages)
    };
    Ok(storages)
}

/// 按比例筛选
fn vec_filter_by_proportion<T>(num: usize, data: Vec<T>) -> Vec<T> {
    let proportion = (num as f64) / ((data.len() + 1) as f64);
    let mut s = vec![];
    let len = data.len();
    for (index, storage) in data.into_iter().enumerate() {
        if ((s.len() as f64) / ((index + 1) as f64)) < proportion || index == len - 1 {
            s.push(storage);
        }
    }
    s
}

fn chart_to_count(storages: Vec<StorageModal>) -> Vec<CountChartItem> {
    storages
        .into_iter()
        .map(|StorageModal { time, num, .. }| CountChartItem {
            time: (time.unix_timestamp_nanos() / 1000000) as i64,
            value: num,
        })
        .collect()
}

fn chart_to_size(storages: Vec<StorageModal>) -> Vec<SizeChartItem> {
    storages
        .into_iter()
        .map(|StorageModal { time, size, .. }| SizeChartItem {
            time: (time.unix_timestamp_nanos() / 1000000) as i64,
            value: size.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::greeter::storage::vec_filter_by_proportion;

    #[test]
    fn test_vec_filter() {
        let mut s = vec![];
        for i in 0..100 {
            s.push(i)
        }
        let s = vec_filter_by_proportion(30, s);
        println!("{:?}", s);
    }
}
