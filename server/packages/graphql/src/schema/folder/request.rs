use async_graphql::InputObject;
use proto::core::{self, FolderAccess};
#[derive(InputObject)]
pub struct UpdateFolderRequest {
    /// 路径
    pub path: String,
    /// bucket 名
    pub bucket_name: String,
    /// 用户凭证
    pub auth: String,
    /// 访问控制
    pub access: FolderAccess,
}

#[allow(clippy::from_over_into)]
impl Into<core::UpdateFolderRequest> for UpdateFolderRequest {
    fn into(self) -> core::UpdateFolderRequest {
        let Self {
            path,
            access,
            auth,
            bucket_name,
        } = self;
        let mut request = core::UpdateFolderRequest {
            path,
            access: 0,
            auth,
            bucket_name,
        };
        request.set_access(access);
        request
    }
}
#[derive(InputObject)]
pub struct CreateFolderRequest {
    /// 路径
    pub path: String,
    /// bucket 名
    pub bucket_name: String,
    /// 路径
    pub father_path: String,
    /// 用户凭证
    pub auth: String,
    /// 访问控制
    pub access: FolderAccess,
}

#[allow(clippy::from_over_into)]
impl Into<core::CreateFolderRequest> for CreateFolderRequest {
    fn into(self) -> core::CreateFolderRequest {
        let Self {
            path,
            access,
            auth,
            father_path,
            bucket_name,
        } = self;
        let mut request = core::CreateFolderRequest {
            path,
            access: 0,
            auth,
            father_path,
            bucket_name,
        };
        request.set_access(access);
        request
    }
}
