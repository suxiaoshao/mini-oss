use std::env::var;

use database::{users::UserModal, Pool, Postgres};
use errors::{TonicError, TonicResult};
use jsonwebtoken::{encode, EncodingKey, Header};

use serde::{Deserialize, Serialize};

use super::{hash::validate_password_hash, jwt_decode::jwt_decode};

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
    fn to_token(&self) -> TonicResult<String> {
        let key = var("secret")?;
        let token = encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(key.as_bytes()),
        )?;
        Ok(token)
    }
    // 新建并生成token
    pub fn new_token(name: String, password: String) -> TonicResult<String> {
        Self::new(name, password).to_token()
    }
}

impl Claims {
    /// 管理员 token
    pub fn manager_token(name: String, password: String) -> TonicResult<String> {
        let n = var("manager_name")?;
        let p = var("manager_password")?;
        if name == n && password == p {
            Claims::new_token(name, password)
        } else {
            Err(TonicError::PasswordError)
        }
    }
    /// 用户生成 token
    pub async fn user_token(
        name: String,
        password: String,
        pool: &Pool<Postgres>,
    ) -> TonicResult<String> {
        let user = UserModal::find_one(&name, pool)
            .await
            .map_err(|_| TonicError::UserNotFound)?;
        // Verify password against PHC string
        if validate_password_hash(&password, &user.password).is_ok() {
            Claims::new_token(name, password)
        } else {
            Err(TonicError::PasswordError)
        }
    }
    /// 验证管理员
    pub fn check_manager(auth: String) -> TonicResult<()> {
        let n = var("manager_name")?;
        let p = var("manager_password")?;
        let chaim = jwt_decode::<Self>(&auth)?;
        if chaim.name != n || chaim.password != p {
            return Err(TonicError::PasswordError);
        };
        Ok(())
    }
    /// 验证用户
    pub async fn check_user(auth: String, pool: &Pool<Postgres>) -> TonicResult<UserModal> {
        let chaim = jwt_decode::<Self>(&auth)?;
        let user = UserModal::find_one(&chaim.name, pool)
            .await
            .map_err(|_| TonicError::UserNotFound)?;
        // Verify password against PHC string
        if validate_password_hash(&chaim.password, &user.password).is_err() {
            return Err(TonicError::PasswordError);
        }
        Ok(user)
    }
}
