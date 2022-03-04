#[cfg(feature = "graphql")]
use async_graphql::{Enum, InputObject, SimpleObject};
#[cfg(feature = "validate")]
use validator::Validate;

#[cfg(feature = "validate")]
use crate::validation::{bucket::validate_bucket, folder::validate_folder, path::validate_path};

#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBucketRequest {
    /// 身份验证
    #[prost(string, tag = "1")]
    pub auth: ::prost::alloc::string::String,
    /// bucket 名
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBucketListReply {
    /// 数据
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<BucketInfo>,
    /// 总数
    #[prost(int64, tag = "2")]
    pub total: i64,
}
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBucketRequest {
    /// 名字
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 访问控制
    #[prost(enumeration = "BucketAccess", tag = "2")]
    pub access: i32,
    /// 用户凭证
    #[prost(string, tag = "3")]
    pub auth: ::prost::alloc::string::String,
}
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBucketRequest {
    /// 名字
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 用户凭证
    #[prost(string, tag = "2")]
    pub auth: ::prost::alloc::string::String,
}
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBucketsRequest {
    /// 用户名
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    /// 用户凭证
    #[prost(string, tag = "2")]
    pub auth: ::prost::alloc::string::String,
}
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[cfg_attr(feature = "validate", derive(Validate))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBucketRequest {
    /// 名字
    #[prost(string, tag = "1")]
    #[cfg_attr(feature = "validate", validate(custom = "validate_bucket"))]
    pub name: ::prost::alloc::string::String,
    /// 访问控制
    #[prost(enumeration = "BucketAccess", tag = "2")]
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
    #[prost(enumeration = "BucketAccess", tag = "4")]
    pub access: i32,
    /// 用户名
    #[prost(string, tag = "5")]
    pub username: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFolderSizeReply {
    /// 对象总数
    #[prost(int64, tag = "1")]
    pub size: i64,
}
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFolderRequest {
    /// 身份验证
    #[prost(string, optional, tag = "1")]
    pub auth: ::core::option::Option<::prost::alloc::string::String>,
    /// bucket 名
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// 路径
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountReply {
    /// 总数
    #[prost(int64, tag = "1")]
    pub total: i64,
}
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFolderListRequest {
    /// 获取多少数据
    #[prost(uint32, tag = "1")]
    pub limit: u32,
    /// 偏移量
    #[prost(uint32, tag = "2")]
    pub offset: u32,
    /// 身份验证
    #[prost(string, optional, tag = "3")]
    pub auth: ::core::option::Option<::prost::alloc::string::String>,
    /// 路径
    #[prost(string, tag = "4")]
    pub path: ::prost::alloc::string::String,
    /// bucket 名
    #[prost(string, tag = "5")]
    pub bucket_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFolderListReply {
    /// 数据
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<FolderInfo>,
    /// 总数
    #[prost(int64, tag = "2")]
    pub total: i64,
}
#[cfg_attr(feature = "validate", derive(Validate))]
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFolderRequest {
    /// 路径
    #[prost(string, tag = "1")]
    #[cfg_attr(feature = "validate", validate(custom = "validate_path"))]
    pub path: ::prost::alloc::string::String,
    /// bucket 名
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// 用户凭证
    #[prost(string, optional, tag = "3")]
    pub auth: ::core::option::Option<::prost::alloc::string::String>,
    /// 访问控制
    #[prost(enumeration = "FolderAccess", tag = "4")]
    pub access: i32,
}
#[cfg_attr(feature = "validate", derive(Validate))]
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFolderRequest {
    /// 名字
    #[prost(string, tag = "1")]
    #[cfg_attr(feature = "validate", validate(custom = "validate_path"))]
    pub path: ::prost::alloc::string::String,
    /// bucket 名
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// 用户凭证
    #[prost(string, optional, tag = "3")]
    pub auth: ::core::option::Option<::prost::alloc::string::String>,
}
#[cfg_attr(feature = "validate", derive(Validate))]
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFolderRequest {
    /// 路径
    #[prost(string, tag = "1")]
    #[cfg_attr(feature = "validate", validate(custom = "validate_folder"))]
    pub path: ::prost::alloc::string::String,
    /// bucket 名
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// 路径
    #[prost(string, tag = "3")]
    pub father_path: ::prost::alloc::string::String,
    /// 用户凭证
    #[prost(string, optional, tag = "4")]
    pub auth: ::core::option::Option<::prost::alloc::string::String>,
    /// 访问控制
    #[prost(enumeration = "FolderAccess", tag = "5")]
    pub access: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FolderInfo {
    /// 路径
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// 创建时间
    #[prost(int64, tag = "2")]
    pub create_time: i64,
    /// 创建时间
    #[prost(int64, tag = "3")]
    pub update_time: i64,
    /// bucket 名
    #[prost(string, tag = "4")]
    pub bucket_name: ::prost::alloc::string::String,
    /// 访问控制
    #[prost(enumeration = "FolderAccess", tag = "5")]
    pub access: i32,
    /// 路径
    #[prost(string, tag = "6")]
    pub father_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SizeReply {
    /// 对象大小
    #[prost(string, tag = "1")]
    pub size: ::prost::alloc::string::String,
}
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectRequest {
    /// 身份验证
    #[prost(string, optional, tag = "1")]
    pub auth: ::core::option::Option<::prost::alloc::string::String>,
    /// bucket 名
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// 路径
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
    /// 文件名
    #[prost(string, tag = "4")]
    pub filename: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectContentReply {
    #[prost(bytes = "vec", tag = "6")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectListReply {
    /// 数据
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<ObjectInfo>,
    /// 总数
    #[prost(int64, tag = "2")]
    pub total: i64,
}
#[cfg_attr(feature = "validate", derive(Validate))]
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectRequest {
    /// 路径
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// bucket 名
    #[prost(string, tag = "2")]
    pub bucket_name: ::prost::alloc::string::String,
    /// 访问控制
    #[prost(enumeration = "ObjectAccess", tag = "3")]
    pub access: i32,
    /// 访问控制
    #[prost(string, optional, tag = "4")]
    pub auth: ::core::option::Option<::prost::alloc::string::String>,
    /// 文件名
    #[prost(string, tag = "5")]
    #[cfg_attr(feature = "validate", validate(custom = "validate_folder"))]
    pub filename: ::prost::alloc::string::String,
    /// 内容
    #[prost(bytes = "vec", tag = "6")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectRequest {
    /// 路径
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// 文件名
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
    /// bucket 名
    #[prost(string, tag = "3")]
    pub bucket_name: ::prost::alloc::string::String,
    /// 访问控制
    #[prost(string, optional, tag = "4")]
    pub auth: ::core::option::Option<::prost::alloc::string::String>,
}
#[cfg_attr(feature = "validate", derive(Validate))]
#[cfg_attr(feature = "graphql", derive(InputObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectRequest {
    /// 路径
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// 旧文件名
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
    /// bucket 名
    #[prost(string, tag = "3")]
    pub bucket_name: ::prost::alloc::string::String,
    /// 访问控制
    #[prost(enumeration = "ObjectAccess", tag = "4")]
    pub access: i32,
    /// 新文件名
    #[prost(string, tag = "6")]
    #[cfg_attr(feature = "validate", validate(custom = "validate_folder"))]
    pub new_filename: ::prost::alloc::string::String,
    /// 访问控制
    #[prost(string, optional, tag = "7")]
    pub auth: ::core::option::Option<::prost::alloc::string::String>,
    /// 自定义 header
    #[prost(message, repeated, tag = "8")]
    pub headers: ::prost::alloc::vec::Vec<Header>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectInfo {
    /// 路径
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// 文件名
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
    /// bucket 名
    #[prost(string, tag = "3")]
    pub bucket_name: ::prost::alloc::string::String,
    /// 访问控制
    #[prost(enumeration = "ObjectAccess", tag = "4")]
    pub access: i32,
    /// 创建时间
    #[prost(int64, tag = "5")]
    pub create_time: i64,
    /// 创建时间
    #[prost(int64, tag = "6")]
    pub update_time: i64,
    /// 大小
    #[prost(int64, tag = "7")]
    pub size: i64,
    /// 摘要
    #[prost(string, tag = "8")]
    pub blake3: ::prost::alloc::string::String,
    /// 自定义 header
    #[prost(message, repeated, tag = "9")]
    pub headers: ::prost::alloc::vec::Vec<Header>,
}

#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize,))]
#[cfg_attr(feature = "graphql", derive(InputObject, SimpleObject))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTimeRequest {
    /// bucket 名
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// 权限
    #[prost(string, tag = "2")]
    pub auth: ::prost::alloc::string::String,
    /// 开始时间
    #[prost(int64, tag = "3")]
    pub start_time: i64,
    /// 结束时间
    #[prost(int64, tag = "4")]
    pub end_time: i64,
}
/// 访问权限类型
#[cfg_attr(feature = "graphql", derive(Enum))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BucketAccess {
    /// 开放
    Open = 0,
    /// 读开放
    ReadOpen = 1,
    /// 不开放
    Private = 2,
}
/// 访问权限类型
#[cfg_attr(feature = "graphql", derive(Enum))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FolderAccess {
    /// 继承
    InheritanceFolder = 0,
    /// 读开放
    ReadOpenFolder = 1,
    /// 不开放
    PrivateFolder = 2,
    /// 开放
    OpenFolder = 3,
}
#[cfg_attr(feature = "graphql", derive(Enum))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ObjectAccess {
    /// 继承
    InheritanceObject = 0,
    /// 读开放
    ReadOpenObject = 1,
    /// 不开放
    PrivateObject = 2,
}
#[cfg(feature = "core_client")]
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
        ) -> Result<tonic::Response<super::super::user::Empty>, tonic::Status> {
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
        ) -> Result<tonic::Response<super::super::user::Empty>, tonic::Status> {
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
        #[doc = " 获取用户 bucket 列表"]
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
        #[doc = " 获取某个 bucket 信息"]
        pub async fn get_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBucketRequest>,
        ) -> Result<tonic::Response<super::BucketInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Bucket/GetBucket");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[cfg(feature = "core_client")]
#[doc = r" Generated client implementations."]
pub mod folder_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

    use tonic::codegen::*;

    #[derive(Debug, Clone)]
    pub struct FolderClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FolderClient<tonic::transport::Channel> {
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
    impl<T> FolderClient<T>
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
        ) -> FolderClient<InterceptedService<T, F>>
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
            FolderClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " 创建文件夹"]
        pub async fn create_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFolderRequest>,
        ) -> Result<tonic::Response<super::FolderInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Folder/CreateFolder");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 删除文件夹"]
        pub async fn delete_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFolderRequest>,
        ) -> Result<tonic::Response<super::super::user::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Folder/DeleteFolder");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 修改文件夹"]
        pub async fn update_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFolderRequest>,
        ) -> Result<tonic::Response<super::FolderInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Folder/UpdateFolder");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取 path 列表"]
        pub async fn get_folder_list(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderListRequest>,
        ) -> Result<tonic::Response<super::GetFolderListReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Folder/GetFolderList");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取 path 总数"]
        pub async fn get_folder_count(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::CountReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Folder/GetFolderCount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取 path 信息"]
        pub async fn get_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::FolderInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Folder/GetFolder");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取 path 下所有 path 总数"]
        pub async fn get_total_by_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::CountReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Folder/GetTotalByFolder");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[cfg(feature = "core_client")]
