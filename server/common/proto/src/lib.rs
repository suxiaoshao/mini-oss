pub mod auth {
    tonic::include_proto!("auth");
}
pub mod user {
    tonic::include_proto!("user");
}
pub use tonic::{async_trait, transport::Server, Request, Response, Status};
