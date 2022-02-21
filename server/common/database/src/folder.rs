use std::time::SystemTime;

#[cfg(feature = "recursion")]
use async_recursion::async_recursion;
use sqlx::{types::time::PrimitiveDateTime, FromRow, Pool, Postgres};

use errors::TonicResult;
use proto::core::FolderInfo;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "folder_access_type")]
pub enum FolderAccess {
    Inheritance,
    ReadOpen,
    Private,
    Open,
}

impl From<proto::core::FolderAccess> for FolderAccess {
    fn from(value: proto::core::FolderAccess) -> Self {
        match value {
            proto::core::FolderAccess::InheritanceFolder => Self::Inheritance,
            proto::core::FolderAccess::ReadOpenFolder => Self::ReadOpen,
            proto::core::FolderAccess::PrivateFolder => Self::Private,
            proto::core::FolderAccess::OpenFolder => Self::Open,
        }
    }
}

#[derive(FromRow)]
pub struct FolderModal {
    /// 名字
    pub path: String,
    /// 创建时间
    pub create_time: PrimitiveDateTime,
    /// 更新时间
    pub update_time: PrimitiveDateTime,
    /// 访问权限
    pub access: FolderAccess,
    /// bucket 名
    pub bucket_name: String,
    /// 父名字
    pub father_path: String,
}
impl FolderModal {
    /// 创建
    pub async fn create(
        path: &str,
        access: impl Into<FolderAccess>,
        bucket_name: &str,
        father_path: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        let access: FolderAccess = access.into();
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        sqlx::query("insert into folder(path, create_time, update_time, access,bucket_name,father_path) values ($1,$2,$3,$4,$5,$6)")
            .bind(path)
            .bind(&time)
            .bind(&time)
            .bind(access)
            .bind(bucket_name)
            .bind(father_path)
            .execute(pool)
            .await?;
        Self::find_one(path, bucket_name, pool).await
    }
    /// 判断是否存在
    pub async fn exist(path: &str, bucket_name: &str, pool: &Pool<Postgres>) -> TonicResult<()> {
        sqlx::query("select * from folder where path = $1 and bucket_name = $2")
            .bind(path)
            .bind(bucket_name)
            .fetch_one(pool)
            .await?;
        Ok(())
    }
    /// 更新
    pub async fn update(
        path: &str,
        access: impl Into<FolderAccess>,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        sqlx::query(
            "update folder set access = $1, update_time = $2 where path = $3 and bucket_name=$4",
        )
        .bind(access.into())
        .bind(time)
        .bind(path)
        .bind(bucket_name)
        .execute(pool)
        .await?;
        Self::find_one(path, bucket_name, pool).await
    }
    /// 获取第一项
    pub async fn find_one(
        path: &str,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        let folder = sqlx::query_as(
            "select path,access,create_time,update_time,bucket_name,father_path from folder where path = $1 and bucket_name=$2",
        )
        .bind(path).bind(bucket_name)
        .fetch_one(pool)
        .await?;
        Ok(folder)
    }
    /// 删除某个 bucket 下所有
    pub async fn delete_by_bucket(bucket_name: &str, pool: &Pool<Postgres>) -> TonicResult<()> {
        sqlx::query("delete from folder where bucket_name = $1")
            .bind(bucket_name)
            .execute(pool)
            .await?;
        Ok(())
    }
    /// 删除某个 path 下所有
    pub async fn delete_by_path(
        bucket_name: &str,
        father_path: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<()> {
        sqlx::query("delete from folder where bucket_name = $1 and path like $2")
            .bind(bucket_name)
            .bind(format!("{}%", father_path))
            .execute(pool)
            .await?;
        Ok(())
    }
    /// 某个 path 下所有文件夹个数
    pub async fn count_by_path(
        bucket_name: &str,
        father_path: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<i64> {
        let father_path = if father_path == "/" {
            format!("{}%", father_path)
        } else {
            format!("{}/%", father_path)
        };
        let (count,): (i64,) = sqlx::query_as(
            "select count(path) from folder where bucket_name = $1 and path like $2",
        )
        .bind(bucket_name)
        .bind(father_path)
        .fetch_one(pool)
        .await?;
        Ok(count)
    }
    /// 判断读取访问权限
    #[cfg(feature = "recursion")]
    #[async_recursion]
    pub async fn read_open(
        path: &str,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<bool> {
        use crate::bucket::BucketModal;
        if path == "/" {
            return BucketModal::read_open(bucket_name, pool).await;
        }
        let Self {
            access,
            father_path,
            ..
        } = Self::find_one(path, bucket_name, pool).await?;
        Ok(match access {
            FolderAccess::Inheritance => Self::read_open(&father_path, bucket_name, pool).await?,
            FolderAccess::ReadOpen => true,
            FolderAccess::Private => false,
            FolderAccess::Open => true,
        })
    }
    /// 判断读取访问权限
    #[cfg(feature = "recursion")]
    #[async_recursion]
    pub async fn write_open(
        path: &str,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<bool> {
        use crate::bucket::BucketModal;
        if path == "/" {
            return BucketModal::write_open(bucket_name, pool).await;
        }
        let Self {
            access,
            father_path,
            ..
        } = Self::find_one(path, bucket_name, pool).await?;
        Ok(match access {
            FolderAccess::Inheritance => Self::write_open(&father_path, bucket_name, pool).await?,
            FolderAccess::ReadOpen => false,
            FolderAccess::Private => false,
            FolderAccess::Open => true,
        })
    }
    /// 获取列表
    pub async fn find_many_by_father_path(
        limit: u32,
        offset: u32,
        father_path: &str,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Vec<Self>> {
        let users = sqlx::query_as(
            "select path,access,create_time,update_time,bucket_name,father_path from folder where father_path = $1 and bucket_name=$2 offset $3 limit $4",
        )
        .bind(father_path)
        .bind(bucket_name)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;
        Ok(users)
    }
    /// 获取总数
    pub async fn count_by_father_path(
        bucket_name: &str,
        father_path: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<i64> {
        let (count,): (i64,) = sqlx::query_as(
            "select count(path) from folder where bucket_name = $1 and father_path=$2",
        )
        .bind(bucket_name)
        .bind(father_path)
        .fetch_one(pool)
        .await?;
        Ok(count)
    }
}
#[allow(clippy::from_over_into)]
impl Into<FolderInfo> for FolderModal {
    fn into(self) -> FolderInfo {
        let FolderModal {
            path,
            create_time,
            update_time,
            access,
            bucket_name,
            father_path,
            ..
        } = self;
        let access: i32 = match access {
            FolderAccess::Inheritance => 0,
            FolderAccess::ReadOpen => 1,
            FolderAccess::Private => 2,
            FolderAccess::Open => 3,
        };
        let create_time = (create_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        let update_time = (update_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        FolderInfo {
            access,
            create_time,
            update_time,
            path,
            bucket_name,
            father_path,
        }
    }
}
#[cfg(test)]
mod test {
    use sqlx::postgres::PgPoolOptions;

    use super::FolderModal;

    #[tokio::test]
    async fn test() {
        // 获取数据库连接池=]
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("postgres").unwrap())
            .await
            .unwrap();
        FolderModal::delete_by_path("as-sushao", "/dsd", &pool)
            .await
            .unwrap();
    }
}
