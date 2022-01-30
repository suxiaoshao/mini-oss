use async_graphql::InputObject;

use proto::core::{self, Header, ObjectAccess};

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
    /// 访问控制
    pub auth: String,
    /// 自定义 header
    pub headers: Vec<Header>,
}

#[allow(clippy::from_over_into)]
impl Into<core::UpdateObjectRequest> for UpdateObjectRequest {
    fn into(self) -> core::UpdateObjectRequest {
        let Self {
            path,
            filename,
            bucket_name,
            access,
            new_filename,
            auth,
            headers,
        } = self;
        let mut request = core::UpdateObjectRequest {
            path,
            access: 0,
            new_filename,
            auth,
            bucket_name,
            filename,
            headers,
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
    /// 访问控制
    pub auth: String,
}
