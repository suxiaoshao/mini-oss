use async_graphql::SimpleObject;

use proto::core::{Header, ObjectAccess};
#[derive(SimpleObject)]
pub struct ObjectInfo {
    /// 路径
    pub path: String,
    /// 文件名
    pub filename: String,
    /// bucket 名
    pub bucket_name: String,
    /// 访问控制
    pub access: ObjectAccess,
    /// 创建时间
    pub create_time: i64,
    /// 创建时间
    pub update_time: i64,
    /// 大小
    pub size: String,
    /// 摘要
    pub blake3: String,
    /// 自定义 header
    pub headers: Vec<Header>,
}

impl From<proto::core::ObjectInfo> for ObjectInfo {
    fn from(user: proto::core::ObjectInfo) -> Self {
        let access = user.access();
        let proto::core::ObjectInfo {
            path,
            bucket_name,
            create_time,
            update_time,
            filename,
            size,
            blake3,
            headers,
            ..
        } = user;
        Self {
            path,
            filename,
            bucket_name,
            access,
            create_time,
            update_time,
            size,
            blake3,
            headers,
        }
    }
}
