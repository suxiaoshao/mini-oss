use std::sync::Arc;

use async_graphql::{ErrorExtensionValues, FieldError, FieldResult};
use tonic::Status;

pub trait ToFieldResult<T> {
    fn to_field(self) -> FieldResult<T>;
}
pub trait ToField {
    fn to_field(self) -> FieldError;
}
impl<T, K> ToFieldResult<T> for Result<T, K>
where
    K: ToField,
{
    fn to_field(self) -> FieldResult<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err.to_field()),
        }
    }
}
impl ToField for Status {
    fn to_field(self) -> FieldError {
        let mut extensions = ErrorExtensionValues::default();
        extensions.set("code", self.code() as i32);
        extensions.set("source", format!("{:#?}", self));
        FieldError {
            message: self.message().to_string(),
            source: Some(Arc::new(self)),
            extensions: Some(extensions),
        }
    }
}
impl ToField for tonic::transport::Error {
    fn to_field(self) -> FieldError {
        let mut extensions = ErrorExtensionValues::default();
        extensions.set("code", 17);
        extensions.set("source", format!("{:#?}", self));
        FieldError {
            message: "服务器内部链接错误".to_string(),
            source: Some(Arc::new(self)),
            extensions: Some(extensions),
        }
    }
}