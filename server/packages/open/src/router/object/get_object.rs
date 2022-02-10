use std::ops::Deref;
use std::sync::Arc;

use axum::extract::Extension;
use futures::future;
use tonic::Request;

use proto::core::object_client::ObjectClient;
use proto::core::{GetObjectContentReply, GetObjectRequest, ObjectInfo};

use crate::errors::response::OpenResponse;
use crate::errors::OpenResult;
use crate::middleware::identity::Identity;
use crate::middleware::path_with_name::PathWithName;

/// 获取文件
pub(crate) async fn get_object(
    Extension(identity): Extension<Arc<Identity>>,
    path: OpenResult<PathWithName>,
) -> OpenResult<OpenResponse<bool>> {
    println!("xixi");
    let Identity { bucket_name, auth } = identity.deref();
    let PathWithName { path, filename } = path?;
    let mut client = ObjectClient::connect("http://core:80").await?;
    let request = GetObjectRequest {
        bucket_name: bucket_name.clone(),
        auth: auth.clone(),
        path,
        filename,
    };
    let (content, object_info) = future::try_join(
        client
            .clone()
            .get_object_content(Request::new(request.clone())),
        client.get_object(Request::new(request)),
    )
    .await?;
    let GetObjectContentReply { content } = content.into_inner();
    let ObjectInfo {
        update_time,
        blake3,
        headers,
        ..
    } = object_info.into_inner();
    println!("{} {} {} {:?}", content.len(), update_time, blake3, headers);
    Ok(OpenResponse { data: true })
}
