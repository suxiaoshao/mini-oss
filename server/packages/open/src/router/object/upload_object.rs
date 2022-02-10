use crate::errors::response::OpenResponse;
use crate::errors::OpenResult;
use crate::middleware::identity::Identity;
use crate::middleware::object_access::ObjectAccess;
use crate::middleware::path_with_name::PathWithName;
use axum::body::Bytes;
use axum::extract::Extension;
use proto::core::object_client::ObjectClient;
use proto::core::CreateObjectRequest;
use std::ops::Deref;
use std::sync::Arc;
use tonic::Request;

/// 上传文件
pub(crate) async fn upload_object(
    Extension(identity): Extension<Arc<Identity>>,
    access: OpenResult<ObjectAccess>,
    path: OpenResult<PathWithName>,
    body: Option<Bytes>,
) -> OpenResult<OpenResponse<bool>> {
    let Identity { bucket_name, auth } = identity.deref();
    let PathWithName { path, filename } = path?;
    let access = access?.0 as i32;
    let mut client = ObjectClient::connect("http://core:80").await?;
    let request = Request::new(CreateObjectRequest {
        access,
        bucket_name: bucket_name.clone(),
        auth: auth.clone(),
        path,
        content: body.unwrap_or_default().to_vec(),
        filename,
    });
    client.create_object(request).await?;
    Ok(OpenResponse { data: true })
}
