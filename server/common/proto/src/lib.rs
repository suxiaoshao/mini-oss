pub mod auth {
    tonic::include_proto!("auth");
}
pub mod user_manage {
    tonic::include_proto!("user_manage");
}
pub use tonic::{async_trait, transport::Server, Request, Response, Status};
