use std::time::SystemTime;

use sqlx::{
    types::{time::OffsetDateTime, Decimal},
    FromRow, Pool, Postgres,
};

use errors::TonicResult;

#[derive(FromRow, Debug)]
pub struct RequestModal {
    /// 创建时间
    pub time: OffsetDateTime,
    /// bucket 名
    pub bucket_name: String,
    /// 对象大小
    pub upload_size: Decimal,
    /// 对象大小
    pub download_size: Decimal,
    /// id
    pub object_id: String,
    /// 用户名
    pub username: String,
}

impl RequestModal {
    /// 创建
    pub async fn create(
        object_id: &str,
        bucket_name: &str,
        upload_size: &Decimal,
        download_size: &Decimal,
        username: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        // 获取现在时间
        let time = OffsetDateTime::from(SystemTime::now());
        sqlx::query(
            r#"insert into request
                            (object_id,bucket_name,upload_size,download_size,time,username)
                            values ($1,$2,$3,$4,$5,$6)"#,
        )
        .bind(object_id)
        .bind(bucket_name)
        .bind(upload_size)
        .bind(download_size)
        .bind(&time)
        .bind(username)
        .execute(pool)
        .await?;
        Self::find_one(object_id, pool).await
    }
    /// 获取第一项
    pub async fn find_one(object_id: &str, pool: &Pool<Postgres>) -> TonicResult<Self> {
        let object = sqlx::query_as(
            r##"select *
                        from request where  object_id = $2"##,
        )
        .bind(object_id)
        .fetch_one(pool)
        .await?;
        Ok(object)
    }
    /// 判断是否存在
    pub async fn exist(object_id: &str, pool: &Pool<Postgres>) -> TonicResult<()> {
        sqlx::query("select * from request where object_id = $1")
            .bind(object_id)
            .fetch_one(pool)
            .await?;
        Ok(())
    }
    /// 添加上传大小
    pub async fn add_upload_size(
        object_id: &str,
        bucket_name: &str,
        size: &Decimal,
        username: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        match Self::exist(object_id, pool).await {
            Ok(_) => {
                sqlx::query("update request set upload_size=upload_size + $1 where object_id = $2")
                    .bind(size)
                    .bind(object_id)
                    .execute(pool)
                    .await?;
                Self::find_one(object_id, pool).await
            }
            Err(_) => {
                Self::create(
                    object_id,
                    bucket_name,
                    size,
                    &Decimal::default(),
                    username,
                    pool,
                )
                .await
            }
        }
    }
    /// 添加下载大小
    pub async fn add_download_size(
        object_id: &str,
        bucket_name: &str,
        size: &Decimal,
        username: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        match Self::exist(object_id, pool).await {
            Ok(_) => {
                sqlx::query(
                    "update request set download_size=download_size + $1 where object_id = $2",
                )
                .bind(size)
                .bind(object_id)
                .execute(pool)
                .await?;
                Self::find_one(object_id, pool).await
            }
            Err(_) => {
                Self::create(
                    object_id,
                    bucket_name,
                    &Decimal::default(),
                    size,
                    username,
                    pool,
                )
                .await
            }
        }
    }
}

/// bucket
impl RequestModal {
    /// 删除某个 bucket 下所有
    pub async fn delete_by_bucket(bucket_name: &str, pool: &Pool<Postgres>) -> TonicResult<()> {
        sqlx::query("delete from request where bucket_name = $1")
            .bind(bucket_name)
            .execute(pool)
            .await?;
        Ok(())
    }
}
/// time
impl RequestModal {
    /// 获取某个时间间隔的上传总量
    pub async fn total_upload_size_by_time(
        bucket_name: &str,
        start_time: &OffsetDateTime,
        end_time: &OffsetDateTime,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Decimal> {
        let (size,): (Option<Decimal>,) = sqlx::query_as("select sum(upload_size) from request where bucket_name = $1 and time >= $2 and time <= $3")
            .bind(bucket_name).bind(start_time).bind(end_time)
            .fetch_one(pool)
            .await?;
        Ok(size.unwrap_or_default())
    }
    /// 获取某个时间间隔的上传总量
    pub async fn total_download_size_by_time(
        bucket_name: &str,
        start_time: &OffsetDateTime,
        end_time: &OffsetDateTime,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Decimal> {
        let (size,): (Option<Decimal>,) = sqlx::query_as("select sum(download_size) from request where bucket_name = $1 and time >= $2 and time <= $3")
            .bind(bucket_name).bind(start_time).bind(end_time)
            .fetch_one(pool)
            .await?;
        Ok(size.unwrap_or_default())
    }
    /// 获取某个时间间隔的上传总量
    pub async fn count_by_time(
        bucket_name: &str,
        start_time: &OffsetDateTime,
        end_time: &OffsetDateTime,
        pool: &Pool<Postgres>,
    ) -> TonicResult<i64> {
        let (total,): (i64,) = sqlx::query_as("select count(object_id) from request where bucket_name = $1 and time >= $2 and time <= $3")
            .bind(bucket_name).bind(start_time).bind(end_time)
            .fetch_one(pool)
            .await?;
        Ok(total)
    }
}
