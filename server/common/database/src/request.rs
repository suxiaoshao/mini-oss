use std::time::SystemTime;

use sqlx::{
    types::{time::PrimitiveDateTime, Decimal},
    FromRow, Pool, Postgres,
};

use errors::TonicResult;

#[derive(FromRow, Debug)]
pub struct RequestModal {
    /// 创建时间
    pub time: PrimitiveDateTime,
    /// bucket 名
    pub bucket_name: String,
    /// 对象大小
    pub upload_size: Decimal,
    /// 对象大小
    pub download_size: Decimal,
    /// id
    pub object_id: String,
}

impl RequestModal {
    /// 创建
    pub async fn create(
        object_id: &str,
        bucket_name: &str,
        upload_size: &Decimal,
        download_size: &Decimal,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        sqlx::query(
            r#"insert into request
                            (object_id,bucket_name,upload_size,download_size,time)
                            values ($1,$2,$3,$4,$5)"#,
        )
        .bind(object_id)
        .bind(bucket_name)
        .bind(upload_size)
        .bind(download_size)
        .bind(&time)
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
        size: usize,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        match Self::exist(object_id, pool).await {
            Ok(_) => {
                sqlx::query("update request set upload_size=upload_size + $1 where object_id = $2")
                    .bind(size as i64)
                    .bind(object_id)
                    .execute(pool)
                    .await?;
                Self::find_one(object_id, pool).await
            }
            Err(_) => {
                Self::create(
                    object_id,
                    bucket_name,
                    &Decimal::from(size),
                    &Decimal::from(0),
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
        size: usize,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        match Self::exist(object_id, pool).await {
            Ok(_) => {
                sqlx::query(
                    "update request set download_size=download_size + $1 where object_id = $2",
                )
                .bind(size as i64)
                .bind(object_id)
                .execute(pool)
                .await?;
                Self::find_one(object_id, pool).await
            }
            Err(_) => {
                Self::create(
                    object_id,
                    bucket_name,
                    &Decimal::from(0),
                    &Decimal::from(size),
                    pool,
                )
                .await
            }
        }
    }
}
