use std::sync::Arc;

use axum::http::header::AUTHORIZATION;
use axum::http::Request;
use axum::response::Response;
use axum_extra::middleware::Next;

use crate::errors::{OpenError, OpenResult};

pub(crate) struct Identity {
    pub(crate) bucket_name: String,
    pub(crate) auth: String,
}

pub(crate) async fn identity<B>(mut req: Request<B>, next: Next<B>) -> OpenResult<Response> {
    let bucket_name = req
        .headers()
        .get("bucket-name")
        .and_then(|h| h.to_str().ok())
        .ok_or(OpenError::NoneBucketName)?
        .to_string();
    let auth = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(OpenError::NoneAuthorization)?
        .to_string();
    req.extensions_mut()
        .insert(Arc::new(Identity { auth, bucket_name }));
    Ok(next.run(req).await)
}
