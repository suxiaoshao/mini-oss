use std::env::var;

use jsonwebtoken::{decode, DecodingKey, Validation};
use proto::Status;
use serde::de::DeserializeOwned;

use crate::errors::grpc::ToStatusResult;

/// token 解码
pub fn jwt_decode<T: DeserializeOwned>(token: &str) -> Result<T, Status> {
    let key = var("secret").to_status()?;
    match decode::<T>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    ) {
        Ok(e) => Ok(e.claims),
        Err(e) => Err(match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                Status::unauthenticated("身份已超时")
            }
            _ => Status::unauthenticated("身份令牌错误"),
        }),
    }
}