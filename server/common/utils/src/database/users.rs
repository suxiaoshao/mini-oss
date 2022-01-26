use std::time::SystemTime;

use sqlx::{types::time::PrimitiveDateTime, FromRow, Pool, Postgres};
use tonic::Status;

use proto::user::UserInfo;

use crate::errors::grpc::ToStatusResult;

#[derive(Debug, FromRow)]
pub struct UserModal {
    /// 名字
    pub name: String,
    /// 密码
    pub password: String,
    /// 创建时间
    pub create_time: PrimitiveDateTime,
    /// 更新时间
    pub update_time: PrimitiveDateTime,
    /// 描述
    pub description: Option<String>,
}
impl UserModal {
    /// 创建
    pub async fn create(
        name: &str,
        password: &str,
        description: &Option<String>,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        sqlx::query("insert into users(name, create_time, update_time, password, description) values ($1,$2,$3,$4,$5)")
            .bind(name)
            .bind(&time)
            .bind(&time)
            .bind(password)
            .bind(description)
            .execute(pool)
            .await
            .to_status()?;
        Self::find_one(name, pool).await
    }
    /// 是否存在
    pub async fn exist(name: &str, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query("select * from users where name = $1")
            .bind(name)
            .fetch_one(pool)
            .await
            .map(|_| ())
    }
    /// 更新
    pub async fn update(
        name: &str,
        description: &Option<String>,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        sqlx::query("update users set description = $1, update_time = $2 where name = $3")
            .bind(description)
            .bind(time)
            .bind(name)
            .execute(pool)
            .await
            .to_status()?;
        Self::find_one(name, pool).await
    }
    /// 修改密码
    pub async fn update_password(
        name: &str,
        new_password: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        // 获取现在时间
        let time = PrimitiveDateTime::from(SystemTime::now());
        sqlx::query("update users set password = $1, update_time = $2 where name = $3")
            .bind(new_password)
            .bind(time)
            .bind(name)
            .execute(pool)
            .await
            .to_status()?;
        Self::find_one(name, pool).await
    }
    /// 删除用户
    pub async fn delete(name: &str, pool: &Pool<Postgres>) -> Result<(), Status> {
        sqlx::query("delete from users where name = $1")
            .bind(name)
            .execute(pool)
            .await
            .to_status()?;
        Ok(())
    }
    /// 获取第一项
    pub async fn find_one(name: &str, pool: &Pool<Postgres>) -> Result<Self, Status> {
        let user = sqlx::query_as(
            "select name,password,create_time,update_time,description from users where name = $1",
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .to_status()?;
        Ok(user)
    }
    /// 获取列表
    pub async fn find_many(
        limit: u32,
        offset: u32,
        pool: &Pool<Postgres>,
    ) -> Result<Vec<Self>, Status> {
        let users =sqlx::query_as("select name,password,create_time,update_time,description from users offset $1 limit $2").bind(offset).bind(limit).fetch_all(pool).await.to_status()?;
        Ok(users)
    }
    /// 获取总数
    pub async fn count(pool: &Pool<Postgres>) -> Result<i64, Status> {
        let (count,): (i64,) = sqlx::query_as("select count(name) from users")
            .fetch_one(pool)
            .await
            .to_status()?;
        Ok(count)
    }
}
#[allow(clippy::from_over_into)]
impl Into<UserInfo> for UserModal {
    fn into(self) -> UserInfo {
        let UserModal {
            name,
            create_time,
            update_time,
            description,
            ..
        } = self;
        let create_time = (create_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        let update_time = (update_time.assume_utc().unix_timestamp_nanos() / 1000000) as i64;
        UserInfo {
            name,
            description,
            create_time,
            update_time,
        }
    }
}
#[cfg(test)]
mod test {
    use sqlx::postgres::PgPoolOptions;

    use super::UserModal;

    #[tokio::test]
    async fn test() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://sushao:sushao@localhost:5432/mini_oss")
            .await
            .unwrap();
        let a = UserModal::find_many(10, 0, &pool).await.unwrap();
        let count = UserModal::count(&pool).await.unwrap();
        println!("{a:?} {count}");
    }
}
