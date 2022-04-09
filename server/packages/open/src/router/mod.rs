use std::sync::Arc;

use axum::body::Body;
use axum::extract::Extension;
use axum::routing::put;
use axum::{middleware, Router};
use database::PgPoolOptions;

use crate::middleware::identity::identity;
use crate::middleware::request_stat::RequestStatLayer;
use crate::router::object::get_object::get_object;
use crate::router::object::upload_object::upload_object;
use anyhow::Result;
pub(crate) mod object;

pub(crate) async fn get_router() -> Result<Router<Body>> {
    // 获取数据库连接池
    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("postgres")?)
            .await?,
    );
    let router = Router::new()
        // `GET /` goes to `root`
        .route("/*path", put(upload_object).get(get_object))
        .layer(RequestStatLayer)
        .layer(Extension(pool))
        .layer(middleware::from_fn(identity));
    Ok(router)
}
