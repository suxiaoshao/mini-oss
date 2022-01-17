pub mod auth {
    tonic::include_proto!("auth");
}
pub mod user {
    tonic::include_proto!("user");
}
pub mod core {
    use tonic::Status;

    tonic::include_proto!("core");
    impl TryFrom<i32> for Access {
        type Error = Status;
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Open),
                1 => Ok(Self::ReadOpen),
                2 => Ok(Self::Private),
                other => Err(Status::invalid_argument(format!(
                    "{other} 不是合法的 access 值"
                ))),
            }
        }
    }
}
pub use tonic::{async_trait, transport::Server, Request, Response, Status};
