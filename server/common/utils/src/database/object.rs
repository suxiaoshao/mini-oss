use std::time::SystemTime;

use sqlx::{types::time::PrimitiveDateTime, FromRow, Pool, Postgres};

use proto::{
    core::{Header, ObjectInfo},
    Status,
};

use crate::errors::grpc::ToStatusResult;

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
    ) -> Result<Self, Status> {
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        let mut bind_headers = vec![];
        for header in headers {
            bind_headers.push(serde_json::to_value(header).to_status()?);
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
            .await
            .to_status()?;
        Self::find_one(path, bucket_name, filename, pool).await
    }
    /// 获取第一项
    pub async fn find_one(
        path: &str,
        bucket_name: &str,
        filename: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        let object = sqlx::query_as(
            r##"select path,access,create_time,update_time,bucket_name,filename,object_id,blake3,size,headers
                        from object where path = $1 and bucket_name=$2 and filename = $3"##,
        )
        .bind(path).bind(bucket_name).bind(filename)
        .fetch_one(pool)
        .await
        .to_status()?;
        Ok(object)
    }
    /// 判断是否存在
    pub async fn exist(
        path: &str,
        bucket_name: &str,
        filename: &str,
        pool: &Pool<Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("select * from object where path = $1 and bucket_name = $2 and filename = $3")
            .bind(path)
            .bind(bucket_name)
            .bind(filename)
            .fetch_one(pool)
            .await
            .map(|_| ())
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
    ) -> Result<Self, Status> {
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        let mut bind_headers = vec![];
        for header in headers {
            bind_headers.push(serde_json::to_value(header).to_status()?);
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
        .await
        .to_status()?;
        Self::find_one(path, bucket_name, new_filename, pool).await
    }
    /// 删除
    pub async fn delete(
        path: &str,
        bucket_name: &str,
        filename: &str,
        pool: &Pool<Postgres>,
    ) -> Result<(), Status> {
        sqlx::query("delete from object where path = $1 and bucket_name = $2 and filename = $3")
            .bind(path)
            .bind(bucket_name)
            .bind(filename)
            .execute(pool)
            .await
            .to_status()?;
        Ok(())
    }
    /// 删除某个 bucket 下所有
    pub async fn delete_by_bucket(bucket_name: &str, pool: &Pool<Postgres>) -> Result<(), Status> {
        sqlx::query("delete from object where bucket_name = $1")
            .bind(bucket_name)
            .execute(pool)
            .await
            .to_status()?;
        Ok(())
    }
    /// 根据 paths 获取对象
    #[cfg(feature = "future")]
    pub async fn find_by_paths(
        bucket_name: &str,
        paths: &[String],
        pool: &Pool<Postgres>,
    ) -> Result<Vec<Self>, Status> {
        let mut result = vec![];
        futures::future::try_join_all(
            paths
                .iter()
                .map(|path| Self::find_total_by_path(path, bucket_name, pool)),
        )
        .await?
        .into_iter()
        .for_each(|mut item| result.append(&mut item));
        Ok(result)
    }
    /// 获取列表
    pub async fn find_total_by_path(
        father_path: &str,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Vec<Self>, Status> {
        let users = sqlx::query_as(r#"select * from object where path = $1 and bucket_name=$2"#)
            .bind(father_path)
            .bind(bucket_name)
            .fetch_all(pool)
            .await
            .to_status()?;
        Ok(users)
    }
    /// 获取列表
    pub async fn find_many_by_path(
        limit: u32,
        offset: u32,
        father_path: &str,
        bucket_name: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Vec<Self>, Status> {
        let users = sqlx::query_as(
            r#"select * from object where path = $1 and bucket_name=$2 offset $3 limit $4"#,
        )
        .bind(father_path)
        .bind(bucket_name)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await
        .to_status()?;
        Ok(users)
    }
    /// 获取总数
    pub async fn count_by_father_path(
        bucket_name: &str,
        father_path: &str,
        pool: &Pool<Postgres>,
    ) -> Result<i64, Status> {
        let (count,): (i64,) =
            sqlx::query_as("select count(path) from object where bucket_name = $1 and path=$2")
                .bind(bucket_name)
                .bind(father_path)
                .fetch_one(pool)
                .await
                .to_status()?;
        Ok(count)
    }
}
#[allow(clippy::from_over_into)]
impl TryInto<ObjectInfo> for ObjectModal {
    fn try_into(self) -> Result<ObjectInfo, Status> {
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
        } = self;
        let access: i32 = match access {
            ObjectAccess::Inheritance => 0,
            ObjectAccess::ReadOpen => 1,
            ObjectAccess::Private => 2,
        };
        let create_time = (create_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        let update_time = (update_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        let mut new_headers = vec![];
        for header in headers {
            new_headers.push(serde_json::from_value(header).to_status()?);
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

    type Error = Status;
}
