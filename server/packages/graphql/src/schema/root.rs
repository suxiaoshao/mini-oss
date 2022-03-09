use async_graphql::{Context, Object};

use proto::core::{
    CountReply, DeleteObjectRequest, GetBucketRequest, GetFolderListReply, GetFolderListRequest,
    GetFolderRequest, GetObjectRequest,
};
use proto::middleware::client::{
    bucket_client, folder_client, login_client, object_client, self_manage_client,
    user_manage_client,
};
use proto::user::{Empty, LoginReply, LoginRequest};
use proto::{
    core::{DeleteBucketRequest, DeleteFolderRequest},
    user::{
        CreateUserRequest, DeleteUserRequest, GetListRequest, GetUserRequest,
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
    async fn manager_login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: LoginRequest,
    ) -> GraphqlResult<String> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = login_client(auth).await?;
        let res = client.manager_login(data).await?;
        Ok(res.get_ref().auth.to_string())
    }
    /// 用户登陆
    async fn user_login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: LoginRequest,
    ) -> GraphqlResult<String> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = login_client(auth).await?;
        let res = client.user_login(data).await?;
        Ok(res.get_ref().auth.to_string())
    }
    /// 用户列表
    async fn user_list<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: GetListRequest,
    ) -> GraphqlResult<UserList> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = user_manage_client(auth).await?;
        let reply = client.get_user_list(data).await?;
        Ok(UserList::from(reply.into_inner()))
    }
    /// 用户信息
    async fn user_info<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: GetUserRequest,
    ) -> GraphqlResult<UserInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = user_manage_client(auth).await?;
        let reply = client.get_user(data).await?;
        Ok(UserInfo::from(reply.into_inner()))
    }
    /// 获取自身用户信息
    async fn self_user_info<'ctx>(&self, ctx: &Context<'ctx>) -> GraphqlResult<UserInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = self_manage_client(auth).await?;
        let reply = client.get_user_info(Empty {}).await?;
        Ok(UserInfo::from(reply.into_inner()))
    }
    /// 用户存储桶列表
    async fn bucket_list<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: GetListRequest,
    ) -> GraphqlResult<BucketList> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = bucket_client(auth).await?;
        let res = client.get_bucket_list(data).await?;
        Ok(BucketList::from(res.into_inner()))
    }
    /// 文件夹列表
    async fn folder_list<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: GetFolderListRequest,
    ) -> GraphqlResult<FolderList> {
        let GetFolderListRequest {
            limit,
            offset,
            path,
            bucket_name,
        } = &data;
        // 需要的总数
        let require_count = limit + offset;

        // 获取文件夹总数
        let _auth = ctx.data::<String>().ok().cloned();
        let mut folder_client = folder_client(_auth.clone()).await?;
        let request = Request::new(GetFolderRequest {
            path: path.clone(),
            bucket_name: bucket_name.clone(),
        });
        let CountReply {
            total: folder_count,
        } = folder_client.get_folder_count(request).await?.into_inner();

        // 获取对象总数
        let mut object_client = object_client(_auth).await?;
        let request = GetFolderRequest {
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
                    .map(|x| FolderInfo::from(x).into())
                    .collect(),
            })
            // 如果需要两种
        } else if folder_count > *offset as i64 && folder_count < require_count as i64 {
            let folder_limit = folder_count as u32 - offset;
            let object_limit = limit - folder_limit;
            let folder_request = Request::new(GetFolderListRequest {
                limit: folder_limit,
                offset: *offset,
                path: path.to_string(),
                bucket_name: bucket_name.to_string(),
            });
            let object_request = Request::new(GetFolderListRequest {
                limit: object_limit,
                offset: 0,
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
                .for_each(|x| data.push(FolderInfo::from(x).into()));
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
    async fn bucket_info<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: GetBucketRequest,
    ) -> GraphqlResult<BucketInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = bucket_client(auth).await?;
        let res = client.get_bucket(data).await?.into_inner();
        Ok(res.into())
    }
    /// 获取文件夹信息
    async fn folder_info<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: GetFolderRequest,
    ) -> GraphqlResult<FolderInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = folder_client(auth).await?;
        let res = client.get_folder(data).await?.into_inner();
        Ok(res.into())
    }
    /// 获取对象信息
    async fn object_info<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: GetObjectRequest,
    ) -> GraphqlResult<ObjectInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = object_client(auth).await?;
        let res = client.get_object(data).await?.into_inner();
        Ok(res.into())
    }
}
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// 用户创建
    async fn manage_user_create<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: CreateUserRequest,
    ) -> GraphqlResult<UserInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = user_manage_client(auth).await?;
        let res = client.create_user(data).await?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户更新
    async fn manage_user_update<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: UpdateUserRequest,
    ) -> GraphqlResult<UserInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = user_manage_client(auth).await?;
        let res = client.update_user(data).await?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 用户删除
    async fn manage_user_delete<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: DeleteUserRequest,
    ) -> GraphqlResult<bool> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = user_manage_client(auth).await?;
        client.delete_user(data).await?;
        Ok(true)
    }
    /// 用户更新密码
    async fn update_password<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: UpdatePasswordRequest,
    ) -> GraphqlResult<String> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = self_manage_client(auth).await?;
        let LoginReply { auth } = client.update_password(data).await?.into_inner();
        Ok(auth)
    }
    /// 用户更新信息
    async fn update_info<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: UpdateUserInfoRequest,
    ) -> GraphqlResult<UserInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = self_manage_client(auth).await?;
        let res = client.update_user_info(data).await?;
        Ok(UserInfo::from(res.into_inner()))
    }
    /// 创建存储桶
    async fn create_bucket<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: CreateBucketRequest,
    ) -> GraphqlResult<BucketInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = bucket_client(auth).await?;
        let res = client.create_bucket(data).await?;
        Ok(BucketInfo::from(res.into_inner()))
    }
    /// 更新存储桶
    async fn update_bucket<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: UpdateBucketRequest,
    ) -> GraphqlResult<BucketInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = bucket_client(auth).await?;
        let res = client.update_bucket(data).await?;
        Ok(BucketInfo::from(res.into_inner()))
    }
    /// 删除存储桶
    async fn delete_bucket<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: DeleteBucketRequest,
    ) -> GraphqlResult<bool> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = bucket_client(auth).await?;
        client.delete_bucket(data).await?;
        Ok(true)
    }
    /// 创建目录
    async fn create_folder<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: CreateFolderRequest,
    ) -> GraphqlResult<FolderInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = folder_client(auth).await?;
        let res = client.create_folder(data).await?.into_inner();
        Ok(FolderInfo::from(res))
    }
    /// 更新目录
    async fn update_folder<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: UpdateFolderRequest,
    ) -> GraphqlResult<FolderInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = folder_client(auth).await?;
        let res = client.update_folder(data).await?.into_inner();
        Ok(FolderInfo::from(res))
    }
    /// 删除目录
    async fn delete_folder<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: DeleteFolderRequest,
    ) -> GraphqlResult<bool> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = folder_client(auth).await?;
        client.delete_folder(data).await?;
        Ok(true)
    }
    /// 更新对象
    async fn update_object<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: UpdateObjectRequest,
    ) -> GraphqlResult<ObjectInfo> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = object_client(auth).await?;
        let res = client.update_object(data).await?;
        Ok(res.into_inner().into())
    }
    /// 删除对象
    async fn delete_object<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: DeleteObjectRequest,
    ) -> GraphqlResult<bool> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = object_client(auth).await?;
        client.delete_object(data).await?;
        Ok(true)
    }
}
