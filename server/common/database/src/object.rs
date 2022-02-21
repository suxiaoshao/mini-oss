use std::time::SystemTime;

use num_traits::ToPrimitive;
use sqlx::types::Decimal;
use sqlx::{types::time::PrimitiveDateTime, FromRow, Pool, Postgres};

use errors::{TonicError, TonicResult};
use proto::core::{Header, ObjectInfo};

use crate::folder::FolderModal;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "object_access_type")]
pub enum ObjectAccess {
    Inheritance,
    ReadOpen,
    Private,
}

impl From<proto::core::ObjectAccess> for ObjectAccess {
    fn from(access: proto::core::ObjectAccess) -> Self {
        match access {
            proto::core::ObjectAccess::InheritanceObject => Self::Inheritance,
            proto::core::ObjectAccess::ReadOpenObject => Self::ReadOpen,
            proto::core::ObjectAccess::PrivateObject => Self::Private,
        }
    }
}

#[derive(FromRow, Debug)]
pub struct ObjectModal {
    /// 目录
    pub path: String,
    /// 创建时间
    pub create_time: PrimitiveDateTime,
    /// 更新时间
    pub update_time: PrimitiveDateTime,
    /// 访问权限
    pub access: ObjectAccess,
    /// bucket 名
    pub bucket_name: String,
    /// object_id
    pub object_id: String,
    /// 文件名
    pub filename: String,
    /// 文件摘要
    pub blake3: String,
    /// 文件大小
    pub size: i64,
    /// 自定义头部
    pub headers: Vec<serde_json::Value>,
}

pub struct ObjectCreateInput<'a> {
    pub path: &'a str,
    pub access: &'a ObjectAccess,
    pub bucket_name: &'a str,
    pub filename: &'a str,
    pub blake3: &'a str,
    pub object_id: &'a str,
    pub size: i64,
    pub headers: &'a [Header],
}

