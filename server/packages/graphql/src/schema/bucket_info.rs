use async_graphql::SimpleObject;
use proto::{
    core::{Access, GetBucketListReply},
    Status,
};

#[derive(SimpleObject)]
pub struct BucketInfo {
    /// 名字
    pub name: String,
    /// 创建时间
    pub create_time: i64,
    /// 更新时间
    pub update_time: i64,
    /// 访问权限
    pub access: Access,
    /// 用户名
    pub user_name: String,
}

impl TryFrom<proto::core::BucketInfo> for BucketInfo {
    fn try_from(user: proto::core::BucketInfo) -> Result<Self, Status> {
        let proto::core::BucketInfo {
            name,
            create_time,
            update_time,
            access,
            user_name,
        } = user;
        let access = Access::try_from(access)?;
        Ok(Self {
            name,
            create_time,
            update_time,
            access,
            user_name,
        })
    }

    type Error = Status;
}
#[derive(SimpleObject)]
pub struct BucketList {
    /// 总数
    pub total: i64,
    /// 数据
    pub data: Vec<BucketInfo>,
}

impl TryFrom<GetBucketListReply> for BucketList {
    fn try_from(list_user: GetBucketListReply) -> Result<BucketList, Status> {
        let mut data = vec![];
        for bucket_info in list_user.data {
            data.push(BucketInfo::try_from(bucket_info)?);
        }
        Ok(Self {
            total: list_user.total,
            data,
        })
    }

    type Error = Status;
}
