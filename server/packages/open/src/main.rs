use std::net::SocketAddr;

use crate::router::get_router;

mod errors;
mod middleware;
mod router;

#[tokio::main]
async fn main() {
    // 获取路由
    let app = get_router();

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
