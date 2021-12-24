use crate::my_result;
use async_graphql::*;
use proto::auth::{login_client::LoginClient, LoginRequest};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 管理员登陆
    async fn manager_login(&self, name: String, password: String) -> FieldResult<String> {
        let mut client = my_result!(LoginClient::connect("http://auth-service:80"));
        let request = tonic::Request::new(LoginRequest { name, password });
        let res = my_result!(client.manager_login(request));
        Ok(res.get_ref().auth.to_string())
    }
    /// 用户登陆
    async fn user_login(&self, name: String, password: String) -> FieldResult<String> {
        let mut client = my_result!(LoginClient::connect("http://auth-service:80"));
        let request = tonic::Request::new(LoginRequest { name, password });
        let res = my_result!(client.user_login(request));
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
    let request = tonic::Request::new(LoginRequest {
        name: "sushao".to_string(),
        password: "sushao".to_string(),
    });
    let res = client.manager_login(request).await.unwrap();
    println!("{}", res.get_ref().auth.to_string());
}
