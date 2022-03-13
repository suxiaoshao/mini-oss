use std::sync::Arc;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::body::BoxBody;
use bson::oid::ObjectId;
use bytes::Bytes;
use database::bucket::BucketModal;
use errors::TonicResult;
use futures::ready;

use database::request::RequestModal;
use database::{Decimal, Pool, Postgres};

// A wrapper for a `hyper::Body` that prints the size of data chunks
pub struct ResponseBody {
    inner: BoxBody,
    id: ObjectId,
    pool: Arc<Pool<Postgres>>,
    bucket_name: String,
}

impl ResponseBody {
    pub fn new(
        inner: BoxBody,
        id: ObjectId,
        pool: Arc<Pool<Postgres>>,
        bucket_name: String,
    ) -> Self {
        Self {
            inner,
            id,
            pool,
            bucket_name,
        }
    }
}

impl http_body::Body for ResponseBody {
    type Data = Bytes;
    type Error = axum::Error;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        let id = self.id;
        let bucket_name = self.bucket_name.clone();
        let pool = Arc::clone(&self.pool);
        if let Some(chunk) = ready!(Pin::new(&mut self.inner).poll_data(cx)?) {
            let len = chunk.as_ref().len();
            let size = Decimal::from(len);
            tokio::spawn(async move {
                // 失败为内部错误
                add_download_size(&id.to_string(), &bucket_name, &size, &pool)
                    .await
                    .unwrap();
            });
            Poll::Ready(Some(Ok(chunk)))
        } else {
            Poll::Ready(None)
        }
    }

    fn poll_trailers(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<hyper::HeaderMap>, Self::Error>> {
        Pin::new(&mut self.inner).poll_trailers(cx)
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> http_body::SizeHint {
        self.inner.size_hint()
    }
}

async fn add_download_size(
    object_id: &str,
    bucket_name: &str,
    size: &Decimal,
    pool: &Pool<Postgres>,
) -> TonicResult<()> {
    let BucketModal { username, .. } = BucketModal::find_one(bucket_name, pool).await?;
    RequestModal::add_download_size(object_id, bucket_name, size, &username, pool).await?;
    Ok(())
}
