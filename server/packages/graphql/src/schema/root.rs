use async_graphql::*;
use proto::{
    auth::{login_client::LoginClient, LoginRequest},
    user_manage::{manage_client::ManageClient, UserCreateInfo, UserDeleteInfo, UserUpdateInfo},
    Request,
};
use utils::errors::graphql::ToFieldResult;

use super::user_info::UserInfo;

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
        let mut client = ManageClient::connect("http://user-mng-service:80")
            .await
            .to_field()?;
        let request = Request::new(UserCreateInfo {
            name,
            password,
            auth,
            description,
        });
        let res = client.user_create(request).await.to_field()?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户更新
    async fn manage_user_update(
        &self,
        name: String,
        auth: String,
        description: Option<String>,
    ) -> FieldResult<UserInfo> {
        let mut client = ManageClient::connect("http://user-mng-service:80")
            .await
            .to_field()?;
        let request = Request::new(UserUpdateInfo {
            name,
            auth,
            description,
        });
        let res = client.user_update(request).await.to_field()?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户更新
    async fn manage_user_delete(&self, name: String, auth: String) -> FieldResult<bool> {
        let mut client = ManageClient::connect("http://user-mng-service:80")
            .await
            .to_field()?;
        let request = Request::new(UserDeleteInfo { name, auth });
        client.user_delete(request).await.to_field()?;
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
