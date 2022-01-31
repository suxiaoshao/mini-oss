use std::sync::Arc;

use proto::core::{CountReply, GetFolderCountRequest};
use proto::{
    async_trait,
    auth::Empty,
    core::{
        folder_server::Folder, CreateFolderRequest, DeleteFolderRequest, FolderInfo,
        GetFolderListReply, GetFolderListRequest, UpdateFolderRequest,
    },
    validation::Validate,
    Request, Response, Status,
};
use utils::database::{folder::FolderModal, Pool, Postgres};
use utils::mongo::Mongo;
use utils::{database::object::ObjectModal, errors::grpc::ToStatusResult};

use crate::utils::check::check_path;

#[derive(Clone)]
pub struct FolderGreeter {
    pool: Arc<Pool<Postgres>>,
    mongo: Arc<Mongo>,
}

impl FolderGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>, mongo: Arc<Mongo>) -> Self {
        Self { pool, mongo }
    }
}
#[async_trait]
impl Folder for FolderGreeter {
    async fn create_folder(
        &self,
        request: Request<CreateFolderRequest>,
    ) -> Result<Response<FolderInfo>, Status> {
        // 验证
        request.get_ref().validate().to_status()?;
        let pool = &self.pool;
        let access = request.get_ref().access();
        let CreateFolderRequest {
            path,
            bucket_name,
            father_path,
            auth,
            ..
        } = request.into_inner();
        // 判断父文件夹是否存在
        check_path(&auth, &bucket_name, &father_path, pool).await?;
        let path = format!("{father_path}/{path}");
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
        // 判断文件夹
        check_path(&auth, &bucket_name, &path, pool).await?;
        // 获取所有文件夹下的文件夹
        let delete_names = FolderModal::recursive_names_by_path(&path, &bucket_name, pool).await?;
        let delete_objects = ObjectModal::find_by_paths(&bucket_name, &delete_names, pool).await?;
        // 删除 sql 中 object
        let delete_object_sql = futures::future::try_join_all(delete_objects.iter().map(
            |ObjectModal {
                 filename,
                 path,
                 bucket_name,
                 ..
             }| ObjectModal::delete(path, bucket_name, filename, pool),
        ));
        // 删除 mongo 中对象
        let delete_object_mongo = futures::future::try_join_all(delete_objects.iter().map(
            |ObjectModal {
                 object_id,
                 bucket_name,
                 ..
             }| self.mongo.delete_file(bucket_name.clone(), object_id),
        ));
        // 删除 folder
        let delete_folders = futures::future::try_join_all(
            delete_names
                .iter()
                .map(|path| FolderModal::delete(path, &bucket_name, pool)),
        );
        futures::future::try_join3(delete_object_sql, delete_object_mongo, delete_folders).await?;
        Ok(Response::new(Empty {}))
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
        // 判断文件夹
        check_path(&auth, &bucket_name, &path, pool).await?;
        let updated = FolderModal::update(&path, access, &bucket_name, pool).await?;
        Ok(Response::new(updated.into()))
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
        // 判断文件夹
        check_path(&auth, &bucket_name, &path, pool).await?;
        let (count, data) = futures::future::try_join(
            FolderModal::count_by_father_path(&bucket_name, &path, pool),
            FolderModal::find_many_by_father_path(*limit, *offset, &path, &bucket_name, pool),
        )
        .await?;
        Ok(Response::new(GetFolderListReply {
            data: data.into_iter().map(|x| x.into()).collect(),
            total: count,
        }))
    }

    async fn get_folder_count(
        &self,
        request: Request<GetFolderCountRequest>,
    ) -> Result<Response<CountReply>, Status> {
        let pool = &self.pool;
        let GetFolderCountRequest {
            path,
            bucket_name,
            auth,
        } = request.into_inner();
        // 判断文件夹
        check_path(&auth, &bucket_name, &path, pool).await?;
        let count = FolderModal::count_by_father_path(&bucket_name, &path, pool).await?;
        Ok(Response::new(CountReply { total: count }))
    }
}
