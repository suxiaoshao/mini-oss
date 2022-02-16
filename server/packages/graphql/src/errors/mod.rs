use std::sync::Arc;

use async_graphql::{ErrorExtensionValues, ErrorExtensions};

use proto::{transport, Code, Status};

#[derive(Debug)]
pub enum GraphqlError {
    Status(Status),
    Transport,
    ParseFolderName,
}

impl GraphqlError {
    pub fn message(&self) -> &str {
        match self {
            GraphqlError::Status(status) => status.message(),
            GraphqlError::Transport => "内部连接错误",
            GraphqlError::ParseFolderName => "目录获取错误",
        }
    }
}

impl Clone for GraphqlError {
    fn clone(&self) -> Self {
        match self {
            Self::Status(status) => Self::Status(Status::new(status.code(), status.message())),
            Self::Transport => Self::Transport,
            GraphqlError::ParseFolderName => GraphqlError::ParseFolderName,
        }
    }
}

impl From<transport::Error> for GraphqlError {
    fn from(_: transport::Error) -> Self {
        Self::Transport
    }
}

impl From<Status> for GraphqlError {
    fn from(error: Status) -> Self {
        Self::Status(error)
    }
}

pub type GraphqlResult<T> = Result<T, GraphqlError>;

impl ErrorExtensions for GraphqlError {
    fn extend(&self) -> async_graphql::Error {
        let mut extensions = ErrorExtensionValues::default();
        extensions.set("source", format!("{self:#?}"));

        match self {
            GraphqlError::Status(status) => {
                let code = match status.code() {
                    Code::Ok => "Ok",
                    Code::Cancelled => "Cancelled",
                    Code::Unknown => "Unknown",
                    Code::InvalidArgument => "InvalidArgument",
                    Code::DeadlineExceeded => "DeadlineExceeded",
                    Code::NotFound => "NotFound",
                    Code::AlreadyExists => "AlreadyExists",
                    Code::PermissionDenied => "PermissionDenied",
                    Code::ResourceExhausted => "ResourceExhausted",
                    Code::FailedPrecondition => "FailedPrecondition",
                    Code::Aborted => "Aborted",
                    Code::OutOfRange => "OutOfRange",
                    Code::Unimplemented => "Unimplemented",
                    Code::Internal => "Internal",
                    Code::Unavailable => "Unavailable",
                    Code::DataLoss => "DataLoss",
                    Code::Unauthenticated => "Unauthenticated",
                };
                extensions.set("code", code);
            }
            GraphqlError::Transport => extensions.set("code", "Transport"),
            GraphqlError::ParseFolderName => extensions.set("code", "Internal"),
        };
        async_graphql::Error {
            message: self.message().to_string(),
            source: Some(Arc::new(self.clone())),
            extensions: Some(extensions),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<async_graphql::Error> for GraphqlError {
    fn into(self) -> async_graphql::Error {
        self.extend()
    }
}
