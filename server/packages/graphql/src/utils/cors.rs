use axum::http::{header::CONTENT_TYPE, Method};
use tower_http::cors::{CorsLayer, Origin};

pub fn get_cors() -> CorsLayer {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Origin::predicate(|_, _| true))
        .allow_headers(vec![CONTENT_TYPE])
        .allow_credentials(true);
    cors
}
