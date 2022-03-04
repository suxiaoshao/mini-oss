use std::sync::Arc;

use database::{Pool, Postgres};
use proto::core::{GetTimeRequest, SizeReply};
use proto::{async_trait, core::request_server, Request, Response, Status};

#[derive(Clone)]
pub struct RequestGreeter {
    pool: Arc<Pool<Postgres>>,
}

impl RequestGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl request_server::Request for RequestGreeter {
    async fn get_upload_size_by_bucket(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<SizeReply>, Status> {
        todo!()
    }

    async fn get_download_size_by_bucket(
        &self,
        request: Request<GetTimeRequest>,
    ) -> Result<Response<SizeReply>, Status> {
        todo!()
    }
}
