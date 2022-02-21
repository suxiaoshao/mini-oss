use std::env::VarError;

use thiserror::Error;
use tonic::Status;

pub type TonicResult<T> = Result<T, TonicError>;

#[derive(Error, Debug)]
pub enum TonicError {
    #[error("{}",.0.message())]
    Status(#[source] Status),
    #[error("管理员配置缺失")]
    NoneConfiguration(#[source] VarError),
    #[error("数据库错误:{}",.0)]
    Sqlx(#[source] sqlx::Error),
    #[error("验证错误:{}",.0)]
    Validate(#[source] validator::ValidationErrors),
    #[cfg(feature = "mongo")]
    #[error("mongo 数据库错误:{}",.0)]
    Mongo(#[source] mongodb::error::Error),
    #[cfg(feature = "mongo")]
    #[error("oid 错误:{}",.0)]
    Bson(#[source] mongodb::bson::oid::Error),
    #[cfg(feature = "mongo")]
    #[error("gridfs 错误:{}",.0)]
    MongoGridfs(#[source] mongodb_gridfs::GridFSError),
    #[error("内部连接错误:{}",.0)]
    Transport(#[source] tonic::transport::Error),
    #[cfg(feature = "json")]
    #[error("json 错误:{}",.0)]
    Json(#[source] serde_json::Error),
    #[cfg(feature = "password")]
    #[error("pbkdf2 错误")]
    Password,
    #[cfg(feature = "jwt")]
    #[error("jwt 错误:{}",.0)]
    Jwt(#[source] jsonwebtoken::errors::Error),
    #[error("账号密码错误")]
    PasswordError,
    #[error("没有此用户")]
    UserNotFound,
    #[error("身份已超时")]
    AuthTimeout,
    #[error("身份令牌错误")]
    TokenError,
    #[error("没有权限操作不属于你的存储桶")]
    BucketPermission,
    #[error("{}存储桶不存在",.0)]
    BucketNotFound(String),
    #[error("{}文件夹不存在",.0)]
    FolderNotFound(String),
    #[error("私有权限的文件夹/对象需要 auth")]
    NoneAuth,
    #[error("{}对象不存在",.0)]
    ObjectNotFound(String),
}

impl From<Status> for TonicError {
    fn from(value: Status) -> Self {
        Self::Status(value)
    }
}

impl From<tonic::transport::Error> for TonicError {
    fn from(value: tonic::transport::Error) -> Self {
        Self::Transport(value)
    }
}

impl From<sqlx::Error> for TonicError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

#[cfg(feature = "mongo")]
impl From<mongodb::error::Error> for TonicError {
    fn from(value: mongodb::error::Error) -> Self {
        Self::Mongo(value)
    }
}

#[cfg(feature = "mongo")]
impl From<mongodb_gridfs::GridFSError> for TonicError {
    fn from(value: mongodb_gridfs::GridFSError) -> Self {
        Self::MongoGridfs(value)
    }
}

#[cfg(feature = "mongo")]
impl From<mongodb::bson::oid::Error> for TonicError {
    fn from(value: mongodb::bson::oid::Error) -> Self {
        Self::Bson(value)
    }
}

impl From<VarError> for TonicError {
    fn from(value: VarError) -> Self {
        Self::NoneConfiguration(value)
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for TonicError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

#[cfg(feature = "jwt")]
impl From<jsonwebtoken::errors::Error> for TonicError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::Jwt(value)
    }
}

#[cfg(feature = "password")]
impl From<pbkdf2::password_hash::Error> for TonicError {
    fn from(_: pbkdf2::password_hash::Error) -> Self {
        Self::Password
    }
}

impl From<validator::ValidationErrors> for TonicError {
    fn from(value: validator::ValidationErrors) -> Self {
        Self::Validate(value)
    }
}

impl From<TonicError> for Status {
    fn from(error: TonicError) -> Self {
        let message = error.to_string();
        match error {
            TonicError::Status(e) => e,

            TonicError::NoneConfiguration(_) | TonicError::Transport(_) => {
                Status::failed_precondition(message)
            }

            TonicError::Sqlx(_) | TonicError::Json(_) => Status::internal(message),
            #[cfg(feature = "mongo")]
            TonicError::Mongo(_) | TonicError::Bson(_) | TonicError::MongoGridfs(_) => {
                Status::internal(message)
            }

            TonicError::Validate(_)
            | TonicError::UserNotFound
            | TonicError::BucketNotFound(_)
            | TonicError::FolderNotFound(_)
            | TonicError::NoneAuth
            | TonicError::ObjectNotFound(_) => Status::invalid_argument(message),

            TonicError::Password
            | TonicError::Jwt(_)
            | TonicError::PasswordError
            | TonicError::AuthTimeout
            | TonicError::TokenError => Status::unauthenticated(message),

            TonicError::BucketPermission => Status::permission_denied(message),
        }
    }
}
