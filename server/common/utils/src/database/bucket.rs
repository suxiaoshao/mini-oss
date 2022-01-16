use sqlx::types::time::PrimitiveDateTime;

#[derive(sqlx::Type)]
#[sqlx(type_name = "access_type")]
pub enum Access {
    Open,
    ReadOpen,
    Private,
}

pub struct Bucket {
    /// 名字
    pub name: String,
    /// 创建时间
    pub create_time: PrimitiveDateTime,
    /// 更新时间
    pub update_time: PrimitiveDateTime,
    /// 访问权限
    pub access: Access,
    /// 用户名
    pub user_name: String,
}
