use std::time::SystemTime;

use sqlx::types::Decimal;
use sqlx::{types::time::OffsetDateTime, FromRow, Pool, Postgres};

use errors::TonicResult;

#[derive(FromRow, Debug)]
pub struct StorageModal {
    /// 创建时间
    pub time: OffsetDateTime,
    /// bucket 名
    pub bucket_name: String,
    /// 对象大小
    pub size: Decimal,
    /// 对象数量
    pub num: i64,
    /// 用户名
    pub username: String,
}
impl StorageModal {
    /// 创建
    pub async fn create(
        bucket_name: &str,
        size: &Decimal,
        num: i64,
        username: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        // 获取现在时间
        let time = OffsetDateTime::from(SystemTime::now());
        sqlx::query(
            r#"insert into storage
                            ( bucket_name,size,num,time,username)
                            values ($1,$2,$3,$4,$5)"#,
        )
        .bind(bucket_name)
        .bind(size)
        .bind(num)
        .bind(&time)
        .bind(username)
        .execute(pool)
        .await?;
        Self::find_one(bucket_name, &time, pool).await
    }
    /// 获取第一项
    pub async fn find_one(
        bucket_name: &str,
        time: &OffsetDateTime,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        let object = sqlx::query_as(
            r##"select *
                        from storage where bucket_name=$1 and time = $2"##,
        )
        .bind(bucket_name)
        .bind(time)
        .fetch_one(pool)
        .await?;
        Ok(object)
    }
}

/// bucket
impl StorageModal {
    /// 删除某个 bucket 下所有
    pub async fn delete_by_bucket(bucket_name: &str, pool: &Pool<Postgres>) -> TonicResult<()> {
        sqlx::query("delete from storage where bucket_name = $1")
            .bind(bucket_name)
            .execute(pool)
            .await?;
        Ok(())
    }
}

/// bucket time
impl StorageModal{
    /// 获取某个时间间隔的所有数据
    pub async fn find_all_by_time_bucket(
        bucket_name: &str,
        start_time: &OffsetDateTime,
        end_time: &OffsetDateTime,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Vec<Self>> {
        let result = sqlx::query_as(
            "select * from storage where bucket_name = $1 and time >= $2 and time <= $3 order by time",
        )
            .bind(bucket_name)
            .bind(start_time)
            .bind(end_time)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }
}
