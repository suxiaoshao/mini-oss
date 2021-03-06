use std::time::SystemTime;

use sqlx::{FromRow, Pool, Postgres};

use crate::time::OffsetDateTime;
use errors::TonicResult;
use proto::core::BucketInfo;

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

#[derive(FromRow)]
pub struct BucketModal {
    /// 名字
    pub name: String,
    /// 创建时间
    pub create_time: OffsetDateTime,
    /// 更新时间
    pub update_time: OffsetDateTime,
    /// 访问权限
    pub access: BucketAccess,
    /// 用户名
    pub username: String,
}

/// self
impl BucketModal {
    /// 创建
    pub async fn create(
        name: &str,
        access: impl Into<BucketAccess>,
        username: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        let access: BucketAccess = access.into();
        // 获取现在时间
        let time = OffsetDateTime::from(SystemTime::now());
        sqlx::query("insert into bucket(name, create_time, update_time, access, username) values ($1,$2,$3,$4,$5)")
            .bind(name)
            .bind(&time)
            .bind(&time)
            .bind(access)
            .bind(username)
            .execute(pool)
            .await?;
        Self::find_one(name, pool).await
    }
    /// 判断是否存在
    pub async fn exist(name: &str, pool: &Pool<Postgres>) -> TonicResult<()> {
        sqlx::query("select * from bucket where name = $1")
            .bind(name)
            .fetch_one(pool)
            .await?;
        Ok(())
    }
    /// 更新
    pub async fn update(
        name: &str,
        access: &BucketAccess,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        // 获取现在时间
        let time = OffsetDateTime::from(SystemTime::now());
        sqlx::query("update bucket set access = $1, update_time = $2 where name = $3")
            .bind(access)
            .bind(time)
            .bind(name)
            .execute(pool)
            .await?;
        Self::find_one(name, pool).await
    }
    /// 获取第一项
    pub async fn find_one(name: &str, pool: &Pool<Postgres>) -> TonicResult<Self> {
        let bucket = sqlx::query_as(
            "select name,access,create_time,update_time,username from bucket where name = $1",
        )
        .bind(name)
        .fetch_one(pool)
        .await?;
        Ok(bucket)
    }
    /// 删除
    pub async fn delete(name: &str, pool: &Pool<Postgres>) -> TonicResult<()> {
        sqlx::query("delete from bucket where name = $1")
            .bind(name)
            .execute(pool)
            .await?;
        Ok(())
    }
    /// 判断读取访问权限
    pub async fn read_open(bucket_name: &str, pool: &Pool<Postgres>) -> TonicResult<bool> {
        let Self { access, .. } = Self::find_one(bucket_name, pool).await?;
        Ok(match access {
            BucketAccess::Open => true,
            BucketAccess::ReadOpen => true,
            BucketAccess::Private => false,
        })
    }
    /// 判断写访问权限
    pub async fn write_open(bucket_name: &str, pool: &Pool<Postgres>) -> TonicResult<bool> {
        let Self { access, .. } = Self::find_one(bucket_name, pool).await?;
        Ok(match access {
            BucketAccess::Open => true,
            BucketAccess::ReadOpen => false,
            BucketAccess::Private => false,
        })
    }
}

/// user
impl BucketModal {
    /// 删除某个用户下所有
    pub async fn delete_by_user(username: &str, pool: &Pool<Postgres>) -> TonicResult<()> {
        sqlx::query("delete from bucket where username = $1")
            .bind(username)
            .execute(pool)
            .await?;
        Ok(())
    }
    /// 获取列表
    pub async fn find_many_by_user(
        limit: u32,
        offset: u32,
        username: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Vec<Self>> {
        let users = sqlx::query_as(
            "select name,access,create_time,update_time,username from bucket where username = $1 offset $2 limit $3",
        )
            .bind(username)
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(users)
    }
    /// 获取全部列表
    pub async fn find_total_by_user(
        username: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Vec<Self>> {
        let users = sqlx::query_as(
            "select name,access,create_time,update_time,username from bucket where username = $1",
        )
        .bind(username)
        .fetch_all(pool)
        .await?;
        Ok(users)
    }
    /// 获取总数
    pub async fn count_by_username(username: &str, pool: &Pool<Postgres>) -> TonicResult<i64> {
        let (count,): (i64,) = sqlx::query_as("select count(name) from bucket where username = $1")
            .bind(username)
            .fetch_one(pool)
            .await?;
        Ok(count)
    }
}

impl From<BucketModal> for BucketInfo {
    fn from(value: BucketModal) -> Self {
        let BucketModal {
            name,
            create_time,
            update_time,
            access,
            username,
            ..
        } = value;
        let access: i32 = match access {
            BucketAccess::Open => 0,
            BucketAccess::ReadOpen => 1,
            BucketAccess::Private => 2,
        };
        let create_time = (create_time.unix_timestamp_nanos() / 1000000) as i64;
        let update_time = (update_time.unix_timestamp_nanos() / 1000000) as i64;
        BucketInfo {
            name,
            access,
            create_time,
            update_time,
            username,
        }
    }
}
