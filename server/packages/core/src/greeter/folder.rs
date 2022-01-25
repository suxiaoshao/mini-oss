use std::sync::Arc;

use proto::{
    async_trait,
    auth::Empty,
    core::{
        folder_server::Folder, CreateFolderRequest, DeleteFolderRequest, FolderInfo,
        GetFolderListReply, GetFolderListRequest, UpdateFolderRequest,
    },
    Request, Response, Status,
};
use utils::{
    database::{bucket::BucketModal, folder::FolderModal, Pool, Postgres},
    validation::check_auth::check_user,
};
#[derive(Clone)]
pub struct FolderGreeter {
    pool: Arc<Pool<Postgres>>,
}

impl FolderGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl Folder for FolderGreeter {
    async fn create_folder(
        &self,
        request: Request<CreateFolderRequest>,
    ) -> Result<Response<FolderInfo>, Status> {
        let pool = &self.pool;
        let access = request.get_ref().access();
        let CreateFolderRequest {
            path,
            bucket_name,
            father_path,
            auth,
            ..
        } = request.into_inner();
        check_bucket(&auth, &bucket_name, pool).await?;
        // 判断父文件夹是否存在
        FolderModal::exist(&father_path, &bucket_name, pool)
            .await
            .map_err(|_| Status::not_found("父文件夹不存在"))?;
        // 判断该文件夹是否存在
        if FolderModal::exist(&path, &bucket_name, &self.pool)
            .await
            .is_ok()
        {
            return Err(Status::already_exists("文件夹名重复"));
        }
        let folder = FolderModal::create(&path, access, &bucket_name, &father_path, pool).await?;
        Ok(Response::new(folder.into()))
    }
    async fn update_folder(
        &self,
        request: Request<UpdateFolderRequest>,
    ) -> Result<Response<FolderInfo>, Status> {
        let pool = &self.pool;
        let access = request.get_ref().access();
        let UpdateFolderRequest {
            path,
            bucket_name,
            auth,
            ..
        } = request.into_inner();
        check_bucket(&auth, &bucket_name, pool).await?;
        // 判断该文件夹是否存在
        FolderModal::exist(&path, &bucket_name, pool)
            .await
            .map_err(|_| Status::not_found("该文件夹不存在"))?;
        let updated = FolderModal::update(&path, access, &bucket_name, pool).await?;
        Ok(Response::new(updated.into()))
    }
    async fn delete_folder(
        &self,
        request: Request<DeleteFolderRequest>,
    ) -> Result<Response<Empty>, Status> {
        let pool = &self.pool;
        let DeleteFolderRequest {
            path,
            bucket_name,
            auth,
        } = request.into_inner();
        check_bucket(&auth, &bucket_name, pool).await?;
        // 判断该文件夹是否存在
        FolderModal::exist(&path, &bucket_name, pool)
            .await
            .map_err(|_| Status::not_found("该文件夹不存在"))?;
        // 获取所有文件夹下的文件夹
        let delete_names = FolderModal::recursive_names_by_path(&path, &bucket_name, pool).await?;
        futures::future::try_join_all(
            delete_names
                .iter()
                .map(|path| FolderModal::delete(path, &bucket_name, pool)),
        )
        .await?;
        Ok(Response::new(Empty {}))
    }

    async fn get_folder_list(
        &self,
        request: Request<GetFolderListRequest>,
    ) -> Result<Response<GetFolderListReply>, Status> {
        let pool = &self.pool;
        let GetFolderListRequest {
            path,
            bucket_name,
            auth,
            limit,
            offset,
        } = request.into_inner();
        let limit = &limit;
        let limit = if limit > &50 { &50 } else { limit };
        let offset = &offset;
        check_bucket(&auth, &bucket_name, pool).await?;
        // 判断父文件夹是否存在
        FolderModal::exist(&path, &bucket_name, pool)
            .await
            .map_err(|_| Status::not_found("该文件夹不存在"))?;
        let (count, data) = tokio::join!(
            FolderModal::count_by_father_path(&bucket_name, &path, pool),
            FolderModal::find_many_by_father_path(*limit, *offset, &path, &bucket_name, pool)
        );
        Ok(Response::new(GetFolderListReply {
            data: data?.into_iter().map(|x| x.into()).collect(),
            total: count?,
        }))
    }
}

async fn check_bucket(auth: &str, bucket_name: &str, pool: &Pool<Postgres>) -> Result<(), Status> {
    // 判断用户
    let username = check_user(auth).await?;
    // 判断该存储桶是否存在
    BucketModal::exist(bucket_name, pool)
        .await
        .map_err(|_| Status::not_found("该存储桶不存在"))?;
    // 判断用户是否一致
    let BucketModal {
        username: modal_username,
        ..
    } = BucketModal::find_one(bucket_name, pool).await?;
    if username != modal_username {
        return Err(Status::permission_denied("没有权限操作不属于你的存储桶"));
    }
    Ok(())
}
