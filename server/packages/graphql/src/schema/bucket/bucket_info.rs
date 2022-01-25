use async_graphql::SimpleObject;
use proto::core::{BucketAccess, GetBucketListReply};

#[derive(SimpleObject)]
pub struct BucketInfo {
    /// 名字
    pub name: String,
    /// 创建时间
    pub create_time: i64,
    /// 更新时间
    pub update_time: i64,
    /// 访问权限
    pub access: BucketAccess,
    /// 用户名
    pub username: String,
}

impl From<proto::core::BucketInfo> for BucketInfo {
    fn from(user: proto::core::BucketInfo) -> Self {
        let access = user.access();
        let proto::core::BucketInfo {
            name,
            create_time,
            update_time,
            username,
            ..
        } = user;
        Self {
            name,
            create_time,
            update_time,
            access,
            username,
        }
    }
}
#[derive(SimpleObject)]
pub struct BucketList {
    /// 总数
    pub total: i64,
    /// 数据
    pub data: Vec<BucketInfo>,
}

impl From<GetBucketListReply> for BucketList {
    fn from(list_user: GetBucketListReply) -> BucketList {
        Self {
            total: list_user.total,
            data: list_user.data.into_iter().map(|x| x.into()).collect(),
        }
    }
}
