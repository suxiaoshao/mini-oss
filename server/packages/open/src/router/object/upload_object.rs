use crate::errors::response::OpenResponse;
use crate::errors::OpenResult;
use crate::extract::object_access::ObjectAccess;
use crate::extract::path_with_name::PathWithName;
use crate::middleware::identity::Identity;
use axum::body::Bytes;
use axum::extract::Extension;
use proto::core::CreateObjectRequest;
use proto::middleware::client::object_client;
use std::ops::Deref;
use std::sync::Arc;
use tonic::Request;

/// 上传文件
pub(crate) async fn upload_object(
    Extension(identity): Extension<Arc<Identity>>,
    access: ObjectAccess,
    path: PathWithName,
    body: Option<Bytes>,
) -> OpenResult<OpenResponse<bool>> {
    let Identity { bucket_name, auth } = identity.deref();
    let PathWithName { path, filename } = path;
    let access = access.0 as i32;
    let mut client = object_client(auth.clone()).await?;
    let request = Request::new(CreateObjectRequest {
        access,
        bucket_name: bucket_name.clone(),
        path,
        content: body.unwrap_or_default().to_vec(),
        filename,
    });
    client.create_object(request).await?;
    Ok(OpenResponse { data: true })
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use proto::{core::CreateObjectRequest, middleware::client::object_client, Request};
    #[tokio::test]
    async fn test() -> Result<()> {
        let mut client = object_client(None).await?;
        let request = Request::new(CreateObjectRequest {
            access: 0,
            bucket_name: "as-sushao".to_string(),
            path: "/%E6%AF%95%E8%AE%BE/".to_string(),
            content: vec![],
            filename: "test.txt".to_string(),
        });
        client.create_object(request).await?;
        Ok(())
    }
}
