use async_graphql::SimpleObject;
use proto::user_manage::ListUserReply;

#[derive(SimpleObject)]
pub struct UserInfo {
    /// 名字
    pub name: String,
    /// 创建时间
    pub create_time: i64,
    /// 更新时间
    pub update_time: i64,
    /// 描述
    pub description: Option<String>,
}
impl From<proto::user_manage::UserInfo> for UserInfo {
    fn from(user: proto::user_manage::UserInfo) -> Self {
        let proto::user_manage::UserInfo {
            name,
            create_time,
            update_time,
            description,
        } = user;
        Self {
            name,
            create_time,
            update_time,
            description,
        }
    }
}
#[derive(SimpleObject)]
pub struct UserList {
    /// 总数
    pub total: i64,
    /// 数据
    pub data: Vec<UserInfo>,
}

impl From<ListUserReply> for UserList {
    fn from(list_user: ListUserReply) -> Self {
        Self {
            total: list_user.total,
            data: list_user.data.into_iter().map(|x| x.into()).collect(),
        }
    }
}
