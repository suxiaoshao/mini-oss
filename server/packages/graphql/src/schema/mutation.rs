use crate::errors::GraphqlResult;
use crate::schema::bucket::bucket_info::BucketInfo;
use crate::schema::bucket::request::{CreateBucketRequest, UpdateBucketRequest};
use crate::schema::folder::folder_info::FolderInfo;
use crate::schema::folder::request::{CreateFolderRequest, UpdateFolderRequest};
use crate::schema::object::object_info::ObjectInfo;
use crate::schema::object::request::UpdateObjectRequest;
use crate::schema::user::UserInfo;
use async_graphql::{Context, Object};
use proto::core::{DeleteBucketRequest, DeleteFolderRequest, DeleteObjectRequest};
use proto::middleware::client::{
    bucket_client, folder_client, object_client, self_manage_client, user_manage_client,
};
use proto::user::{
    CreateUserRequest, DeleteUserRequest, LoginReply, UpdatePasswordRequest, UpdateUserInfoRequest,
    UpdateUserRequest,
};

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
