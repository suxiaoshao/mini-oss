use std::ops::Deref;
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::Extension;
use tonic::Request;

use proto::core::object_client::ObjectClient;
use proto::core::CreateObjectRequest;

use crate::errors::response::OpenResponse;
use crate::errors::OpenResult;
use crate::errors::ToOpen;
use crate::middleware::identity::Identity;
use crate::middleware::object_access::ObjectAccess;
use crate::middleware::path_with_name::PathWithName;

pub(crate) async fn upload_object(
    Extension(identity): Extension<Arc<Identity>>,
    access: OpenResult<ObjectAccess>,
    path: OpenResult<PathWithName>,
    body: Option<Bytes>,
) -> OpenResult<OpenResponse<bool>> {
    let Identity { bucket_name, auth } = identity.deref();
    let PathWithName { path, filename } = path?;
    let access = access?.0 as i32;
    let mut client = ObjectClient::connect("http://core:80").await.to_open()?;
    let request = Request::new(CreateObjectRequest {
        access,
        bucket_name: bucket_name.clone(),
        auth: Some(auth.clone()),
        path,
        content: body.unwrap_or_default().to_vec(),
        filename,
    });
    client.create_object(request).await.to_open()?;
    Ok(OpenResponse { data: true })
}