#[doc = r" Generated client implementations."]
pub mod object_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ObjectClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectClient<tonic::transport::Channel> {
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
    impl<T> ObjectClient<T>
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
        ) -> ObjectClient<InterceptedService<T, F>>
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
            ObjectClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " 创建文件夹"]
        pub async fn create_object(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateObjectRequest>,
        ) -> Result<tonic::Response<super::super::user::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Object/CreateObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 删除文件夹"]
        pub async fn delete_object(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectRequest>,
        ) -> Result<tonic::Response<super::super::user::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Object/DeleteObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 修改文件夹"]
        pub async fn update_object(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateObjectRequest>,
        ) -> Result<tonic::Response<super::ObjectInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Object/UpdateObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取path列表"]
        pub async fn get_object_list(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderListRequest>,
        ) -> Result<tonic::Response<super::GetObjectListReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Object/GetObjectList");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取 object 总数"]
        pub async fn get_object_count(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::CountReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Object/GetObjectCount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取 object 信息"]
        pub async fn get_object(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectRequest>,
        ) -> Result<tonic::Response<super::ObjectInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Object/GetObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取文件内容"]
        pub async fn get_object_content(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectRequest>,
        ) -> Result<tonic::Response<super::GetObjectContentReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Object/GetObjectContent");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取 path 下所有 object 总数"]
        pub async fn get_total_by_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::CountReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Object/GetTotalByFolder");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取 path 下所有 object 大小"]
        pub async fn get_size_by_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::SizeReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Object/GetSizeByFolder");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取 bucket 下所有 object 总数"]
        pub async fn get_total_by_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBucketRequest>,
        ) -> Result<tonic::Response<super::CountReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Object/GetTotalByBucket");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取 bucket 下所有 object 大小"]
        pub async fn get_size_by_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBucketRequest>,
        ) -> Result<tonic::Response<super::SizeReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Object/GetSizeByBucket");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[cfg(feature = "core_client")]
