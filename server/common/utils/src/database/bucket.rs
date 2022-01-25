use std::time::SystemTime;

use sqlx::{types::time::PrimitiveDateTime, Pool, Postgres};

use proto::{core::BucketInfo, Status};

use crate::errors::grpc::ToStatusResult;

#[derive(sqlx::Type)]
#[sqlx(type_name = "bucket_access_type")]
pub enum BucketAccess {
    Open,
    ReadOpen,
    Private,
}

impl From<proto::core::BucketAccess> for BucketAccess {
    fn from(access: proto::core::BucketAccess) -> Self {
        match access {
            proto::core::BucketAccess::Open => Self::Open,
            proto::core::BucketAccess::ReadOpen => Self::ReadOpen,
            proto::core::BucketAccess::Private => Self::Private,
        }
    }
}

pub struct BucketModal {
    /// 名字
    pub name: String,
    /// 创建时间
    pub create_time: PrimitiveDateTime,
    /// 更新时间
    pub update_time: PrimitiveDateTime,
    /// 访问权限
    pub access: BucketAccess,
    /// 用户名
    pub username: String,
}
impl BucketModal {
    /// 创建
    pub async fn create(
        name: &str,
        access: impl Into<BucketAccess>,
        username: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        let access: BucketAccess = access.into();
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        sqlx::query("insert into bucket(name, create_time, update_time, access, username) values ($1,$2,$3,$4,$5)")
            .bind(name)
            .bind(&time)
            .bind(&time)
            .bind(access)
            .bind(username)
            .execute(pool)
            .await
            .to_status()?;
        Self::find_one(name, pool).await
    }
    /// 判断是否存在
    pub async fn exist(name: &str, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query("select * from bucket where name = $1")
            .bind(name)
            .fetch_one(pool)
            .await
            .map(|_| ())
    }
    /// 更新
    pub async fn update(
        name: &str,
        access: &BucketAccess,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        sqlx::query("update bucket set access = $1, update_time = $2 where name = $3")
            .bind(access)
            .bind(time)
            .bind(name)
            .execute(pool)
            .await
            .to_status()?;
        Self::find_one(name, pool).await
    }
    /// 获取第一项
    pub async fn find_one(name: &str, pool: &Pool<Postgres>) -> Result<Self, Status> {
        let bucket: RowBucket = sqlx::query_as(
            "select name,access,create_time,update_time,username from bucket where name = $1",
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .to_status()?;
        Ok(bucket.into())
    }
    /// 删除
    pub async fn delete(name: &str, pool: &Pool<Postgres>) -> Result<(), Status> {
        sqlx::query("delete from bucket where name = $1")
            .bind(name)
            .execute(pool)
            .await
            .to_status()?;
        Ok(())
    }
    /// 删除某个用户下所有
    pub async fn delete_by_user(username: &str, pool: &Pool<Postgres>) -> Result<(), Status> {
        sqlx::query("delete from bucket where username = $1")
            .bind(username)
            .execute(pool)
            .await
            .to_status()?;
        Ok(())
    }
    /// 获取列表
    pub async fn find_many_by_user(
        limit: u32,
        offset: u32,
        username: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Vec<Self>, Status> {
        let users: Vec<RowBucket> = sqlx::query_as(
            "select name,access,create_time,update_time,username from bucket where username = $1 offset $2 limit $3",
        )
        .bind(username)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await
        .to_status()?;
        Ok(users.into_iter().map(|x| x.into()).collect())
    }
    /// 获取全部列表
    pub async fn find_total_by_user(
        username: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Vec<Self>, Status> {
        let users: Vec<RowBucket> = sqlx::query_as(
            "select name,access,create_time,update_time,username from bucket where username = $1",
        )
        .bind(username)
        .fetch_all(pool)
        .await
        .to_status()?;
        Ok(users.into_iter().map(|x| x.into()).collect())
    }
    /// 获取总数
    pub async fn count_by_name(username: &str, pool: &Pool<Postgres>) -> Result<i64, Status> {
        let (count,): (i64,) = sqlx::query_as("select count(name) from bucket where username = $1")
            .bind(username)
            .fetch_one(pool)
            .await
            .to_status()?;
        Ok(count)
    }
}

#[allow(clippy::from_over_into)]
impl Into<BucketInfo> for BucketModal {
    fn into(self) -> BucketInfo {
        let BucketModal {
            name,
            create_time,
            update_time,
            access,
            username,
            ..
        } = self;
        let access: i32 = match access {
            BucketAccess::Open => 0,
            BucketAccess::ReadOpen => 1,
            BucketAccess::Private => 2,
        };
        let create_time = (create_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        let update_time = (update_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        BucketInfo {
            name,
            access,
            create_time,
            update_time,
            username,
        }
    }
}

type RowBucket = (
    String,
    BucketAccess,
    PrimitiveDateTime,
    PrimitiveDateTime,
    String,
);

impl From<RowBucket> for BucketModal {
    fn from((name, access, create_time, update_time, username): RowBucket) -> Self {
        Self {
            name,
            access,
            create_time,
            update_time,
            username,
        }
    }
}
