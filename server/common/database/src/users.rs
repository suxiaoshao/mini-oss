use std::time::SystemTime;

use sqlx::{types::time::PrimitiveDateTime, Pool, Postgres};
use tonic::Status;

use proto::user_manage::UserInfo;
use utils::errors::grpc::ToStatusResult;
#[derive(Debug)]
pub struct User {
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
impl User {
    /// 创建
    pub async fn create(
        name: &str,
        password: &str,
        description: &Option<String>,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        if Self::exist(name, pool).await {
            return Err(Status::already_exists("用户名重复"));
        }
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
    async fn exist(name: &str, pool: &Pool<Postgres>) -> bool {
        sqlx::query("select * from users where name = $1")
            .bind(name)
            .fetch_one(pool)
            .await
            .is_ok()
    }
    /// 更新
    pub async fn update(
        name: &str,
        description: &Option<String>,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        if !Self::exist(name, pool).await {
            return Err(Status::not_found("该用户不存在"));
        }
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
    /// 删除用户
    pub async fn delete(name: &str, pool: &Pool<Postgres>) -> Result<(), Status> {
        if !Self::exist(name, pool).await {
            return Err(Status::not_found("该用户不存在"));
        }
        sqlx::query("delete from users where name = $1")
            .bind(name)
            .execute(pool)
            .await
            .to_status()?;
        Ok(())
    }
    /// 获取第一项
    pub async fn find_one(name: &str, pool: &Pool<Postgres>) -> Result<Self, Status> {
        let (name, password, create_time, update_time, description): (
            String,
            String,
            PrimitiveDateTime,
            PrimitiveDateTime,
            Option<String>,
        ) = sqlx::query_as(
            "select name,password,create_time,update_time,description from users where name = $1",
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .to_status()?;
        Ok(Self {
            name,
            password,
            create_time,
            update_time,
            description,
        })
    }
}
impl Into<UserInfo> for User {
    fn into(self) -> UserInfo {
        let User {
            name,
            create_time,
            update_time,
            description,
            ..
        } = self;
        let create_time = create_time.assume_utc().unix_timestamp();
        let update_time = update_time.assume_utc().unix_timestamp();
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

    use super::User;

    #[tokio::test]
    async fn test() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://sushao:sushao@localhost:5432/mini_oss")
            .await
            .unwrap();
        let a = User::find_one("suaho", &pool).await.unwrap();
        println!("{:?}", a);
    }
}