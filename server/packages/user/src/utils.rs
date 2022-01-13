use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};
use proto::{
    auth::{check_client::CheckClient, CheckRequest},
    Request, Status,
};
use utils::errors::grpc::ToStatusResult;

/// 验证管理员身份
pub async fn check_manager(auth: &str) -> Result<(), Status> {
    let mut client = CheckClient::connect("http://auth-service:80")
        .await
        .map_err(|_| Status::internal("内部连接错误"))?;
    let check_request = Request::new(CheckRequest {
        auth: auth.to_string(),
    });
    client.check_manager(check_request).await?;
    Ok(())
}

/// 验证用户身份
pub async fn check_user(auth: &str) -> Result<String, Status> {
    let mut client = CheckClient::connect("http://auth-service:80")
        .await
        .map_err(|_| Status::internal("内部连接错误"))?;
    let check_request = Request::new(CheckRequest {
        auth: auth.to_string(),
    });
    let name = client.check_user(check_request).await?;
    Ok(name.into_inner().name)
}

/// 密码加盐
pub fn to_hash(password: &str) -> Result<String, Status> {
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($pbkdf2-sha256$...)
    let password_hash = Pbkdf2
        .hash_password(password.as_bytes(), &salt)
        .to_status()?
        .to_string();
    Ok(password_hash)
}
