use jsonwebtoken::{encode, EncodingKey, Header};
use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Pbkdf2,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tonic::Status;

use crate::{
    database::User,
    utils::{jwt_decode, var},
};

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
    /// 新建
    fn new(name: String, password: String) -> Self {
        Self {
            name,
            password,
            exp: 10000000000,
        }
    }
    /// 生成 token
    fn to_token(&self) -> Result<String, Status> {
        let key = var("secret")?;
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(key.as_bytes()),
        )
        .map_err(|_| -> Status { Status::internal("令牌生成错误") })
    }
}

impl Claims {
    /// 管理员 token
    pub fn manager_token(name: String, password: String) -> Result<String, Status> {
        let n = var("manager_name")?;
        let p = var("manager_password")?;
        if name == n && password == p {
            let claims = Claims::new(name, password);
            claims.to_token()
        } else {
            Err(Status::invalid_argument("账号密码错误"))
        }
    }
    /// 用户生成 token
    pub async fn user_token(
        name: String,
        password: String,
        pool: &Pool<Postgres>,
    ) -> Result<String, Status> {
        let user = User::find_one(&name, pool)
            .await
            .ok_or_else(|| Status::not_found("没有此用户"))?;
        // Verify password against PHC string
        let parsed_hash =
            PasswordHash::new(&user.password).map_err(|_| Status::internal("密码解析错误"))?;
        if Pbkdf2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            let claims = Claims::new(name, password);
            claims.to_token()
        } else {
            Err(Status::invalid_argument("账号密码错误"))
        }
    }
    /// 验证管理员
    pub fn check_manager(auth: String) -> Result<(), Status> {
        let n = var("manager_name")?;
        let p = var("manager_password")?;
        let chaim = jwt_decode::<Self>(&auth)?;
        if chaim.name != n || chaim.password != p {
            return Err(Status::invalid_argument("身份令牌错误"));
        };
        Ok(())
    }
    /// 验证用户
    pub async fn check_user(auth: String, pool: &Pool<Postgres>) -> Result<(), Status> {
        let chaim = jwt_decode::<Self>(&auth)?;
        let user = User::find_one(&chaim.name, pool)
            .await
            .ok_or_else(|| Status::not_found("没有此用户"))?;
        // Verify password against PHC string
        let parsed_hash =
            PasswordHash::new(&user.password).map_err(|_| Status::internal("密码解析错误"))?;
        if Pbkdf2
            .verify_password(chaim.password.as_bytes(), &parsed_hash)
            .is_err()
        {
            return Err(Status::invalid_argument("账号密码错误"));
        }
        Ok(())
    }
}
