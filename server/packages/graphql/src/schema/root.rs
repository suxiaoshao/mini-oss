use async_graphql::*;
use proto::{
    auth::{login_client::LoginClient, LoginRequest},
    user_manage::{
        user_manage_client::UserManageClient, user_message_client::UserMessageClient,
        CreateUserRequest, DeleteUserRequest, GetUserRequest, ListRequest, UpdateUserRequest,
    },
    Request,
};
use utils::errors::graphql::ToFieldResult;

use super::user_info::{UserInfo, UserList};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 管理员登陆
    async fn manager_login(&self, name: String, password: String) -> FieldResult<String> {
        let mut client = LoginClient::connect("http://auth-service:80")
            .await
            .to_field()?;
        let request = Request::new(LoginRequest { name, password });
        let res = client.manager_login(request).await.to_field()?;
        Ok(res.get_ref().auth.to_string())
    }
    /// 用户登陆
    async fn user_login(&self, name: String, password: String) -> FieldResult<String> {
        let mut client = LoginClient::connect("http://auth-service:80")
            .await
            .to_field()?;
        let request = Request::new(LoginRequest { name, password });
        let res = client.user_login(request).await.to_field()?;
        Ok(res.get_ref().auth.to_string())
    }
    /// 用户列表
    async fn user_list(&self, limit: u32, offset: u32, auth: String) -> FieldResult<UserList> {
        let mut client = UserMessageClient::connect("http://user-mng-service:80")
            .await
            .to_field()?;
        let request = Request::new(ListRequest {
            limit,
            offset,
            auth,
        });
        let reply = client.list_user(request).await?;
        Ok(UserList::from(reply.into_inner()))
    }
    /// 用户信息
    async fn user_info(&self, name: String, auth: String) -> FieldResult<UserInfo> {
        let mut client = UserMessageClient::connect("http://user-mng-service:80")
            .await
            .to_field()?;
        let request = Request::new(GetUserRequest { name, auth });
        let reply = client.get_user(request).await?;
        Ok(UserInfo::from(reply.into_inner()))
    }
}
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// 用户创建
    async fn manage_user_create(
        &self,
        name: String,
        password: String,
        auth: String,
        description: Option<String>,
    ) -> FieldResult<UserInfo> {
        let mut client = UserManageClient::connect("http://user-mng-service:80")
            .await
            .to_field()?;
        let request = Request::new(CreateUserRequest {
            name,
            password,
            auth,
            description,
        });
        let res = client.create_user(request).await.to_field()?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户更新
    async fn manage_user_update(
        &self,
        name: String,
        auth: String,
        description: Option<String>,
    ) -> FieldResult<UserInfo> {
        let mut client = UserManageClient::connect("http://user-mng-service:80")
            .await
            .to_field()?;
        let request = Request::new(UpdateUserRequest {
            name,
            auth,
            description,
        });
        let res = client.update_user(request).await.to_field()?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户更新
    async fn manage_user_delete(&self, name: String, auth: String) -> FieldResult<bool> {
        let mut client = UserManageClient::connect("http://user-mng-service:80")
            .await
            .to_field()?;
        let request = Request::new(DeleteUserRequest { name, auth });
        client.delete_user(request).await.to_field()?;
        Ok(true)
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
    println!("{}", res.get_ref().auth.to_string());
}
