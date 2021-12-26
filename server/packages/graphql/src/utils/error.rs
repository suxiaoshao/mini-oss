use async_graphql::{ErrorExtensionValues, ErrorExtensions, FieldError};
use thiserror::Error;
use tonic::Status;
#[derive(Debug, Error)]
pub enum MyError {
    #[error("{}:{}",.0.code(),.0.message())]
    Status(Status),
    #[error("网络错误")]
    TransportError(tonic::transport::Error),
}

impl ErrorExtensions for MyError {
    // lets define our base extensions
    fn extend(&self) -> FieldError {
        match self {
            MyError::Status(status) => {
                let mut extensions = ErrorExtensionValues::default();
                extensions.set("code", status.code() as i32);
                extensions.set("source", format!("{:#?}", status));
                FieldError {
                    message: status.message().to_string(),
                    source: None,
                    extensions: Some(extensions),
                }
            }
            MyError::TransportError(err) => {
                let mut extensions = ErrorExtensionValues::default();
                extensions.set("code", 17);
                extensions.set("source", format!("{:#?}", err));
                FieldError {
                    message: "服务器内部链接错误".to_string(),
                    source: None,
                    extensions: Some(extensions),
                }
            }
        }
    }
}
pub trait ToMyResult<T> {
    fn to_my_result(self) -> Result<T, MyError>;
}
impl<M> ToMyResult<M> for Result<M, Status> {
    fn to_my_result(self) -> Result<M, MyError> {
        match self {
            Ok(o) => Ok(o),
            Err(err) => Err(MyError::Status(err)),
        }
    }
}
impl<M> ToMyResult<M> for Result<M, tonic::transport::Error> {
    fn to_my_result(self) -> Result<M, MyError> {
        match self {
            Ok(o) => Ok(o),
            Err(err) => Err(MyError::TransportError(err)),
        }
    }
}
#[macro_export]
macro_rules! my_result {
    ($expression:expr) => {{
        use crate::utils::error::ToMyResult;
        $expression.await.to_my_result().extend()?
    }};
}
