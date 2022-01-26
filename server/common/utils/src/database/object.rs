use sqlx::{types::time::PrimitiveDateTime, FromRow};

use super::folder::ObjectAccess;

#[derive(sqlx::Type)]
#[sqlx(type_name = "header_type")]
pub struct HeaderType {
    key: String,
    value: String,
}
#[derive(FromRow)]
pub struct FolderModal {
    /// 目录
    pub path: String,
    /// 创建时间
    pub create_time: PrimitiveDateTime,
    /// 更新时间
    pub update_time: PrimitiveDateTime,
    /// 访问权限
    pub access: ObjectAccess,
    /// bucket 名
    pub bucket_name: String,
    /// object_id
    pub object_id: String,
    /// 文件名
    pub filename: String,
    /// 文件摘要
    pub blake3: String,
    /// 文件大小
    pub size: i32,
    /// 自定义头部
    pub headers: Vec<HeaderType>,
}
