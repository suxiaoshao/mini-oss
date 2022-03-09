use errors::{TonicError, TonicResult};
use proto::middleware::client::login_client;
use proto::user::CheckRequest;

/// 验证管理员身份
pub async fn check_manager(auth: Option<String>) -> TonicResult<()> {
    match auth {
        Some(auth) => {
            let mut client = login_client(Some(auth.clone())).await?;
            client.check_manager(CheckRequest { auth }).await?;
            Ok(())
        }
        None => Err(TonicError::NoneAuth),
    }
}

/// 验证用户身份
pub async fn check_user(auth: Option<String>) -> TonicResult<String> {
    match auth {
        None => Err(TonicError::NoneAuth),
        Some(auth) => {
            let mut client = login_client(Some(auth.clone())).await?;
            let name = client
                .check_user(CheckRequest {
                    auth: auth.to_string(),
                })
                .await?;
            Ok(name.into_inner().name)
        }
    }
}
