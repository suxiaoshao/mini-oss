use std::sync::Arc;

use axum::headers::{HeaderMapExt, Host};
use axum::http::header::AUTHORIZATION;
use axum::http::Request;
use axum::response::Response;
use axum_extra::middleware::Next;
use nom::bytes::complete::{tag, take_until};
use nom::sequence::tuple;
use nom::IResult;

use crate::errors::{OpenError, OpenResult};

pub(crate) struct Identity {
    pub(crate) bucket_name: String,
    pub(crate) auth: Option<String>,
}

pub(crate) async fn identity<B>(mut req: Request<B>, next: Next<B>) -> OpenResult<Response> {
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
    req.extensions_mut()
        .insert(Arc::new(Identity { auth, bucket_name }));
    Ok(next.run(req).await)
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
