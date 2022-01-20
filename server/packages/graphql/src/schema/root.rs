use ::utils::errors::graphql::ToFieldResult;
use async_graphql::{FieldResult, Object};
use proto::{
    auth::{login_client::LoginClient, LoginReply, LoginRequest},
    core::{bucket_client::BucketClient, DeleteBucketRequest},
    user::{
        self_manage_client::SelfManageClient, user_manage_client::UserManageClient,
        CreateUserRequest, DeleteUserRequest, GetListRequest, GetUserInfoRequest, GetUserRequest,
        UpdatePasswordRequest, UpdateUserInfoRequest, UpdateUserRequest,
    },
    Request,
};

use super::{
    bucket::{
        bucket_info::{BucketInfo, BucketList},
        request,
    },
    user_info::{UserInfo, UserList},
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 管理员登陆
    async fn manager_login(&self, data: LoginRequest) -> FieldResult<String> {
        let mut client = LoginClient::connect("http://auth-service:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let res = client.manager_login(request).await.to_field()?;
        Ok(res.get_ref().auth.to_string())
    }
    /// 用户登陆
    async fn user_login(&self, data: LoginRequest) -> FieldResult<String> {
        let mut client = LoginClient::connect("http://auth-service:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let res = client.user_login(request).await.to_field()?;
        Ok(res.get_ref().auth.to_string())
    }
    /// 用户列表
    async fn user_list(&self, data: GetListRequest) -> FieldResult<UserList> {
        let mut client = UserManageClient::connect("http://user-service:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let reply = client.get_user_list(request).await?;
        Ok(UserList::from(reply.into_inner()))
    }
    /// 用户信息
    async fn user_info(&self, data: GetUserRequest) -> FieldResult<UserInfo> {
        let mut client = UserManageClient::connect("http://user-service:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let reply = client.get_user(request).await?;
        Ok(UserInfo::from(reply.into_inner()))
    }
    /// 获取自身用户信息
    async fn self_user_info(&self, data: GetUserInfoRequest) -> FieldResult<UserInfo> {
        let mut client = SelfManageClient::connect("http://user-service:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let reply = client.get_user_info(request).await?;
        Ok(UserInfo::from(reply.into_inner()))
    }
    /// 用户存储桶列表
    async fn bucket_list(&self, data: GetListRequest) -> FieldResult<BucketList> {
        let mut client = BucketClient::connect("http://core-service:80")
            .await
            .to_field()?;
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
        let mut client = UserManageClient::connect("http://user-service:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let res = client.create_user(request).await.to_field()?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户更新
    async fn manage_user_update(&self, data: UpdateUserRequest) -> FieldResult<UserInfo> {
        let mut client = UserManageClient::connect("http://user-service:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let res = client.update_user(request).await.to_field()?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户删除
    async fn manage_user_delete(&self, data: DeleteUserRequest) -> FieldResult<bool> {
        let mut client = UserManageClient::connect("http://user-service:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        client.delete_user(request).await.to_field()?;
        Ok(true)
    }
    /// 用户更新密码
    async fn update_password(&self, data: UpdatePasswordRequest) -> FieldResult<String> {
        let mut client = SelfManageClient::connect("http://user-service:80")
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
        let mut client = SelfManageClient::connect("http://user-service:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let res = client.update_user_info(request).await.to_field()?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 创建存储桶
    async fn create_bucket(&self, data: request::CreateBucketRequest) -> FieldResult<BucketInfo> {
        let mut client = BucketClient::connect("http://core-service:80")
            .await
            .to_field()?;
        let request = Request::new(data.into());
        let res = client.create_bucket(request).await.to_field()?;
        Ok(BucketInfo::from(res.into_inner()))
    }
    /// 更新存储桶
    async fn update_bucket(&self, data: request::UpdateBucketRequest) -> FieldResult<BucketInfo> {
        let mut client = BucketClient::connect("http://core-service:80")
            .await
            .to_field()?;
        let request = Request::new(data.into());
        let res = client.update_bucket(request).await.to_field()?;
        Ok(BucketInfo::from(res.into_inner()))
    }
    /// 删除存储桶
    async fn delete_bucket(&self, data: DeleteBucketRequest) -> FieldResult<bool> {
        let mut client = BucketClient::connect("http://core-service:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        client.delete_bucket(request).await.to_field()?;
        Ok(true)
    }
}
