#[cfg(any(feature = "password", feature = "future"))]
use crate::errors::grpc::ToStatusResult;
#[cfg(feature = "password")]
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
#[cfg(feature = "password")]
use proto::Status;

#[cfg(feature = "password")]
/// 密码加盐
pub fn password_to_hash(password: &str) -> Result<String, Status> {
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($pbkdf2-sha256$...)
    let password_hash = Pbkdf2
        .hash_password(password.as_bytes(), &salt)
        .to_status()?
        .to_string();
    Ok(password_hash)
}
#[cfg(feature = "password")]
/// 验证密码 hash
pub fn validate_password_hash(pre_hash: &str, hashed: &str) -> Result<(), Status> {
    let parsed_hash = PasswordHash::new(hashed).to_status()?;
    Pbkdf2
        .verify_password(pre_hash.as_bytes(), &parsed_hash)
        .to_status()
}

#[cfg(feature = "future")]
use futures::stream::{Stream, StreamExt};
#[cfg(feature = "future")]
/// 文件 hash
pub async fn file_hash(mut source: impl Stream<Item = Vec<u8>> + Unpin) -> String {
    let mut hasher = blake3::Hasher::new();
    while let Some(data) = source.next().await {
        hasher.update(data.as_slice());
    }
    let hash = hasher.finalize();
    hash.to_string()
}
