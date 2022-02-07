use crate::middleware::identity::identity;
use axum::body::Body;
use axum::routing::put;
use axum::Router;
use axum_extra::middleware;
pub(crate) mod object;

use self::object::upload_object;

pub(crate) fn get_router() -> Router<Body> {
    Router::new()
        // `GET /` goes to `root`
        .route(
            "/*path",
            put(upload_object).route_layer(middleware::from_fn(identity)),
        )
}
