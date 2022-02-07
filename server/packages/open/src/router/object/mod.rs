use std::ops::Deref;
use std::sync::Arc;

use axum::extract::multipart::MultipartRejection;
use axum::extract::Multipart;
use axum::extract::{Extension, Path};
use tonic::Request;

use proto::core::object_client::ObjectClient;
use proto::core::{CreateObjectsRequest, UploadFile};

use crate::errors::response::OpenResponse;
use crate::errors::OpenResult;
use crate::errors::ToOpen;
use crate::middleware::identity::Identity;
use crate::middleware::object_access::ObjectAccess;

pub(crate) async fn upload_object(
    Extension(identity): Extension<Arc<Identity>>,
    access: OpenResult<ObjectAccess>,
    multipart: Result<Multipart, MultipartRejection>,
    Path(path): Path<String>,
) -> OpenResult<OpenResponse<bool>> {
    let Identity { bucket_name, auth } = identity.deref();
    let files = parse_multipart(multipart.to_open()?).await;
    let access = access?.0 as i32;
    let mut client = ObjectClient::connect("http://core:80").await.to_open()?;
    let request = Request::new(CreateObjectsRequest {
        access,
        bucket_name: bucket_name.clone(),
        auth: auth.clone(),
        path,
        files: files
            .into_iter()
            .map(|(filename, content)| UploadFile { filename, content })
            .collect(),
    });
    client.create_objects(request).await.to_open()?;
    Ok(OpenResponse { data: true })
}

async fn parse_multipart(mut multipart: Multipart) -> Vec<(String, Vec<u8>)> {
    let mut list = vec![];
    while let Ok(Some(field)) = multipart.next_field().await {
        let filename = field.file_name().map(|x| x.to_string());
        let data = field.bytes().await;
        if let (Some(filename), Ok(data)) = (filename, data) {
            list.push((filename, data.to_vec()));
        }
    }
    list
}
