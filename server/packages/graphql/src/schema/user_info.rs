use async_graphql::SimpleObject;

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
