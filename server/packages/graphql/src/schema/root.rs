use std::io::Read;

use async_graphql::{Context, FieldResult, Object, Upload};

use ::utils::errors::graphql::ToFieldResult;
use proto::core::object_client::ObjectClient;
use proto::core::DeleteObjectRequest;
use proto::{
    auth::{login_client::LoginClient, LoginReply, LoginRequest},
    core::{
        bucket_client::BucketClient, folder_client::FolderClient, DeleteBucketRequest,
        DeleteFolderRequest,
    },
    user::{
        self_manage_client::SelfManageClient, user_manage_client::UserManageClient,
        CreateUserRequest, DeleteUserRequest, GetListRequest, GetUserInfoRequest, GetUserRequest,
        UpdatePasswordRequest, UpdateUserInfoRequest, UpdateUserRequest,
    },
    Request,
};

use crate::schema::object::object_info::ObjectInfo;
use crate::schema::object::request::{CreateObjectRequest, UpdateObjectRequest};

use super::{
    bucket::{
        bucket_info::{BucketInfo, BucketList},
        request::{CreateBucketRequest, UpdateBucketRequest},
    },
    folder::{
        folder_info::FolderInfo,
        request::{CreateFolderRequest, UpdateFolderRequest},
    },
    user_info::{UserInfo, UserList},
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 管理员登陆
    async fn manager_login(&self, data: LoginRequest) -> FieldResult<String> {
        let mut client = LoginClient::connect("http://auth:80").await.to_field()?;
        let request = Request::new(data);
        let res = client.manager_login(request).await.to_field()?;
        Ok(res.get_ref().auth.to_string())
    }
    /// 用户登陆
    async fn user_login(&self, data: LoginRequest) -> FieldResult<String> {
        let mut client = LoginClient::connect("http://auth:80").await.to_field()?;
        let request = Request::new(data);
        let res = client.user_login(request).await.to_field()?;
        Ok(res.get_ref().auth.to_string())
    }
    /// 用户列表
    async fn user_list(&self, data: GetListRequest) -> FieldResult<UserList> {
        let mut client = UserManageClient::connect("http://user:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let reply = client.get_user_list(request).await?;
        Ok(UserList::from(reply.into_inner()))
    }
    /// 用户信息
    async fn user_info(&self, data: GetUserRequest) -> FieldResult<UserInfo> {
        let mut client = UserManageClient::connect("http://user:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let reply = client.get_user(request).await?;
        Ok(UserInfo::from(reply.into_inner()))
    }
    /// 获取自身用户信息
    async fn self_user_info(&self, data: GetUserInfoRequest) -> FieldResult<UserInfo> {
        let mut client = SelfManageClient::connect("http://user:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let reply = client.get_user_info(request).await?;
        Ok(UserInfo::from(reply.into_inner()))
    }
    /// 用户存储桶列表
    async fn bucket_list(&self, data: GetListRequest) -> FieldResult<BucketList> {
        let mut client = BucketClient::connect("http://core:80").await.to_field()?;
        let request = Request::new(data);
        let res = client.get_bucket_list(request).await.to_field()?;
        Ok(BucketList::from(res.into_inner()))
    }
}
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// 用户创建
    async fn manage_user_create(&self, data: CreateUserRequest) -> FieldResult<UserInfo> {
        let mut client = UserManageClient::connect("http://user:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let res = client.create_user(request).await.to_field()?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户更新
    async fn manage_user_update(&self, data: UpdateUserRequest) -> FieldResult<UserInfo> {
        let mut client = UserManageClient::connect("http://user:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let res = client.update_user(request).await.to_field()?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户删除
    async fn manage_user_delete(&self, data: DeleteUserRequest) -> FieldResult<bool> {
        let mut client = UserManageClient::connect("http://user:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        client.delete_user(request).await.to_field()?;
        Ok(true)
    }
    /// 用户更新密码
    async fn update_password(&self, data: UpdatePasswordRequest) -> FieldResult<String> {
        let mut client = SelfManageClient::connect("http://user:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let LoginReply { auth } = client
            .update_password(request)
            .await
            .to_field()?
            .into_inner();
        Ok(auth)
    }
    /// 用户更新信息
    async fn update_info(&self, data: UpdateUserInfoRequest) -> FieldResult<UserInfo> {
        let mut client = SelfManageClient::connect("http://user:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let res = client.update_user_info(request).await.to_field()?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 创建存储桶
    async fn create_bucket(&self, data: CreateBucketRequest) -> FieldResult<BucketInfo> {
        let mut client = BucketClient::connect("http://core:80").await.to_field()?;
        let request = Request::new(data.into());
        let res = client.create_bucket(request).await.to_field()?;
        Ok(BucketInfo::from(res.into_inner()))
    }
    /// 更新存储桶
    async fn update_bucket(&self, data: UpdateBucketRequest) -> FieldResult<BucketInfo> {
        let mut client = BucketClient::connect("http://core:80").await.to_field()?;
        let request = Request::new(data.into());
        let res = client.update_bucket(request).await.to_field()?;
        Ok(BucketInfo::from(res.into_inner()))
    }
    /// 删除存储桶
    async fn delete_bucket(&self, data: DeleteBucketRequest) -> FieldResult<bool> {
        let mut client = BucketClient::connect("http://core:80").await.to_field()?;
        let request = Request::new(data);
        client.delete_bucket(request).await.to_field()?;
        Ok(true)
    }
    /// 创建目录
    async fn create_folder(&self, data: CreateFolderRequest) -> FieldResult<FolderInfo> {
        let mut client = FolderClient::connect("http://core:80").await.to_field()?;
        let request = Request::new(data.into());
        let res = client.create_folder(request).await.to_field()?;
        Ok(FolderInfo::from(res.into_inner()))
    }
    /// 更新目录
    async fn update_folder(&self, data: UpdateFolderRequest) -> FieldResult<FolderInfo> {
        let mut client = FolderClient::connect("http://core:80").await.to_field()?;
        let request = Request::new(data.into());
        let res = client.update_folder(request).await.to_field()?;
        Ok(FolderInfo::from(res.into_inner()))
    }
    /// 删除目录
    async fn delete_folder(&self, data: DeleteFolderRequest) -> FieldResult<bool> {
        let mut client = FolderClient::connect("http://core:80").await.to_field()?;
        let request = Request::new(data);
        client.delete_folder(request).await.to_field()?;
        Ok(true)
    }
    /// 创建对象
    async fn create_object(
        &self,
        ctx: &Context<'_>,
        data: CreateObjectRequest,
        file: Upload,
    ) -> FieldResult<ObjectInfo> {
        let mut client = ObjectClient::connect("http://core:80").await.to_field()?;

        // 获取文件数据
        let file = file.value(ctx)?;
        let filename = file.filename;
        let mut content = vec![];
        let mut file = file.content;
        file.read_to_end(&mut content)?;
        // 获取输入数据
        let CreateObjectRequest {
            path,
            bucket_name,
            access,
            auth,
        } = data;
        let mut message = proto::core::CreateObjectRequest {
            path,
            filename,
            bucket_name,
            access: 0,
            content,
            auth,
        };
        message.set_access(access);
        let request = Request::new(message);
        let res = client.create_object(request).await.to_field()?;
        Ok(res.into_inner().into())
    }
    /// 更新对象
    async fn update_object(&self, data: UpdateObjectRequest) -> FieldResult<ObjectInfo> {
        let mut client = ObjectClient::connect("http://core:80").await.to_field()?;
        let request = Request::new(data.into());
        let res = client.update_object(request).await.to_field()?;
        Ok(res.into_inner().into())
    }
    /// 删除对象
    async fn delete_object(&self, data: DeleteObjectRequest) -> FieldResult<bool> {
        let mut client = ObjectClient::connect("http://core:80").await.to_field()?;
        let request = Request::new(data);
        client.delete_object(request).await.to_field()?;
        Ok(true)
    }
}
