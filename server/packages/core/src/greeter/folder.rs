use std::sync::Arc;

use database::folder::FolderModal;
use database::object::ObjectModal;
use database::{Pool, Postgres};
use proto::core::GetFolderRequest;
use proto::user::{CountReply, Empty};
use proto::{
    async_trait,
    core::{
        folder_server::Folder, CreateFolderRequest, DeleteFolderRequest, FolderInfo,
        GetFolderListReply, GetFolderListRequest, UpdateFolderRequest,
    },
    Request, Response, Status,
};
use validation::TonicValidate;

use crate::utils::check::{check_folder_readable, check_folder_writeable};
use crate::utils::mongo::Mongo;

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
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        // 验证
        request.get_ref().validate()?;
        let pool = &self.pool;
        let access = request.get_ref().access();
        let CreateFolderRequest {
            path,
            bucket_name,
            father_path,
            ..
        } = request.into_inner();
        // 判断父文件夹是否可写
        check_folder_writeable(auth, &bucket_name, &father_path, pool).await?;
        let path = format!("{father_path}{path}/");
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
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        // 验证
        request.get_ref().validate()?;
        let pool = &self.pool;
        let DeleteFolderRequest { path, bucket_name } = request.into_inner();
        // 判断此文件夹是否可写
        check_folder_writeable(auth, &bucket_name, &path, pool).await?;
        // 获取文件夹下的所有 object
        let delete_objects = ObjectModal::find_by_paths(&bucket_name, &path, pool).await?;
        // 删除 sql 中 object
        let delete_object_sql = ObjectModal::delete_by_path(&bucket_name, &path, pool);
        // 删除 mongo 中对象
        let delete_object_mongo = futures::future::try_join_all(delete_objects.iter().map(
            |ObjectModal {
                 object_id,
                 bucket_name,
                 ..
             }| self.mongo.delete_file(bucket_name.clone(), object_id),
        ));
        // 删除 folder
        let delete_folders = FolderModal::delete_by_path(&bucket_name, &path, pool);
        futures::future::try_join3(delete_object_sql, delete_object_mongo, delete_folders).await?;
        Ok(Response::new(Empty {}))
    }
    async fn update_folder(
        &self,
        request: Request<UpdateFolderRequest>,
    ) -> Result<Response<FolderInfo>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        // 验证
        request.get_ref().validate()?;
        let pool = &self.pool;
        let access = request.get_ref().access();
        let UpdateFolderRequest {
            path, bucket_name, ..
        } = request.into_inner();
        // 判断文件夹是否可写
        check_folder_writeable(auth, &bucket_name, &path, pool).await?;
        let updated = FolderModal::update(&path, access, &bucket_name, pool).await?;
        Ok(Response::new(updated.into()))
    }

    async fn get_folder_list(
        &self,
        request: Request<GetFolderListRequest>,
    ) -> Result<Response<GetFolderListReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetFolderListRequest {
            path,
            bucket_name,
            limit,
            offset,
        } = request.into_inner();
        let limit = &limit;
        let limit = if limit > &50 { &50 } else { limit };
        let offset = &offset;
        // 判断文件夹是否可读
        check_folder_readable(auth, &bucket_name, &path, pool).await?;
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
        request: Request<GetFolderRequest>,
    ) -> Result<Response<CountReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetFolderRequest { path, bucket_name } = request.into_inner();
        // 判断文件夹
        check_folder_readable(auth, &bucket_name, &path, pool).await?;
        let count = FolderModal::count_by_father_path(&bucket_name, &path, pool).await?;
        Ok(Response::new(CountReply { total: count }))
    }

    async fn get_folder(
        &self,
        request: Request<GetFolderRequest>,
    ) -> Result<Response<FolderInfo>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetFolderRequest { bucket_name, path } = request.into_inner();
        check_folder_readable(auth, &bucket_name, &path, pool).await?;
        let folder = FolderModal::find_one(&path, &bucket_name, pool).await?;
        Ok(Response::new(folder.into()))
    }

    async fn get_total_by_folder(
        &self,
        request: Request<GetFolderRequest>,
    ) -> Result<Response<CountReply>, Status> {
        // 获取 auth
        let auth = request.extensions().get::<String>().cloned();
        let pool = &self.pool;
        let GetFolderRequest { bucket_name, path } = request.into_inner();
        check_folder_readable(auth, &bucket_name, &path, pool).await?;
        let total = FolderModal::count_by_path(&bucket_name, &path, pool).await?;
        Ok(Response::new(CountReply { total }))
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use proto::core::folder_client::FolderClient;
    use proto::core::DeleteFolderRequest;
    use proto::Request;

    #[tokio::test]
    async fn test() -> Result<()> {
        let mut client = FolderClient::connect("http://localhost:80").await?;
        let request = Request::new(DeleteFolderRequest {
            path: "/ss".to_string(),
            bucket_name: "as-sushao".to_string(),
        });
        client.delete_folder(request).await?;
        Ok(())
    }
}
