use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use tonic::{Code, Status};

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
    pub fn manager_token(name: String, password: String) -> Result<String, Status> {
        if let (Ok(n), Ok(p)) = (
            std::env::var("manager_name"),
            std::env::var("manager_password"),
        ) {
            if name == n && password == p {
                let claims = Claims::new(name, password);
                claims.to_token()
            } else {
                todo!()
            }
        } else {
            Err(Status::new(
                tonic::Code::FailedPrecondition,
                "管理员配置缺失",
            ))
        }
    }
    pub fn user_token(name: String, password: String) -> Result<String, Status> {
        todo!()
    }
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
