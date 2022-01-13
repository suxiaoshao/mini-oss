use database::{Pool, Postgres};
use proto::{
    async_trait,
    auth::{check_server::Check, CheckReply, CheckRequest, Empty},
    Request, Response, Status,
};

use crate::validation::Claims;

pub struct CheckGreeter {
    pool: Pool<Postgres>,
}

impl CheckGreeter {
    pub fn new(pool: Pool<Postgres>) -> CheckGreeter {
        Self { pool }
    }
}

#[async_trait]
impl Check for CheckGreeter {
    async fn check_manager(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<Empty>, Status> {
        let auth = request.into_inner().auth;
        Claims::check_manager(auth)?;
        Ok(Response::new(Empty {}))
    }
    async fn check_user(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckReply>, Status> {
        let auth = request.into_inner().auth;
        let name = Claims::check_user(auth, &self.pool).await?;
        Ok(Response::new(CheckReply { name }))
    }
}
