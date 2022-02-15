use async_graphql::{ComplexObject, SimpleObject};
use proto::core::{
    folder_client::FolderClient, object_client::ObjectClient, CountReply, FolderAccess,
    GetFolderRequest, SizeReply,
};

use crate::errors::{GraphqlError, GraphqlResult};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct FolderInfo {
    /// 路径
    pub path: String,
    /// 创建时间
    pub create_time: i64,
    /// 创建时间
    pub update_time: i64,
    /// bucket 名
    pub bucket_name: String,
    /// 访问控制
    pub access: FolderAccess,
    /// 路径
    pub father_path: String,
    #[graphql(skip)]
    pub auth: Option<String>,
}
#[ComplexObject]
impl FolderInfo {
    async fn folder_name(&self) -> GraphqlResult<String> {
        self.path
            .split('/')
            .last()
            .map(|x| x.to_string())
            .ok_or(GraphqlError::ParseFolderName)
    }
    async fn folder_count(&self) -> GraphqlResult<i64> {
        let mut client = FolderClient::connect("http://core:80").await?;
        let request = GetFolderRequest {
            auth: self.auth.clone(),
            bucket_name: self.bucket_name.clone(),
            path: self.path.clone(),
        };
        let CountReply { total } = client.get_total_by_folder(request).await?.into_inner();
        Ok(total)
    }
    async fn object_count(&self) -> GraphqlResult<i64> {
        let mut client = ObjectClient::connect("http://core:80").await?;
        let request = GetFolderRequest {
            auth: self.auth.clone(),
            bucket_name: self.bucket_name.clone(),
            path: self.path.clone(),
        };
        let CountReply { total } = client.get_total_by_folder(request).await?.into_inner();
        Ok(total)
    }
    async fn object_size(&self) -> GraphqlResult<i64> {
        let mut client = ObjectClient::connect("http://core:80").await?;
        let request = GetFolderRequest {
            auth: self.auth.clone(),
            bucket_name: self.bucket_name.clone(),
            path: self.path.clone(),
        };
        let SizeReply { size } = client.get_size_by_folder(request).await?.into_inner();
        Ok(size)
    }
}

impl From<(proto::core::FolderInfo, Option<String>)> for FolderInfo {
    fn from((folder, auth): (proto::core::FolderInfo, Option<String>)) -> Self {
        let access = folder.access();
        let proto::core::FolderInfo {
            path,
            create_time,
            update_time,
            father_path,
            bucket_name,
            ..
        } = folder;
        Self {
            path,
            create_time,
            update_time,
            access,
            bucket_name,
            father_path,
            auth,
        }
    }
}
