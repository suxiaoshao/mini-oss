use async_graphql::{ComplexObject, FieldResult, SimpleObject};
use proto::core::FolderAccess;
use utils::errors::graphql::ToFieldResult;

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
    async fn folder_name(&self) -> FieldResult<String> {
        self.path
            .split('/')
            .last()
            .map(|x| x.to_string())
            .to_field()
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
