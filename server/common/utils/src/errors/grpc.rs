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
impl ToStatus for pbkdf2::password_hash::Error {
    fn to_status(self) -> Status {
        Status::failed_precondition("hash 错误")
    }
}
impl ToStatus for jsonwebtoken::errors::Error {
    fn to_status(self) -> Status {
        Status::invalid_argument("token错误")
    }
}
impl ToStatus for sqlx::error::Error {
    fn to_status(self) -> Status {
        Status::internal(format!("数据库错误:{self}"))
    }
}
