use ::utils::errors::graphql::ToFieldResult;
use async_graphql::{FieldResult, Object};
use proto::{
    auth::{login_client::LoginClient, LoginReply, LoginRequest},
    user::{
        self_manage_client::SelfManageClient, user_manage_client::UserManageClient,
        CreateUserRequest, DeleteUserRequest, GetUserInfoRequest, GetUserRequest, ListRequest,
        UpdatePasswordRequest, UpdateUserInfoRequest, UpdateUserRequest,
    },
    Request,
};

use super::user_info::{UserInfo, UserList};

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
    async fn user_list(&self, data: ListRequest) -> FieldResult<UserList> {
        let mut client = UserManageClient::connect("http://user-service:80")
            .await
            .to_field()?;
        let request = Request::new(data);
        let reply = client.list_user(request).await?;
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
    /// 用户更新
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
}
#[tokio::test]
async fn test() {
    let mut client = LoginClient::connect("http://localhost:80").await.unwrap();
    let request = Request::new(LoginRequest {
        name: "sushao".to_string(),
        password: "sushao".to_string(),
    });
    let res = client.manager_login(request).await.unwrap();
    println!("{}", res.get_ref().auth);
}
