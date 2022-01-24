use std::env::VarError;
use tonic::Status;

/// result 生成需要
pub trait ToStatusResult<T> {
    fn to_status(self) -> Result<T, Status>;
}
pub trait ToStatus {
    fn to_status(self) -> Status;
}
impl<T, K> ToStatusResult<T> for Result<T, K>
where
    K: ToStatus,
{
    fn to_status(self) -> Result<T, Status> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err.to_status()),
        }
    }
}

impl ToStatus for VarError {
    fn to_status(self) -> Status {
        Status::internal("管理员配置缺失")
    }
}
#[cfg(feature = "password")]
impl ToStatus for pbkdf2::password_hash::Error {
    fn to_status(self) -> Status {
        Status::failed_precondition("hash 错误")
    }
}
#[cfg(feature = "jwt")]
impl ToStatus for jsonwebtoken::errors::Error {
    fn to_status(self) -> Status {
        Status::invalid_argument("token错误")
    }
}
#[cfg(feature = "sql")]
impl ToStatus for sqlx::error::Error {
    fn to_status(self) -> Status {
        Status::internal(format!("数据库错误:{self}"))
    }
}
impl ToStatus for proto::validation::ValidationErrors {
    fn to_status(self) -> Status {
        if let Some((name, err)) = &self.field_errors().into_iter().find(|_| true) {
            if let Some(err) = err.get(0) {
                return Status::invalid_argument(format!("{}{}", name, err.code));
            };
        };
        Status::invalid_argument("参数错误")
    }
}
#[cfg(feature = "mongo")]
impl ToStatus for mongodb::error::Error {
    fn to_status(self) -> Status {
        Status::internal(format!("数据库错误:{self}"))
    }
}
impl ToStatus for tonic::transport::Error {
    fn to_status(self) -> Status {
        Status::internal("内部连接错误")
    }
}
