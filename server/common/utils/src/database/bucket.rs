use std::time::SystemTime;

use sqlx::{types::time::PrimitiveDateTime, Pool, Postgres};

use proto::{core::BucketInfo, Status};

use crate::errors::grpc::ToStatusResult;

#[derive(sqlx::Type)]
#[sqlx(type_name = "access_type")]
pub enum Access {
    Open,
    ReadOpen,
    Private,
}

impl From<proto::core::Access> for Access {
    fn from(access: proto::core::Access) -> Self {
        match access {
            proto::core::Access::Open => Self::Open,
            proto::core::Access::ReadOpen => Self::ReadOpen,
            proto::core::Access::Private => Self::Private,
        }
    }
}

impl TryFrom<i32> for Access {
    type Error = Status;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Open),
            1 => Ok(Self::ReadOpen),
            2 => Ok(Self::Private),
            other => Err(Status::invalid_argument(format!(
                "{other} 不是合法的 access 值"
            ))),
        }
    }
}

pub struct Bucket {
    /// 名字
    pub name: String,
    /// 创建时间
    pub create_time: PrimitiveDateTime,
    /// 更新时间
    pub update_time: PrimitiveDateTime,
    /// 访问权限
    pub access: Access,
    /// 用户名
    pub user_name: String,
}
impl Bucket {
    /// 创建
    pub async fn create(
        name: &str,
        access: impl Into<Access>,
        user_name: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        let access: Access = access.into();
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        sqlx::query("insert into bucket(name, create_time, update_time, access, user_name) values ($1,$2,$3,$4,$5)")
            .bind(name)
            .bind(&time)
            .bind(&time)
            .bind(access)
            .bind(user_name)
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
        access: &Access,
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
            "select name,access,create_time,update_time,user_name from bucket where name = $1",
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .to_status()?;
        Ok(bucket.into())
    }
    /// 删除删除
    pub async fn delete(name: &str, pool: &Pool<Postgres>) -> Result<(), Status> {
        sqlx::query("delete from bucket where name = $1")
            .bind(name)
            .execute(pool)
            .await
            .to_status()?;
        Ok(())
    }
    /// 获取列表
    pub async fn find_many_by_user(
        limit: u32,
        offset: u32,
        user_name: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Vec<Self>, Status> {
        let users: Vec<RowBucket> = sqlx::query_as(
            "select name,access,create_time,update_time,user_name from bucket where user_name = $1 offset $2 limit $3",
        )
        .bind(user_name)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await
        .to_status()?;
        Ok(users.into_iter().map(|x| x.into()).collect())
    }
    /// 获取总数
    pub async fn count_by_name(user_name: &str, pool: &Pool<Postgres>) -> Result<i64, Status> {
        let (count,): (i64,) =
            sqlx::query_as("select count(name) from bucket where user_name = $1")
                .bind(user_name)
                .fetch_one(pool)
                .await
                .to_status()?;
        Ok(count)
    }
}

#[allow(clippy::from_over_into)]
impl Into<BucketInfo> for Bucket {
    fn into(self) -> BucketInfo {
        let Bucket {
            name,
            create_time,
            update_time,
            access,
            user_name,
            ..
        } = self;
        let access: i32 = match access {
            Access::Open => 0,
            Access::ReadOpen => 1,
            Access::Private => 2,
        };
        let create_time = (create_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        let update_time = (update_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        BucketInfo {
            name,
            access,
            create_time,
            update_time,
            user_name,
        }
    }
}

type RowBucket = (String, Access, PrimitiveDateTime, PrimitiveDateTime, String);

impl From<RowBucket> for Bucket {
    fn from((name, access, create_time, update_time, user_name): RowBucket) -> Self {
        Self {
            name,
            access,
            create_time,
            update_time,
            user_name,
        }
    }
}
