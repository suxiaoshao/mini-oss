use std::str::FromStr;

use axum::{
    headers::HeaderName,
    http::{
        header::{InvalidHeaderName, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
};
use tower_http::cors::{CorsLayer, Origin};

pub fn get_cors() -> Result<CorsLayer, InvalidHeaderName> {
    Ok(CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT])
        // allow requests from any origin
        .allow_origin(Origin::predicate(|_, _| true))
        .allow_headers(vec![
            CONTENT_TYPE,
            AUTHORIZATION,
            HeaderName::from_str("bucket-name")?,
            HeaderName::from_str("object-access")?,
        ])
        .allow_credentials(true))
}
