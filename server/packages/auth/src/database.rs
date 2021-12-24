use sqlx::{types::time::PrimitiveDateTime, Pool, Postgres};

pub struct User {
    pub name: String,
    pub password: String,
    pub create_time: PrimitiveDateTime,
    pub update_time: PrimitiveDateTime,
}
impl User {
    pub async fn find_one(name: &str, pool: &Pool<Postgres>) -> Option<Self> {
        let (name, password, create_time, update_time): (
            String,
            String,
            PrimitiveDateTime,
            PrimitiveDateTime,
        ) = sqlx::query_as(
            "select `name`,`password`,`create_time`,`update_time` from users where `name` = ?",
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .ok()?;
        Some(Self {
            name,
            password,
            create_time,
            update_time,
        })
    }
}
