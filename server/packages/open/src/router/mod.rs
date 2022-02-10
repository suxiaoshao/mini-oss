use axum::body::Body;
use axum::routing::put;
use axum::Router;
use axum_extra::middleware;

use crate::middleware::identity::identity;
use crate::router::object::get_object::get_object;
use crate::router::object::upload_object::upload_object;

pub(crate) mod object;

pub(crate) fn get_router() -> Router<Body> {
    Router::new()
        // `GET /` goes to `root`
        .route(
            "/*path",
            put(upload_object)
                .get(get_object)
                .route_layer(middleware::from_fn(identity)),
        )
}
