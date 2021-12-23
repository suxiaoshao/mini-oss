use async_graphql::{ErrorExtensions, FieldError};
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
        self.extend_with(|err, e| match err {
            MyError::Status(status) => {
                e.set("code", status.code() as i32);
                e.set("message", status.message())
            }
            MyError::TransportError(_) => {
                e.set("code", 17);
                e.set("message", "网络链接错误")
            }
        })
    }
}
impl From<Status> for MyError {
    fn from(status: Status) -> Self {
        Self::Status(status)
    }
}
impl From<tonic::transport::Error> for MyError {
    fn from(error: tonic::transport::Error) -> Self {
        Self::TransportError(error)
    }
}
