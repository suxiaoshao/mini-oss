use jsonwebtoken::{decode, DecodingKey, Validation};
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};
use serde::de::DeserializeOwned;
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
/// 获取环境变量
pub fn var(key: &str) -> Result<String, Status> {
    std::env::var(key).map_err(|_| Status::failed_precondition("管理员配置缺失"))
}
/// token 解码
pub fn jwt_decode<T: DeserializeOwned>(token: &str) -> Result<T, Status> {
    let key = var("secret")?;
    match decode::<T>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    ) {
        Ok(e) => Ok(e.claims),
        Err(e) => Err(match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                Status::deadline_exceeded("身份已超时")
            }
            _ => Status::invalid_argument("身份令牌错误"),
        }),
    }
}
