use async_graphql::*;
use proto::{
    auth::{login_client::LoginClient, LoginRequest},
    Request,
};
use utils::errors::graphql::ToFieldResult;

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
    async fn delete(&self, a: i32) -> i32 {
        a
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
