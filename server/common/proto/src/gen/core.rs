use crate::validation::name::validate_name;
use async_graphql::InputObject;
use serde::Deserialize;
use validator::Validate;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBucketListReply {
    /// 数据
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<BucketInfo>,
    /// 总数
    #[prost(int64, tag = "2")]
    pub total: i64,
}
#[derive(InputObject, Clone, PartialEq, ::prost::Message)]
pub struct UpdateBucketRequest {
    /// 名字
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 访问控制
    #[prost(enumeration = "Access", tag = "2")]
    pub access: i32,
    /// 用户凭证
    #[prost(string, tag = "3")]
    pub auth: ::prost::alloc::string::String,
}
#[derive(InputObject, Clone, PartialEq, ::prost::Message)]
pub struct DeleteBucketRequest {
    /// 名字
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 用户凭证
    #[prost(string, tag = "2")]
    pub auth: ::prost::alloc::string::String,
}
#[derive(InputObject, Clone, PartialEq, ::prost::Message)]
pub struct DeleteBucketsRequest {
    /// 用户名
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    /// 用户凭证
    #[prost(string, tag = "2")]
    pub auth: ::prost::alloc::string::String,
}
#[derive(InputObject, Clone, PartialEq, ::prost::Message, Validate, Deserialize)]
pub struct CreateBucketRequest {
    /// 名字
    #[prost(string, tag = "1")]
    #[validate(custom = "validate_name")]
    pub name: ::prost::alloc::string::String,
    /// 访问控制
    #[prost(enumeration = "Access", tag = "2")]
    pub access: i32,
    /// 用户凭证
    #[prost(string, tag = "3")]
    pub auth: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketInfo {
    /// 名字
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 创建时间
    #[prost(int64, tag = "2")]
    pub create_time: i64,
    /// 创建时间
    #[prost(int64, tag = "3")]
    pub update_time: i64,
    /// 访问控制
    #[prost(enumeration = "Access", tag = "4")]
    pub access: i32,
    /// 用户名
    #[prost(string, tag = "5")]
    pub username: ::prost::alloc::string::String,
}
/// 访问权限类型
#[derive(
    async_graphql::Enum,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum Access {
    /// 开放
    Open = 0,
    /// 读开放
    ReadOpen = 1,
    /// 不开放
    Private = 2,
}
#[doc = r" Generated client implementations."]
pub mod bucket_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct BucketClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BucketClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BucketClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BucketClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            BucketClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " 创建 Bucket"]
        pub async fn create_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBucketRequest>,
        ) -> Result<tonic::Response<super::BucketInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Bucket/CreateBucket");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 删除 Bucket"]
        pub async fn delete_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBucketRequest>,
        ) -> Result<tonic::Response<super::super::auth::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Bucket/DeleteBucket");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 删除 某一个用户下的Bucket"]
        pub async fn delete_buckets(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBucketsRequest>,
        ) -> Result<tonic::Response<super::super::auth::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Bucket/DeleteBuckets");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 修改 Bucket"]
        pub async fn update_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBucketRequest>,
        ) -> Result<tonic::Response<super::BucketInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Bucket/UpdateBucket");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取用户 bucker 列表"]
        pub async fn get_bucket_list(
            &mut self,
            request: impl tonic::IntoRequest<super::super::user::GetListRequest>,
        ) -> Result<tonic::Response<super::GetBucketListReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Bucket/GetBucketList");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod bucket_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BucketServer."]
    #[async_trait]
    pub trait Bucket: Send + Sync + 'static {
        #[doc = " 创建 Bucket"]
        async fn create_bucket(
            &self,
            request: tonic::Request<super::CreateBucketRequest>,
        ) -> Result<tonic::Response<super::BucketInfo>, tonic::Status>;
        #[doc = " 删除 Bucket"]
        async fn delete_bucket(
            &self,
            request: tonic::Request<super::DeleteBucketRequest>,
        ) -> Result<tonic::Response<super::super::auth::Empty>, tonic::Status>;
        #[doc = " 删除 某一个用户下的Bucket"]
        async fn delete_buckets(
            &self,
            request: tonic::Request<super::DeleteBucketsRequest>,
        ) -> Result<tonic::Response<super::super::auth::Empty>, tonic::Status>;
        #[doc = " 修改 Bucket"]
        async fn update_bucket(
            &self,
            request: tonic::Request<super::UpdateBucketRequest>,
        ) -> Result<tonic::Response<super::BucketInfo>, tonic::Status>;
        #[doc = " 获取用户 bucker 列表"]
        async fn get_bucket_list(
            &self,
            request: tonic::Request<super::super::user::GetListRequest>,
        ) -> Result<tonic::Response<super::GetBucketListReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BucketServer<T: Bucket> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Bucket> BucketServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BucketServer<T>
    where
        T: Bucket,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/core.Bucket/CreateBucket" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBucketSvc<T: Bucket>(pub Arc<T>);
                    impl<T: Bucket> tonic::server::UnaryService<super::CreateBucketRequest> for CreateBucketSvc<T> {
                        type Response = super::BucketInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBucketRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_bucket(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateBucketSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/core.Bucket/DeleteBucket" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBucketSvc<T: Bucket>(pub Arc<T>);
                    impl<T: Bucket> tonic::server::UnaryService<super::DeleteBucketRequest> for DeleteBucketSvc<T> {
                        type Response = super::super::auth::Empty;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBucketRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_bucket(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteBucketSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/core.Bucket/DeleteBuckets" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBucketsSvc<T: Bucket>(pub Arc<T>);
                    impl<T: Bucket> tonic::server::UnaryService<super::DeleteBucketsRequest> for DeleteBucketsSvc<T> {
                        type Response = super::super::auth::Empty;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBucketsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_buckets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteBucketsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/core.Bucket/UpdateBucket" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateBucketSvc<T: Bucket>(pub Arc<T>);
                    impl<T: Bucket> tonic::server::UnaryService<super::UpdateBucketRequest> for UpdateBucketSvc<T> {
                        type Response = super::BucketInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateBucketRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_bucket(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateBucketSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/core.Bucket/GetBucketList" => {
                    #[allow(non_camel_case_types)]
                    struct GetBucketListSvc<T: Bucket>(pub Arc<T>);
                    impl<T: Bucket> tonic::server::UnaryService<super::super::user::GetListRequest>
                        for GetBucketListSvc<T>
                    {
                        type Response = super::GetBucketListReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::user::GetListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_bucket_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBucketListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Bucket> Clone for BucketServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Bucket> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Bucket> tonic::transport::NamedService for BucketServer<T> {
        const NAME: &'static str = "core.Bucket";
    }
}
