use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};

use errors::TonicResult;

/// 密码加盐
pub fn password_to_hash(password: &str) -> TonicResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($pbkdf2-sha256$...)

    let password_hash = Pbkdf2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

/// 验证密码 hash
pub fn validate_password_hash(pre_hash: &str, hashed: &str) -> TonicResult<()> {
    let parsed_hash = PasswordHash::new(hashed)?;
    Pbkdf2.verify_password(pre_hash.as_bytes(), &parsed_hash)?;
    Ok(())
}
