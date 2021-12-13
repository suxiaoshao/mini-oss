use async_graphql::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn delete(&self, a: i32) -> i32 {
        a
    }
}
