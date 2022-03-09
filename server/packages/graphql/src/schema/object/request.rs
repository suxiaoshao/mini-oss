use async_graphql::InputObject;

use proto::core::{self, Header, ObjectAccess};
use proto::{IntoRequest, Request};

#[derive(InputObject)]
pub struct HeaderType {
    pub key: String,
    pub value: String,
}

#[derive(InputObject)]
pub struct UpdateObjectRequest {
    /// 路径
    pub path: String,
    /// 旧文件名
    pub filename: String,
    /// bucket 名
    pub bucket_name: String,
    /// 访问控制
    pub access: ObjectAccess,
    /// 新文件名
    pub new_filename: String,
    /// 自定义 header
    pub headers: Vec<HeaderType>,
}

impl IntoRequest<core::UpdateObjectRequest> for UpdateObjectRequest {
    fn into_request(self) -> Request<core::UpdateObjectRequest> {
        Request::new(self.into())
    }
}

impl From<UpdateObjectRequest> for core::UpdateObjectRequest {
    fn from(value: UpdateObjectRequest) -> core::UpdateObjectRequest {
        let UpdateObjectRequest {
            path,
            filename,
            bucket_name,
            access,
            new_filename,
            headers,
        } = value;
        let mut request = core::UpdateObjectRequest {
            path,
            access: 0,
            new_filename,
            bucket_name,
            filename,
            headers: headers
                .into_iter()
                .map(|HeaderType { key, value }| Header { key, value })
                .collect(),
        };
        request.set_access(access);
        request
    }
}
#[derive(InputObject)]
pub struct CreateObjectRequest {
    /// 路径
    pub path: String,
    /// bucket 名
    pub bucket_name: String,
    /// 访问控制
    pub access: ObjectAccess,
}
