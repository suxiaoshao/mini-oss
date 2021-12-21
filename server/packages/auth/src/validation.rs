use jsonwebtoken::{encode, EncodingKey, Header};
use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Pbkdf2,
};
use serde::{Deserialize, Serialize};
use tonic::{Code, Status};

use crate::database::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// 用户名
    name: String,
    /// 密码
    password: String,
    /// 超时时间
    exp: usize,
}

impl Claims {
    /// 管理员 token
    pub fn manager_token(name: String, password: String) -> Result<String, Status> {
        if let (Ok(n), Ok(p)) = (
            std::env::var("manager_name"),
            std::env::var("manager_password"),
        ) {
            if name == n && password == p {
                let claims = Claims::new(name, password);
                claims.to_token()
            } else {
                Err(Status::new(Code::InvalidArgument, "账号密码错误"))
            }
        } else {
            Err(Status::new(
                tonic::Code::FailedPrecondition,
                "管理员配置缺失",
            ))
        }
    }
    /// 用户生成 token
    pub async fn user_token(name: String, password: String) -> Result<String, Status> {
        let user = User::find_one(&name)
            .await
            .ok_or_else(|| Status::new(Code::NotFound, "没有此用户"))?;
        // Verify password against PHC string
        let parsed_hash = PasswordHash::new(&user.password)
            .map_err(|_| Status::new(Code::Internal, "密码解析错误"))?;
        if Pbkdf2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            let claims = Claims::new(name, password);
            claims.to_token()
        } else {
            Err(Status::new(Code::InvalidArgument, "账号密码错误"))
        }
    }
    /// 新建
    fn new(name: String, password: String) -> Self {
        Self {
            name,
            password,
            exp: 10000000000,
        }
    }
    fn to_token(&self) -> Result<String, Status> {
        if let Ok(key) = std::env::var("secret") {
            encode(
                &Header::default(),
                self,
                &EncodingKey::from_secret(key.as_bytes()),
            )
            .map_err(|_| -> Status { Status::new(Code::Internal, "令牌生成错误") })
        } else {
            Err(Status::new(
                tonic::Code::FailedPrecondition,
                "管理员配置缺失",
            ))
        }
    }
}
