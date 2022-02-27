use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};

use crate::errors::OpenError;

pub(crate) struct ObjectAccess(pub proto::core::ObjectAccess);

#[async_trait]
impl<B> FromRequest<B> for ObjectAccess
where
    B: Send, // required by `async_trait`
{
    type Rejection = OpenError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let access = match req
            .headers()
            .and_then(|h| h.get("object-access"))
            .and_then(|x| x.to_str().ok())
        {
            None => proto::core::ObjectAccess::InheritanceObject,
            Some("InheritanceObject" | "INHERITANCE_OBJECT") => {
                proto::core::ObjectAccess::InheritanceObject
            }
            Some("PrivateObject" | "PRIVATE_OBJECT") => proto::core::ObjectAccess::PrivateObject,
            Some("ReadOpenObject" | "READ_OPEN_OBJECT") => {
                proto::core::ObjectAccess::ReadOpenObject
            }
            Some(s) => {
                return Err(Self::Rejection::NotObjectAccess(s.to_string()));
            }
        };
        Ok(ObjectAccess(access))
    }
}
