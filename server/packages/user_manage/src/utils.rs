use proto::auth::{check_client::CheckClient, CheckRequest};
use tonic::{Request, Status};

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