impl ObjectModal {
    /// 创建
    pub async fn create<'a>(
        ObjectCreateInput {
            path,
            access,
            bucket_name,
            filename,
            blake3,
            object_id,
            size,
            headers,
        }: ObjectCreateInput<'a>,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        let mut bind_headers = vec![];
        for header in headers {
            bind_headers.push(serde_json::to_value(header)?);
        }
        sqlx::query(r#"insert into object
                            (path, create_time, update_time, access,bucket_name,filename,object_id,blake3,size,headers)
                            values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"#)
            .bind(path)
            .bind(&time)
            .bind(&time)
            .bind(access)
            .bind(bucket_name)
            .bind(filename)
            .bind(object_id)
            .bind(blake3)
            .bind(size)
            .bind(bind_headers)
            .execute(pool)
            .await?;
        Self::find_one(path, bucket_name, filename, pool).await
    }
    /// 获取第一项
    pub async fn find_one(
        path: &str,
        bucket_name: &str,
        filename: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        let object = sqlx::query_as(
            r##"select path,access,create_time,update_time,bucket_name,filename,object_id,blake3,size,headers
                        from object where path = $1 and bucket_name=$2 and filename = $3"##,
        )
        .bind(path).bind(bucket_name).bind(filename)
        .fetch_one(pool)
        .await
        ?;
        Ok(object)
    }
    /// 判断是否存在
    pub async fn exist(
        path: &str,
        bucket_name: &str,
        filename: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<()> {
        sqlx::query("select * from object where path = $1 and bucket_name = $2 and filename = $3")
            .bind(path)
            .bind(bucket_name)
            .bind(filename)
            .fetch_one(pool)
            .await?;
        Ok(())
    }
    /// 更新
    pub async fn update(
        path: &str,
        access: impl Into<ObjectAccess>,
        bucket_name: &str,
        filename: &str,
        new_filename: &str,
        headers: &[Header],
        pool: &Pool<Postgres>,
    ) -> TonicResult<Self> {
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        let mut bind_headers = vec![];
        for header in headers {
            bind_headers.push(serde_json::to_value(header)?);
        }
        sqlx::query(
            r#"update object set access = $1, update_time = $2,filename=$3,headers=$4
                where path = $5 and bucket_name=$6 and filename=$7"#,
        )
        .bind(access.into())
        .bind(time)
        .bind(new_filename)
        .bind(bind_headers)
        .bind(path)
        .bind(bucket_name)
        .bind(filename)
        .execute(pool)
        .await?;
        Self::find_one(path, bucket_name, new_filename, pool).await
    }
    /// 删除
    pub async fn delete(
        path: &str,
        bucket_name: &str,
        filename: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<()> {
        sqlx::query("delete from object where path = $1 and bucket_name = $2 and filename = $3")
            .bind(path)
            .bind(bucket_name)
            .bind(filename)
            .execute(pool)
            .await?;
        Ok(())
    }
    /// 删除某个 bucket 下所有
    pub async fn delete_by_bucket(bucket_name: &str, pool: &Pool<Postgres>) -> TonicResult<()> {
        sqlx::query("delete from object where bucket_name = $1")
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
        sqlx::query("delete from object where bucket_name = $1 and path like $2")
            .bind(bucket_name)
            .bind(format!("{}%", father_path))
            .execute(pool)
            .await?;
        Ok(())
    }
    /// 根据 paths 获取对象
    pub async fn find_by_paths(
        bucket_name: &str,
        father_path: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Vec<Self>> {
        let users = sqlx::query_as(r#"select * from object where path like $1 and bucket_name=$2"#)
            .bind(format!("{}%", father_path))
            .bind(bucket_name)
            .fetch_all(pool)
            .await?;
        Ok(users)
    }
    /// 获取列表
    pub async fn find_many_by_path(
        limit: u32,
        offset: u32,
        father_path: &str,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<Vec<Self>> {
        let users = sqlx::query_as(
            r#"select * from object where path = $1 and bucket_name=$2 offset $3 limit $4"#,
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
        let (count,): (i64,) =
            sqlx::query_as("select count(path) from object where bucket_name = $1 and path=$2")
                .bind(bucket_name)
                .bind(father_path)
                .fetch_one(pool)
                .await?;
        Ok(count)
    }
    /// 判断读取访问权限
    pub async fn read_open(
        path: &str,
        bucket_name: &str,
        filename: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<bool> {
        let Self { access, .. } = Self::find_one(path, bucket_name, filename, pool).await?;
        Ok(match access {
            ObjectAccess::Inheritance => FolderModal::read_open(path, bucket_name, pool).await?,
            ObjectAccess::ReadOpen => true,
            ObjectAccess::Private => false,
        })
    }
    /// 判断读取访问权限
    pub async fn write_open(
        path: &str,
        bucket_name: &str,
        filename: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<bool> {
        let Self { access, .. } = Self::find_one(path, bucket_name, filename, pool).await?;
        Ok(match access {
            ObjectAccess::Inheritance => FolderModal::write_open(path, bucket_name, pool).await?,
            _ => false,
        })
    }
    /// 某个 path 下所有对象个数
    pub async fn count_by_path(
        bucket_name: &str,
        father_path: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<i64> {
        let (count,): (i64,) = sqlx::query_as(
            "select count(path) from object where bucket_name = $1 and path like $2",
        )
        .bind(bucket_name)
        .bind(format!("{}%", father_path))
        .fetch_one(pool)
        .await?;
        Ok(count)
    }
    /// 某个 path 下所有对象大小
    pub async fn size_by_path(
        bucket_name: &str,
        father_path: &str,
        pool: &Pool<Postgres>,
    ) -> TonicResult<i64> {
        let (count,): (Option<Decimal>,) =
            sqlx::query_as("select sum(size) from object where bucket_name = $1 and path like $2")
                .bind(bucket_name)
                .bind(format!("{}%", father_path))
                .fetch_one(pool)
                .await?;
        Ok(count.and_then(|x| x.to_i64()).unwrap_or(0))
    }
}

impl TryFrom<ObjectModal> for ObjectInfo {
    type Error = TonicError;

    fn try_from(value: ObjectModal) -> TonicResult<ObjectInfo> {
        let ObjectModal {
            path,
            create_time,
            update_time,
            access,
            bucket_name,
            blake3,
            filename,
            headers,
            size,
            ..
        } = value;
        let access: i32 = match access {
            ObjectAccess::Inheritance => 0,
            ObjectAccess::ReadOpen => 1,
            ObjectAccess::Private => 2,
        };
        let create_time = (create_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        let update_time = (update_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        let mut new_headers = vec![];
        for header in headers {
            new_headers.push(serde_json::from_value(header)?);
        }
        Ok(ObjectInfo {
            access,
            create_time,
            update_time,
            path,
            bucket_name,
            filename,
            blake3,
            headers: new_headers,
            size,
        })
    }
}
