use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};
use tonic::Status;

pub fn to_hash(password: &str) -> Result<String, Status> {
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($pbkdf2-sha256$...)
    let password_hash = Pbkdf2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| Status::internal("密码解析错误"))?
        .to_string();
    Ok(password_hash)
}
