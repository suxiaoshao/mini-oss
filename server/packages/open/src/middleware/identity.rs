use std::sync::Arc;

use axum::headers::{HeaderMapExt, Host};
use axum::http::header::AUTHORIZATION;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use nom::bytes::complete::{tag, take_until};
use nom::sequence::tuple;
use nom::IResult;

use crate::errors::{OpenError, OpenResult};

#[derive(Debug)]
pub(crate) struct Identity {
    pub(crate) bucket_name: String,
    pub(crate) auth: Option<String>,
}

pub(crate) async fn identity<B>(mut req: Request<B>, next: Next<B>) -> OpenResult<Response> {
    let identity = Identity::try_from(&req)?;

    req.extensions_mut().insert(Arc::new(identity));
    Ok(next.run(req).await)
}

impl<B> TryFrom<&Request<B>> for Identity {
    type Error = OpenError;

    fn try_from(req: &Request<B>) -> Result<Self, Self::Error> {
        let host = req
            .headers()
            .typed_get::<Host>()
            .as_ref()
            .map(|h| h.hostname().to_string())
            .ok_or(OpenError::NoneBucketName)?;

        let (_, bucket_name) = parse_bucket_name(&host).map_err(|_| OpenError::NoneBucketName)?;

        let auth = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .map(|x| x.to_string());
        Ok(Identity { auth, bucket_name })
    }
}

/// 从 hostname 中获取 bucket_name
fn parse_bucket_name(hostname: &str) -> IResult<&str, String> {
    let (input, (bucket_name, ..)) = tuple((take_until("."), tag(".open.mini-oss.")))(hostname)?;
    Ok((input, bucket_name.to_string()))
}
#[cfg(test)]
mod test {
    use crate::middleware::identity::parse_bucket_name;

    #[test]
    fn test() {
        let input = "sdasf";
        assert!(parse_bucket_name(input).is_err());
        let input = ".sdasf.open.mini-oss.top";
        assert!(parse_bucket_name(input).is_err());
        let input = "as-sdasf.open.mini-oss.top";
        assert!(parse_bucket_name(input).is_ok());
        let input = "sdasf.open.mini-oss.sushao.me";
        assert!(parse_bucket_name(input).is_ok());
        let input = "sdasf.open.mini-oss.sushao.top";
        assert!(parse_bucket_name(input).is_ok());
    }
}
