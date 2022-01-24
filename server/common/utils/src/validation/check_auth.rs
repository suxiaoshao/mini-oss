use proto::{
    auth::{check_client::CheckClient, CheckRequest},
    Request, Status,
};

use crate::errors::grpc::ToStatusResult;

/// 验证管理员身份
pub async fn check_manager(auth: &str) -> Result<(), Status> {
    let mut client = CheckClient::connect("http://auth-service:80")
        .await
        .to_status()?;
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
        .to_status()?;
    let check_request = Request::new(CheckRequest {
        auth: auth.to_string(),
    });
    let name = client.check_user(check_request).await?;
    Ok(name.into_inner().name)
}
