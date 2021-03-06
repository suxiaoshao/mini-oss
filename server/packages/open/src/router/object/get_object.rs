use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;

use axum::body::BoxBody;
use axum::extract::Extension;
use axum::headers::{ETag, HeaderMapExt, IfModifiedSince, IfNoneMatch, LastModified};
use axum::http::header::{HeaderMap, HeaderName, HeaderValue};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use futures::future;
use time::OffsetDateTime;
use tonic::Request;

use proto::core::{GetObjectContentReply, GetObjectRequest, Header, ObjectInfo};
use proto::middleware::client::object_client;

use crate::errors::{OpenError, OpenResult};
use crate::extract::path_with_name::PathWithName;
use crate::middleware::identity::Identity;

/// 获取文件
pub(crate) async fn get_object(
    Extension(identity): Extension<Arc<Identity>>,
    path: PathWithName,
    headers: HeaderMap,
) -> OpenResult<Response<BoxBody>> {
    let Identity { bucket_name, auth } = identity.deref();
    let PathWithName { path, filename } = path;
    let mut client = object_client(auth.clone()).await?;
    let request = GetObjectRequest {
        bucket_name: bucket_name.clone(),
        path,
        filename,
    };

    // etag 缓存处理
    if let Some(if_none_match) = headers.typed_get::<IfNoneMatch>() {
        let object_info = client
            .get_object(Request::new(request.clone()))
            .await?
            .into_inner();
        return if if_none_match.precondition_passes(&etag(&object_info.blake3)?) {
            let GetObjectContentReply { content } = client
                .get_object_content(Request::new(request.clone()))
                .await?
                .into_inner();
            res_content(object_info, content)
        } else {
            res_304(object_info)
        };
    };

    // last modified 缓存处理
    if let Some(if_modified_since) = headers.typed_get::<IfModifiedSince>() {
        let object_info = client
            .get_object(Request::new(request.clone()))
            .await?
            .into_inner();
        let system_time = system_time(object_info.update_time)?;
        return if if_modified_since.is_modified(system_time) {
            let GetObjectContentReply { content } = client
                .get_object_content(Request::new(request.clone()))
                .await?
                .into_inner();
            res_content(object_info, content)
        } else {
            res_304(object_info)
        };
    }

    // 未命中缓存
    let (content, object_info) = future::try_join(
        client
            .clone()
            .get_object_content(Request::new(request.clone())),
        client.get_object(Request::new(request)),
    )
    .await?;
    let GetObjectContentReply { content } = content.into_inner();
    res_content(object_info.into_inner(), content)
}

/// 返回缓存击中
fn res_304(object_info: ObjectInfo) -> OpenResult<Response<BoxBody>> {
    let headers = get_header_map(object_info)?;
    Ok((StatusCode::NOT_MODIFIED, headers).into_response())
}
/// 返回内容
fn res_content(object_info: ObjectInfo, content: Vec<u8>) -> OpenResult<Response<BoxBody>> {
    let headers = get_header_map(object_info)?;
    Ok((headers, content).into_response())
}

/// 获取头部
fn get_header_map(object_info: ObjectInfo) -> OpenResult<HeaderMap> {
    let ObjectInfo {
        update_time,
        blake3,
        headers,
        ..
    } = object_info;
    let mut header_map = HeaderMap::new();

    // etag
    let etag = etag(&blake3)?;
    header_map.typed_insert(etag);

    // last modified
    let update_time = system_time(update_time)?;
    let modified = LastModified::from(update_time);
    header_map.typed_insert(modified);

    // 用户自定义头部
    for Header { key, value } in headers {
        header_map.insert(HeaderName::from_str(&key)?, HeaderValue::from_str(&value)?);
    }
    Ok(header_map)
}

/// 获取系统时间
fn system_time(time: i64) -> OpenResult<SystemTime> {
    Ok(SystemTime::from(
        OffsetDateTime::from_unix_timestamp_nanos(time as i128 * 1000000)
            .map_err(|_| OpenError::HttpHeadersParse)?,
    ))
}

/// 获取 etag 的值
fn etag(blake3: &str) -> OpenResult<ETag> {
    format!("\"{}\"", blake3)
        .parse::<ETag>()
        .map_err(|_| OpenError::HttpHeadersParse)
}
