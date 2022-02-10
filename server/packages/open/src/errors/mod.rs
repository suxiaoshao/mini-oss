use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tonic::Status;

use self::response::OpenErrorResponse;
use self::status::OpenStatus;

pub(crate) mod response;
mod status;
#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Error, Debug, Serialize)]
pub(crate) enum OpenError {
    #[error("内部连接错误")]
    TransportError,
    #[error("{}",.0.message)]
    Status(OpenStatus),
    #[error("未知错误")]
    #[allow(dead_code)]
    UnknownError,
    #[error("缺少存储桶名")]
    NoneBucketName,
    #[error("{}不是合法的对象访问权限",.0)]
    NotObjectAccess(String),
    #[error("{}不是合法的对象路径",.0)]
    NotObjectPath(String),
}

impl From<tonic::transport::Error> for OpenError {
    fn from(_: tonic::transport::Error) -> Self {
        OpenError::TransportError
    }
}

impl From<Status> for OpenError {
    fn from(value: Status) -> Self {
        Self::Status(value.into())
    }
}

pub(crate) type OpenResult<T> = Result<T, OpenError>;

impl IntoResponse for OpenError {
    fn into_response(self) -> Response {
        match serde_json::to_value(OpenErrorResponse::from(self)) {
            Ok(e) => Json(e),
            Err(_) => Json(json!({
                "code":"UnknownError",
                "message":"未知错误"
            })),
        }
        .into_response()
    }
}
