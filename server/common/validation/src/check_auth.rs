use errors::TonicResult;
use proto::middleware::client::login_client;
use proto::{user::CheckRequest, Request};

/// 验证管理员身份
pub async fn check_manager(auth: &str) -> TonicResult<()> {
    let mut client = login_client(Some(auth.to_string())).await?;
    let check_request = Request::new(CheckRequest {
        auth: auth.to_string(),
    });
    client.check_manager(check_request).await?;
    Ok(())
}

/// 验证用户身份
pub async fn check_user(auth: &str) -> TonicResult<String> {
    let mut client = login_client(Some(auth.to_string())).await?;
    let check_request = Request::new(CheckRequest {
        auth: auth.to_string(),
    });
    let name = client.check_user(check_request).await?;
    Ok(name.into_inner().name)
}