#[doc = r" Generated client implementations."]
pub mod request_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct RequestClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RequestClient<tonic::transport::Channel> {
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
    impl<T> RequestClient<T>
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
        ) -> RequestClient<InterceptedService<T, F>>
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
            RequestClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " 获取上传数据大小"]
        pub async fn get_upload_size_by_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTimeRequest>,
        ) -> Result<tonic::Response<super::SizeReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/core.Request/GetUploadSizeByBucket");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取下载数据大小"]
        pub async fn get_download_size_by_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTimeRequest>,
        ) -> Result<tonic::Response<super::SizeReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/core.Request/GetDownloadSizeByBucket");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[cfg(feature = "core_server")]
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
        ) -> Result<tonic::Response<super::super::user::Empty>, tonic::Status>;
        #[doc = " 删除 某一个用户下的Bucket"]
        async fn delete_buckets(
            &self,
            request: tonic::Request<super::DeleteBucketsRequest>,
        ) -> Result<tonic::Response<super::super::user::Empty>, tonic::Status>;
        #[doc = " 修改 Bucket"]
        async fn update_bucket(
            &self,
            request: tonic::Request<super::UpdateBucketRequest>,
        ) -> Result<tonic::Response<super::BucketInfo>, tonic::Status>;
        #[doc = " 获取用户 bucket 列表"]
        async fn get_bucket_list(
            &self,
            request: tonic::Request<super::super::user::GetListRequest>,
        ) -> Result<tonic::Response<super::GetBucketListReply>, tonic::Status>;
        #[doc = " 获取某个 bucket 信息"]
        async fn get_bucket(
            &self,
            request: tonic::Request<super::GetBucketRequest>,
        ) -> Result<tonic::Response<super::BucketInfo>, tonic::Status>;
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
                        type Response = super::super::user::Empty;
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
                        type Response = super::super::user::Empty;
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
                "/core.Bucket/GetBucket" => {
                    #[allow(non_camel_case_types)]
                    struct GetBucketSvc<T: Bucket>(pub Arc<T>);
                    impl<T: Bucket> tonic::server::UnaryService<super::GetBucketRequest> for GetBucketSvc<T> {
                        type Response = super::BucketInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBucketRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_bucket(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBucketSvc(inner);
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
#[cfg(feature = "core_server")]
#[doc = r" Generated server implementations."]
pub mod folder_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

    use tonic::codegen::*;

    #[doc = "Generated trait containing gRPC methods that should be implemented for use with FolderServer."]
    #[async_trait]
    pub trait Folder: Send + Sync + 'static {
        #[doc = " 创建文件夹"]
        async fn create_folder(
            &self,
            request: tonic::Request<super::CreateFolderRequest>,
        ) -> Result<tonic::Response<super::FolderInfo>, tonic::Status>;
        #[doc = " 删除文件夹"]
        async fn delete_folder(
            &self,
            request: tonic::Request<super::DeleteFolderRequest>,
        ) -> Result<tonic::Response<super::super::user::Empty>, tonic::Status>;
        #[doc = " 修改文件夹"]
        async fn update_folder(
            &self,
            request: tonic::Request<super::UpdateFolderRequest>,
        ) -> Result<tonic::Response<super::FolderInfo>, tonic::Status>;
        #[doc = " 获取 path 列表"]
        async fn get_folder_list(
            &self,
            request: tonic::Request<super::GetFolderListRequest>,
        ) -> Result<tonic::Response<super::GetFolderListReply>, tonic::Status>;
        #[doc = " 获取 path 总数"]
        async fn get_folder_count(
            &self,
            request: tonic::Request<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::CountReply>, tonic::Status>;
        #[doc = " 获取 path 信息"]
        async fn get_folder(
            &self,
            request: tonic::Request<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::FolderInfo>, tonic::Status>;
        #[doc = " 获取 path 下所有 path 总数"]
        async fn get_total_by_folder(
            &self,
            request: tonic::Request<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::CountReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct FolderServer<T: Folder> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Folder> FolderServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FolderServer<T>
    where
        T: Folder,
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
                "/core.Folder/CreateFolder" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFolderSvc<T: Folder>(pub Arc<T>);
                    impl<T: Folder> tonic::server::UnaryService<super::CreateFolderRequest> for CreateFolderSvc<T> {
                        type Response = super::FolderInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFolderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_folder(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateFolderSvc(inner);
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
                "/core.Folder/DeleteFolder" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteFolderSvc<T: Folder>(pub Arc<T>);
                    impl<T: Folder> tonic::server::UnaryService<super::DeleteFolderRequest> for DeleteFolderSvc<T> {
                        type Response = super::super::user::Empty;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteFolderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_folder(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteFolderSvc(inner);
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
                "/core.Folder/UpdateFolder" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFolderSvc<T: Folder>(pub Arc<T>);
                    impl<T: Folder> tonic::server::UnaryService<super::UpdateFolderRequest> for UpdateFolderSvc<T> {
                        type Response = super::FolderInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateFolderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_folder(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateFolderSvc(inner);
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
                "/core.Folder/GetFolderList" => {
                    #[allow(non_camel_case_types)]
                    struct GetFolderListSvc<T: Folder>(pub Arc<T>);
                    impl<T: Folder> tonic::server::UnaryService<super::GetFolderListRequest> for GetFolderListSvc<T> {
                        type Response = super::GetFolderListReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFolderListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_folder_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFolderListSvc(inner);
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
                "/core.Folder/GetFolderCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetFolderCountSvc<T: Folder>(pub Arc<T>);
                    impl<T: Folder> tonic::server::UnaryService<super::GetFolderRequest> for GetFolderCountSvc<T> {
                        type Response = super::CountReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFolderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_folder_count(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFolderCountSvc(inner);
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
                "/core.Folder/GetFolder" => {
                    #[allow(non_camel_case_types)]
                    struct GetFolderSvc<T: Folder>(pub Arc<T>);
                    impl<T: Folder> tonic::server::UnaryService<super::GetFolderRequest> for GetFolderSvc<T> {
                        type Response = super::FolderInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFolderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_folder(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFolderSvc(inner);
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
                "/core.Folder/GetTotalByFolder" => {
                    #[allow(non_camel_case_types)]
                    struct GetTotalByFolderSvc<T: Folder>(pub Arc<T>);
                    impl<T: Folder> tonic::server::UnaryService<super::GetFolderRequest> for GetTotalByFolderSvc<T> {
                        type Response = super::CountReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFolderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_total_by_folder(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTotalByFolderSvc(inner);
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
    impl<T: Folder> Clone for FolderServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Folder> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Folder> tonic::transport::NamedService for FolderServer<T> {
        const NAME: &'static str = "core.Folder";
    }
}
#[cfg(feature = "core_server")]
#[doc = r" Generated server implementations."]
pub mod object_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ObjectServer."]
    #[async_trait]
    pub trait Object: Send + Sync + 'static {
        #[doc = " 创建文件夹"]
        async fn create_object(
            &self,
            request: tonic::Request<super::CreateObjectRequest>,
        ) -> Result<tonic::Response<super::super::user::Empty>, tonic::Status>;
        #[doc = " 删除文件夹"]
        async fn delete_object(
            &self,
            request: tonic::Request<super::DeleteObjectRequest>,
        ) -> Result<tonic::Response<super::super::user::Empty>, tonic::Status>;
        #[doc = " 修改文件夹"]
        async fn update_object(
            &self,
            request: tonic::Request<super::UpdateObjectRequest>,
        ) -> Result<tonic::Response<super::ObjectInfo>, tonic::Status>;
        #[doc = " 获取path列表"]
        async fn get_object_list(
            &self,
            request: tonic::Request<super::GetFolderListRequest>,
        ) -> Result<tonic::Response<super::GetObjectListReply>, tonic::Status>;
        #[doc = " 获取 object 总数"]
        async fn get_object_count(
            &self,
            request: tonic::Request<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::CountReply>, tonic::Status>;
        #[doc = " 获取 object 信息"]
        async fn get_object(
            &self,
            request: tonic::Request<super::GetObjectRequest>,
        ) -> Result<tonic::Response<super::ObjectInfo>, tonic::Status>;
        #[doc = " 获取文件内容"]
        async fn get_object_content(
            &self,
            request: tonic::Request<super::GetObjectRequest>,
        ) -> Result<tonic::Response<super::GetObjectContentReply>, tonic::Status>;
        #[doc = " 获取 path 下所有 object 总数"]
        async fn get_total_by_folder(
            &self,
            request: tonic::Request<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::CountReply>, tonic::Status>;
        #[doc = " 获取 path 下所有 object 大小"]
        async fn get_size_by_folder(
            &self,
            request: tonic::Request<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::SizeReply>, tonic::Status>;
        #[doc = " 获取 bucket 下所有 object 总数"]
        async fn get_total_by_bucket(
            &self,
            request: tonic::Request<super::GetBucketRequest>,
        ) -> Result<tonic::Response<super::CountReply>, tonic::Status>;
        #[doc = " 获取 bucket 下所有 object 大小"]
        async fn get_size_by_bucket(
            &self,
            request: tonic::Request<super::GetBucketRequest>,
        ) -> Result<tonic::Response<super::SizeReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ObjectServer<T: Object> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Object> ObjectServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ObjectServer<T>
    where
        T: Object,
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
                "/core.Object/CreateObject" => {
                    #[allow(non_camel_case_types)]
                    struct CreateObjectSvc<T: Object>(pub Arc<T>);
                    impl<T: Object> tonic::server::UnaryService<super::CreateObjectRequest> for CreateObjectSvc<T> {
                        type Response = super::super::user::Empty;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateObjectSvc(inner);
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
                "/core.Object/DeleteObject" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectSvc<T: Object>(pub Arc<T>);
                    impl<T: Object> tonic::server::UnaryService<super::DeleteObjectRequest> for DeleteObjectSvc<T> {
                        type Response = super::super::user::Empty;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteObjectSvc(inner);
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
                "/core.Object/UpdateObject" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateObjectSvc<T: Object>(pub Arc<T>);
                    impl<T: Object> tonic::server::UnaryService<super::UpdateObjectRequest> for UpdateObjectSvc<T> {
                        type Response = super::ObjectInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateObjectSvc(inner);
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
                "/core.Object/GetObjectList" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectListSvc<T: Object>(pub Arc<T>);
                    impl<T: Object> tonic::server::UnaryService<super::GetFolderListRequest> for GetObjectListSvc<T> {
                        type Response = super::GetObjectListReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFolderListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_object_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectListSvc(inner);
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
                "/core.Object/GetObjectCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectCountSvc<T: Object>(pub Arc<T>);
                    impl<T: Object> tonic::server::UnaryService<super::GetFolderRequest> for GetObjectCountSvc<T> {
                        type Response = super::CountReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFolderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_object_count(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectCountSvc(inner);
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
                "/core.Object/GetObject" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectSvc<T: Object>(pub Arc<T>);
                    impl<T: Object> tonic::server::UnaryService<super::GetObjectRequest> for GetObjectSvc<T> {
                        type Response = super::ObjectInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectSvc(inner);
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
                "/core.Object/GetObjectContent" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectContentSvc<T: Object>(pub Arc<T>);
                    impl<T: Object> tonic::server::UnaryService<super::GetObjectRequest> for GetObjectContentSvc<T> {
                        type Response = super::GetObjectContentReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_object_content(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectContentSvc(inner);
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
                "/core.Object/GetTotalByFolder" => {
                    #[allow(non_camel_case_types)]
                    struct GetTotalByFolderSvc<T: Object>(pub Arc<T>);
                    impl<T: Object> tonic::server::UnaryService<super::GetFolderRequest> for GetTotalByFolderSvc<T> {
                        type Response = super::CountReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFolderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_total_by_folder(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTotalByFolderSvc(inner);
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
                "/core.Object/GetSizeByFolder" => {
                    #[allow(non_camel_case_types)]
                    struct GetSizeByFolderSvc<T: Object>(pub Arc<T>);
                    impl<T: Object> tonic::server::UnaryService<super::GetFolderRequest> for GetSizeByFolderSvc<T> {
                        type Response = super::SizeReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFolderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_size_by_folder(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSizeByFolderSvc(inner);
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
                "/core.Object/GetTotalByBucket" => {
                    #[allow(non_camel_case_types)]
                    struct GetTotalByBucketSvc<T: Object>(pub Arc<T>);
                    impl<T: Object> tonic::server::UnaryService<super::GetBucketRequest> for GetTotalByBucketSvc<T> {
                        type Response = super::CountReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBucketRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_total_by_bucket(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTotalByBucketSvc(inner);
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
                "/core.Object/GetSizeByBucket" => {
                    #[allow(non_camel_case_types)]
                    struct GetSizeByBucketSvc<T: Object>(pub Arc<T>);
                    impl<T: Object> tonic::server::UnaryService<super::GetBucketRequest> for GetSizeByBucketSvc<T> {
                        type Response = super::SizeReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBucketRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_size_by_bucket(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSizeByBucketSvc(inner);
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
    impl<T: Object> Clone for ObjectServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Object> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Object> tonic::transport::NamedService for ObjectServer<T> {
        const NAME: &'static str = "core.Object";
    }
}
#[cfg(feature = "core_server")]
#[doc = r" Generated server implementations."]
pub mod request_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RequestServer."]
    #[async_trait]
    pub trait Request: Send + Sync + 'static {
        #[doc = " 获取上传数据大小"]
        async fn get_upload_size_by_bucket(
            &self,
            request: tonic::Request<super::GetTimeRequest>,
        ) -> Result<tonic::Response<super::SizeReply>, tonic::Status>;
        #[doc = " 获取下载数据大小"]
        async fn get_download_size_by_bucket(
            &self,
            request: tonic::Request<super::GetTimeRequest>,
        ) -> Result<tonic::Response<super::SizeReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RequestServer<T: Request> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Request> RequestServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RequestServer<T>
    where
        T: Request,
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
                "/core.Request/GetUploadSizeByBucket" => {
                    #[allow(non_camel_case_types)]
                    struct GetUploadSizeByBucketSvc<T: Request>(pub Arc<T>);
                    impl<T: Request> tonic::server::UnaryService<super::GetTimeRequest>
                        for GetUploadSizeByBucketSvc<T>
                    {
                        type Response = super::SizeReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTimeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_upload_size_by_bucket(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUploadSizeByBucketSvc(inner);
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
                "/core.Request/GetDownloadSizeByBucket" => {
                    #[allow(non_camel_case_types)]
                    struct GetDownloadSizeByBucketSvc<T: Request>(pub Arc<T>);
                    impl<T: Request> tonic::server::UnaryService<super::GetTimeRequest>
                        for GetDownloadSizeByBucketSvc<T>
                    {
                        type Response = super::SizeReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTimeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_download_size_by_bucket(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDownloadSizeByBucketSvc(inner);
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
    impl<T: Request> Clone for RequestServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Request> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Request> tonic::transport::NamedService for RequestServer<T> {
        const NAME: &'static str = "core.Request";
    }
}
