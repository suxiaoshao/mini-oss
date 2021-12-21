use sqlx::{postgres::PgPoolOptions, types::time::PrimitiveDateTime, Pool, Postgres};

pub struct User {
    pub name: String,
    pub password: String,
    pub create_time: PrimitiveDateTime,
    pub update_time: PrimitiveDateTime,
}
lazy_static! {
    static ref POOL: Pool<Postgres> = {
        use tokio::runtime::Runtime;
        Runtime::new().unwrap().block_on(async {
            PgPoolOptions::new()
                .max_connections(5)
                .connect(&std::env::var("postgres").unwrap())
                .await
                .unwrap()
        })
    };
}
impl User {
    pub async fn find_one(name: &str) -> Option<Self> {
        let (name, password, create_time, update_time): (
            String,
            String,
            PrimitiveDateTime,
            PrimitiveDateTime,
        ) = sqlx::query_as(
            "select `name`,`password`,`create_time`,`update_time` from users where `name` = ?",
        )
        .bind(name)
        .fetch_one(&*POOL)
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
