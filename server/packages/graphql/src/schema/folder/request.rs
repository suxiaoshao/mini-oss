use async_graphql::InputObject;
use proto::core::{self, FolderAccess};
use proto::{IntoRequest, Request};

#[derive(InputObject)]
pub struct UpdateFolderRequest {
    /// 路径
    pub path: String,
    /// bucket 名
    pub bucket_name: String,
    /// 用户凭证
    pub auth: Option<String>,
    /// 访问控制
    pub access: FolderAccess,
}

impl IntoRequest<core::UpdateFolderRequest> for UpdateFolderRequest {
    fn into_request(self) -> Request<core::UpdateFolderRequest> {
        Request::new(self.into())
    }
}

impl From<UpdateFolderRequest> for core::UpdateFolderRequest {
    fn from(value: UpdateFolderRequest) -> core::UpdateFolderRequest {
        let UpdateFolderRequest {
            path,
            access,
            auth,
            bucket_name,
        } = value;
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
    pub auth: Option<String>,
    /// 访问控制
    pub access: FolderAccess,
}

impl IntoRequest<core::CreateFolderRequest> for CreateFolderRequest {
    fn into_request(self) -> Request<core::CreateFolderRequest> {
        Request::new(self.into())
    }
}

impl From<CreateFolderRequest> for core::CreateFolderRequest {
    fn from(value: CreateFolderRequest) -> core::CreateFolderRequest {
        let CreateFolderRequest {
            path,
            access,
            auth,
            father_path,
            bucket_name,
        } = value;
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
