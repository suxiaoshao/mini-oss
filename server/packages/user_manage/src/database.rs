use std::time::SystemTime;

use sqlx::{types::time::PrimitiveDateTime, Pool, Postgres};
use tonic::Status;

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
    pub async fn create(
        name: &str,
        password: &str,
        description: Option<&str>,
        pool: &Pool<Postgres>,
    ) -> Result<Self, Status> {
        let time = PrimitiveDateTime::from(SystemTime::now());
        sqlx::query("insert into users(name, create_time, update_time, password, description) values (?,?,?,?)").bind(name).bind(&time).bind(&time).bind(password).bind(description).execute(pool).await.map_err::<Status,_>(|e|todo!())?;
        todo!()
    }
}
