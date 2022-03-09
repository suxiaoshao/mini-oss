use async_graphql::{ComplexObject, Context, SimpleObject};
use proto::core::{CountReply, FolderAccess, GetFolderRequest, SizeReply};
use proto::middleware::client::{folder_client, object_client};

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
}
#[ComplexObject]
impl FolderInfo {
    async fn folder_name(&self) -> GraphqlResult<String> {
        self.path
            .split('/')
            .nth_back(1)
            .map(|x| x.to_string())
            .ok_or(GraphqlError::ParseFolderName)
    }
    async fn folder_count<'ctx>(&self, ctx: &Context<'ctx>) -> GraphqlResult<i64> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = folder_client(auth).await?;
        let request = GetFolderRequest {
            bucket_name: self.bucket_name.clone(),
            path: self.path.clone(),
        };
        let CountReply { total } = client.get_total_by_folder(request).await?.into_inner();
        Ok(total)
    }
    async fn object_count<'ctx>(&self, ctx: &Context<'ctx>) -> GraphqlResult<i64> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = object_client(auth).await?;
        let request = GetFolderRequest {
            bucket_name: self.bucket_name.clone(),
            path: self.path.clone(),
        };
        let CountReply { total } = client.get_total_by_folder(request).await?.into_inner();
        Ok(total)
    }
    async fn object_size<'ctx>(&self, ctx: &Context<'ctx>) -> GraphqlResult<String> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = object_client(auth).await?;
        let request = GetFolderRequest {
            bucket_name: self.bucket_name.clone(),
            path: self.path.clone(),
        };
        let SizeReply { size } = client.get_size_by_folder(request).await?.into_inner();
        Ok(size)
    }
}

impl From<proto::core::FolderInfo> for FolderInfo {
    fn from(folder: proto::core::FolderInfo) -> Self {
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
        }
    }
}
