use std::env::var;

use errors::{TonicError, TonicResult};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::de::DeserializeOwned;

/// token 解码
pub fn jwt_decode<T: DeserializeOwned>(token: &str) -> TonicResult<T> {
    let key = var("secret")?;
    match decode::<T>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    ) {
        Ok(e) => Ok(e.claims),
        Err(e) => Err(match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => TonicError::AuthTimeout,
            _ => TonicError::TokenError,
        }),
    }
}
