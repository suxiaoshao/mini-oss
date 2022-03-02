use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use axum::{
    body::BoxBody,
    http::{Request, Response},
};
use bson::oid::ObjectId;
use futures::{ready, Future};
use pin_project_lite::pin_project;
use tower::{Layer, Service};

use database::{Pool, Postgres};

use self::{request_body::RequestBody, response_body::ResponseBody};

use super::identity::Identity;

pub mod handle_error;
pub mod request_body;
pub mod response_body;

/// Apply a transformation to the request body.
///
/// See the [module docs](crate::map_request_body) for an example.
#[derive(Clone)]
pub struct RequestStatLayer;
impl<S> Layer<S> for RequestStatLayer {
    type Service = RequestStat<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestStat::new(inner)
    }
}

/// Apply a transformation to the request body.
///
/// See the [module docs](crate::map_request_body) for an example.
#[derive(Clone)]
pub struct RequestStat<S> {
    inner: S,
}

impl<S> RequestStat<S> {
    /// Create a new [`MapRequestBody`].
    ///
    /// `F` is expected to be a function that takes a body and returns another body.
    pub fn new(service: S) -> Self {
        Self { inner: service }
    }
}

impl<S> Service<Request<axum::body::Body>> for RequestStat<S>
where
    S: Service<Request<RequestBody>, Response = Response<BoxBody>>,
{
    type Response = Response<ResponseBody>;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<hyper::Body>) -> Self::Future {
        // 必须有 pool
        let pool = &req.extensions().get::<Arc<Pool<Postgres>>>().unwrap();
        // 必须解析出
        let Identity { bucket_name, .. } =
            req.extensions().get::<Arc<Identity>>().unwrap().as_ref();
        let bucket_name = bucket_name.clone();

        let pool1 = Arc::clone(pool);
        let pool2 = Arc::clone(pool);

        let object_id = ObjectId::new();
        let req = req.map(|x| RequestBody::new(x, object_id, pool1, bucket_name.clone()));
        ResponseFuture {
            inner: self.inner.call(req),
            id: object_id,
            pool: pool2,
            bucket_name,
        }
    }
}

pin_project! {
    /// Response future for [`MapResponseBody`].
    pub struct ResponseFuture<Fut> {
        #[pin]
        inner: Fut,
        id:ObjectId,
        pool:Arc<Pool<Postgres>>,
        bucket_name: String,
    }
}

impl<Fut, E> Future for ResponseFuture<Fut>
where
    Fut: Future<Output = Result<Response<BoxBody>, E>>,
{
    type Output = Result<Response<ResponseBody>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pool = Arc::clone(&self.pool);
        let bucket_name = self.bucket_name.clone();
        let id = self.id;
        let this = self.project();
        let res = ready!(this.inner.poll(cx)?);
        Poll::Ready(Ok(res.map(|x| ResponseBody::new(x, id, pool, bucket_name))))
    }
}
