use proto::auth::{check_server::Check, CheckReply, CheckRequest};
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::validation::Claims;

pub struct CheckGreeter {
    pool: Pool<Postgres>,
}

impl CheckGreeter {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> CheckGreeter {
        Self { pool }
    }
}

#[tonic::async_trait]
impl Check for CheckGreeter {
    async fn check_manager(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckReply>, Status> {
        let auth = request.into_inner().auth;
        Claims::check_manager(auth)?;
        Ok(Response::new(CheckReply {}))
    }
    async fn check_user(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckReply>, Status> {
        let auth = request.into_inner().auth;
        Claims::check_user(auth, &self.pool).await?;
        Ok(Response::new(CheckReply {}))
    }
}
