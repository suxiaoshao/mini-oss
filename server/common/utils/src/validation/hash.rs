use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use proto::Status;

use crate::errors::grpc::ToStatusResult;

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
/// 验证 hash
pub fn validate_password_hash(pre_hash: &str, hashed: &str) -> Result<(), Status> {
    let parsed_hash = PasswordHash::new(hashed).to_status()?;
    Pbkdf2
        .verify_password(pre_hash.as_bytes(), &parsed_hash)
        .to_status()
}
