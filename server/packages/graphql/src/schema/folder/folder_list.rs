use crate::schema::folder::folder_info::FolderInfo;
use crate::schema::object::object_info::ObjectInfo;
use async_graphql::{SimpleObject, Union};

#[derive(SimpleObject)]
pub struct FolderList {
    pub total: i64,
    pub data: Vec<FolderItem>,
}
#[derive(Union)]
pub enum FolderItem {
    Object(ObjectInfo),
    Folder(FolderInfo),
}
