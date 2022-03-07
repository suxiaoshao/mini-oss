use async_graphql::Object;

use proto::core::object_client::ObjectClient;
use proto::core::{
    CountReply, DeleteObjectRequest, GetBucketRequest, GetFolderListReply, GetFolderListRequest,
    GetFolderRequest, GetObjectRequest,
};
use proto::user::login_client::LoginClient;
use proto::user::{LoginReply, LoginRequest};
use proto::{
    core::{
        bucket_client::BucketClient, folder_client::FolderClient, DeleteBucketRequest,
        DeleteFolderRequest,
    },
    user::{
        self_manage_client::SelfManageClient, user_manage_client::UserManageClient,
        CreateUserRequest, DeleteUserRequest, GetListRequest, GetUserInfoRequest, GetUserRequest,
        UpdatePasswordRequest, UpdateUserInfoRequest, UpdateUserRequest,
    },
    Request,
};

use crate::errors::GraphqlResult;
use crate::schema::folder::folder_list::FolderList;
use crate::schema::object::object_info::ObjectInfo;
use crate::schema::object::request::UpdateObjectRequest;

use super::{
    bucket::{
        bucket_info::{BucketInfo, BucketList},
        request::{CreateBucketRequest, UpdateBucketRequest},
    },
    folder::{
        folder_info::FolderInfo,
        request::{CreateFolderRequest, UpdateFolderRequest},
    },
    user_info::{UserInfo, UserList},
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 管理员登陆
    async fn manager_login(&self, data: LoginRequest) -> GraphqlResult<String> {
        let mut client = LoginClient::connect("http://user:80").await?;
        let res = client.manager_login(data).await?;
        Ok(res.get_ref().auth.to_string())
    }
    /// 用户登陆
    async fn user_login(&self, data: LoginRequest) -> GraphqlResult<String> {
        let mut client = LoginClient::connect("http://user:80").await?;
        let res = client.user_login(data).await?;
        Ok(res.get_ref().auth.to_string())
    }
    /// 用户列表
    async fn user_list(&self, data: GetListRequest) -> GraphqlResult<UserList> {
        let mut client = UserManageClient::connect("http://user:80").await?;
        let reply = client.get_user_list(data).await?;
        Ok(UserList::from(reply.into_inner()))
    }
    /// 用户信息
    async fn user_info(&self, data: GetUserRequest) -> GraphqlResult<UserInfo> {
        let mut client = UserManageClient::connect("http://user:80").await?;
        let reply = client.get_user(data).await?;
        Ok(UserInfo::from(reply.into_inner()))
    }
    /// 获取自身用户信息
    async fn self_user_info(&self, data: GetUserInfoRequest) -> GraphqlResult<UserInfo> {
        let mut client = SelfManageClient::connect("http://user:80").await?;
        let reply = client.get_user_info(data).await?;
        Ok(UserInfo::from(reply.into_inner()))
    }
    /// 用户存储桶列表
    async fn bucket_list(&self, data: GetListRequest) -> GraphqlResult<BucketList> {
        let mut client = BucketClient::connect("http://core:80").await?;
        let res = client.get_bucket_list(data).await?;
        Ok(BucketList::from(res.into_inner()))
    }
    /// 文件夹列表
    async fn folder_list(&self, data: GetFolderListRequest) -> GraphqlResult<FolderList> {
        let GetFolderListRequest {
            limit,
            offset,
            auth,
            path,
            bucket_name,
        } = &data;
        let op_auth = auth.clone();
        // 需要的总数
        let require_count = limit + offset;

        // 获取文件夹总数
        let mut folder_client = FolderClient::connect("http://core:80").await?;
        let request = Request::new(GetFolderRequest {
            auth: auth.clone(),
            path: path.clone(),
            bucket_name: bucket_name.clone(),
        });
        let CountReply {
            total: folder_count,
        } = folder_client.get_folder_count(request).await?.into_inner();

        // 获取对象总数
        let mut object_client = ObjectClient::connect("http://core:80").await?;
        let request = GetFolderRequest {
            auth: auth.clone(),
            path: path.clone(),
            bucket_name: bucket_name.clone(),
        };
        let CountReply {
            total: object_count,
        } = object_client.get_object_count(request).await?.into_inner();

        // 数据库内的总数
        let total = object_count + folder_count;

        // 如果文件夹总数大于需要的总数
        if folder_count > require_count as i64 {
            let request = Request::new(data);
            let GetFolderListReply { data, .. } =
                folder_client.get_folder_list(request).await?.into_inner();
            Ok(FolderList {
                total,
                data: data
                    .into_iter()
                    .map(|x| FolderInfo::from((x, op_auth.clone())).into())
                    .collect(),
            })
            // 如果需要两种
        } else if folder_count > *offset as i64 && folder_count < require_count as i64 {
            let folder_limit = folder_count as u32 - offset;
            let object_limit = limit - folder_limit;
            let folder_request = Request::new(GetFolderListRequest {
                limit: folder_limit,
                offset: *offset,
                auth: auth.clone(),
                path: path.to_string(),
                bucket_name: bucket_name.to_string(),
            });
            let object_request = Request::new(GetFolderListRequest {
                limit: object_limit,
                offset: 0,
                auth: auth.clone(),
                path: path.to_string(),
                bucket_name: bucket_name.to_string(),
            });
            let (folder_data, object_data) = futures::future::try_join(
                folder_client.get_folder_list(folder_request),
                object_client.get_object_list(object_request),
            )
            .await?;
            let mut data = vec![];
            folder_data
                .into_inner()
                .data
                .into_iter()
                .for_each(|x| data.push(FolderInfo::from((x, op_auth.clone())).into()));
            object_data
                .into_inner()
                .data
                .into_iter()
                .for_each(|x| data.push(ObjectInfo::from(x).into()));
            Ok(FolderList { total, data })
        } else {
            let object_request = Request::new(GetFolderListRequest {
                limit: *limit,
                offset: offset - folder_count as u32,
                auth: auth.clone(),
                path: path.to_string(),
                bucket_name: bucket_name.to_string(),
            });
            let data = object_client
                .get_object_list(object_request)
                .await?
                .into_inner()
                .data
                .into_iter()
                .map(|x| ObjectInfo::from(x).into())
                .collect();
            Ok(FolderList { data, total })
        }
    }
    /// 获取存储桶信息
    async fn bucket_info(&self, data: GetBucketRequest) -> GraphqlResult<BucketInfo> {
        let mut client = BucketClient::connect("http://core:80").await?;
        let res = client.get_bucket(data).await?.into_inner();
        Ok(res.into())
    }
    /// 获取文件夹信息
    async fn folder_info(&self, data: GetFolderRequest) -> GraphqlResult<FolderInfo> {
        let mut client = FolderClient::connect("http://core:80").await?;
        let auth = data.auth.clone();
        let res = client.get_folder(data).await?.into_inner();
        Ok((res, auth).into())
    }
    /// 获取对象信息
    async fn object_info(&self, data: GetObjectRequest) -> GraphqlResult<ObjectInfo> {
        let mut client = ObjectClient::connect("http://core:80").await?;
        let res = client.get_object(data).await?.into_inner();
        Ok(res.into())
    }
}
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// 用户创建
    async fn manage_user_create(&self, data: CreateUserRequest) -> GraphqlResult<UserInfo> {
        let mut client = UserManageClient::connect("http://user:80").await?;
        let res = client.create_user(data).await?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户更新
    async fn manage_user_update(&self, data: UpdateUserRequest) -> GraphqlResult<UserInfo> {
        let mut client = UserManageClient::connect("http://user:80").await?;
        let res = client.update_user(data).await?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户删除
    async fn manage_user_delete(&self, data: DeleteUserRequest) -> GraphqlResult<bool> {
        let mut client = UserManageClient::connect("http://user:80").await?;
        client.delete_user(data).await?;
        Ok(true)
    }
    /// 用户更新密码
    async fn update_password(&self, data: UpdatePasswordRequest) -> GraphqlResult<String> {
        let mut client = SelfManageClient::connect("http://user:80").await?;
        let LoginReply { auth } = client.update_password(data).await?.into_inner();
        Ok(auth)
    }
    /// 用户更新信息
    async fn update_info(&self, data: UpdateUserInfoRequest) -> GraphqlResult<UserInfo> {
        let mut client = SelfManageClient::connect("http://user:80").await?;
        let res = client.update_user_info(data).await?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 创建存储桶
    async fn create_bucket(&self, data: CreateBucketRequest) -> GraphqlResult<BucketInfo> {
        let mut client = BucketClient::connect("http://core:80").await?;
        let res = client.create_bucket(data).await?;
        Ok(BucketInfo::from(res.into_inner()))
    }
    /// 更新存储桶
    async fn update_bucket(&self, data: UpdateBucketRequest) -> GraphqlResult<BucketInfo> {
        let mut client = BucketClient::connect("http://core:80").await?;
        let res = client.update_bucket(data).await?;
        Ok(BucketInfo::from(res.into_inner()))
    }
    /// 删除存储桶
    async fn delete_bucket(&self, data: DeleteBucketRequest) -> GraphqlResult<bool> {
        let mut client = BucketClient::connect("http://core:80").await?;
        client.delete_bucket(data).await?;
        Ok(true)
    }
    /// 创建目录
    async fn create_folder(&self, data: CreateFolderRequest) -> GraphqlResult<FolderInfo> {
        let mut client = FolderClient::connect("http://core:80").await?;
        let auth = data.auth.clone();
        let res = client.create_folder(data).await?.into_inner();
        Ok(FolderInfo::from((res, auth)))
    }
    /// 更新目录
    async fn update_folder(&self, data: UpdateFolderRequest) -> GraphqlResult<FolderInfo> {
        let mut client = FolderClient::connect("http://core:80").await?;
        let auth = data.auth.clone();
        let res = client.update_folder(data).await?.into_inner();
        Ok(FolderInfo::from((res, auth)))
    }
    /// 删除目录
    async fn delete_folder(&self, data: DeleteFolderRequest) -> GraphqlResult<bool> {
        let mut client = FolderClient::connect("http://core:80").await?;
        client.delete_folder(data).await?;
        Ok(true)
    }
    /// 更新对象
    async fn update_object(&self, data: UpdateObjectRequest) -> GraphqlResult<ObjectInfo> {
        let mut client = ObjectClient::connect("http://core:80").await?;
        let res = client.update_object(data).await?;
        Ok(res.into_inner().into())
    }
    /// 删除对象
    async fn delete_object(&self, data: DeleteObjectRequest) -> GraphqlResult<bool> {
        let mut client = ObjectClient::connect("http://core:80").await?;
        client.delete_object(data).await?;
        Ok(true)
    }
}
