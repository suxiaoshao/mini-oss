use async_graphql::{Context, Object};

use proto::core::{
    CountReply, GetBucketRequest, GetFolderListReply, GetFolderListRequest, GetFolderRequest,
    GetObjectRequest,
};
use proto::middleware::client::{
    bucket_client, folder_client, login_client, object_client, self_manage_client,
    user_manage_client,
};
use proto::user::{Empty, LoginRequest};
use proto::{
    user::{GetListRequest, GetUserRequest},
    Request,
};

use crate::errors::GraphqlResult;
use crate::schema::folder::folder_list::FolderList;
use crate::schema::object::object_info::ObjectInfo;

use super::{
    bucket::bucket_info::{BucketInfo, BucketList},
    folder::folder_info::FolderInfo,
    user::{UserInfo, UserList},
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
