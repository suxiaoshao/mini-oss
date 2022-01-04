use axum::http::{header::CONTENT_TYPE, Method};
use tower_http::cors::{CorsLayer, Origin};

pub fn get_cors() -> CorsLayer {
    let origins = vec![
        "http://admin.mini-oss.top:30002".parse().unwrap(),
    ];
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Origin::list(origins))
        .allow_headers(vec![CONTENT_TYPE])
        .allow_credentials(true);
    cors
}
