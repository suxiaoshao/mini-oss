use std::time::SystemTime;

use sqlx::{types::time::PrimitiveDateTime, Pool, Postgres};

use proto::{core::FolderInfo, Status};

use crate::errors::grpc::ToStatusResult;

#[derive(sqlx::Type)]
#[sqlx(type_name = "object_access_type")]
pub enum ObjectAccess {
    Bucket,
    ReadOpen,
    Private,
}
impl From<proto::core::ObjectAccess> for ObjectAccess {
    fn from(access: proto::core::ObjectAccess) -> Self {
        match access {
            proto::core::ObjectAccess::BucketObject => Self::Bucket,
            proto::core::ObjectAccess::ReadOpenObject => Self::ReadOpen,
            proto::core::ObjectAccess::PrivateObject => Self::Private,
        }
    }
}

pub struct FolderModal {
    /// 名字
    pub path: String,
    /// 创建时间
    pub create_time: PrimitiveDateTime,
    /// 更新时间
    pub update_time: PrimitiveDateTime,
    /// 访问权限
    pub access: ObjectAccess,
    /// bucket 名
    pub bucket_name: String,
    /// 父名字
    pub father_path: String,
}
impl FolderModal {
    /// 创建
    pub async fn create(
        path: &str,
        access: impl Into<ObjectAccess>,
        bucket_name: &str,
        father_path: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        let access: ObjectAccess = access.into();
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
            .await
            .to_status()?;
        Self::find_one(path, bucket_name, pool).await
    }
    /// 判断是否存在
    pub async fn exist(
        path: &str,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("select * from folder where path = $1 and bucket_name = $2")
            .bind(path)
            .bind(bucket_name)
            .fetch_one(pool)
            .await
            .map(|_| ())
    }
    /// 更新
    pub async fn update(
        path: &str,
        access: impl Into<ObjectAccess>,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
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
        .await
        .to_status()?;
        Self::find_one(path, bucket_name, pool).await
    }
    /// 获取第一项
    pub async fn find_one(
        path: &str,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        let folder:RowFolder = sqlx::query_as(
            "select path,access,create_time,update_time,bucket_name,father_path from folder where path = $1 and bucket_name=$2",
        )
        .bind(path).bind(bucket_name)
        .fetch_one(pool)
        .await
        .to_status()?;
        Ok(folder.into())
    }
    /// 删除
    pub async fn delete(
        path: &str,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> Result<(), Status> {
        sqlx::query("delete from folder where path = $1 and bucket_name = $2")
            .bind(path)
            .bind(bucket_name)
            .execute(pool)
            .await
            .to_status()?;
        Ok(())
    }
    /// 获取列表
    pub async fn find_many_by_user(
        limit: u32,
        offset: u32,
        father_path: &str,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Vec<Self>, Status> {
        let users: Vec<RowFolder> = sqlx::query_as(
            "select path,access,create_time,update_time,bucket_name,father_path from folder where father_path = $1 and bucket_name=$2 offset $3 limit $4",
        )
        .bind(father_path)
        .bind(bucket_name)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await
        .to_status()?;
        Ok(users.into_iter().map(|x| x.into()).collect())
    }
    /// 获取总数
    pub async fn count_by_name(
        bucket_name: &str,
        father_path: &str,
        pool: &Pool<Postgres>,
    ) -> Result<i64, Status> {
        let (count,): (i64,) = sqlx::query_as(
            "select count(path) from folder where bucket_name = $1 and father_path=$2",
        )
        .bind(bucket_name)
        .bind(father_path)
        .fetch_one(pool)
        .await
        .to_status()?;
        Ok(count)
    }
}
type RowFolder = (
    String,
    ObjectAccess,
    PrimitiveDateTime,
    PrimitiveDateTime,
    String,
    String,
);
impl From<RowFolder> for FolderModal {
    fn from((path, access, create_time, update_time, bucket_name, father_path): RowFolder) -> Self {
        Self {
            path,
            create_time,
            update_time,
            access,
            bucket_name,
            father_path,
        }
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
            ObjectAccess::Bucket => 0,
            ObjectAccess::ReadOpen => 1,
            ObjectAccess::Private => 2,
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
