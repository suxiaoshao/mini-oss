use std::sync::Arc;

use proto::core::{
    CountReply, GetFolderRequest, GetObjectContentReply, GetObjectRequest, SizeReply,
};
use proto::{
    async_trait,
    auth::Empty,
    core::{
        object_server::Object, CreateObjectRequest, DeleteObjectRequest, GetFolderListRequest,
        GetObjectListReply, ObjectInfo, UpdateObjectRequest,
    },
    validation::Validate,
    Request, Response, Status,
};
use utils::database::object::ObjectAccess;
use utils::{
    database::{
        object::{ObjectCreateInput, ObjectModal},
        Pool, Postgres,
    },
    errors::grpc::ToStatusResult,
    mongo::Mongo,
    validation::hash::file_hash,
};

use crate::utils::check::{check_folder_writeable, check_object_writeable};
use crate::utils::{
    check::{check_folder_readable, check_object_readable},
    headers::headers_from,
};

#[derive(Clone)]
pub struct ObjectGreeter {
    pool: Arc<Pool<Postgres>>,
    mongo: Arc<Mongo>,
}

impl ObjectGreeter {
    pub fn new(pool: Arc<Pool<Postgres>>, mongo: Arc<Mongo>) -> Self {
        Self { pool, mongo }
    }
}
#[async_trait]
impl Object for ObjectGreeter {
    async fn create_object(
        &self,
        request: Request<CreateObjectRequest>,
    ) -> Result<Response<Empty>, Status> {
        // 验证
        request.get_ref().validate().to_status()?;
        let access = request.get_ref().access();
        let pool = &self.pool;
        let CreateObjectRequest {
            path,
            bucket_name,
            auth,
            content,
            filename,
            ..
        } = request.into_inner();
        // 验证文件夹是否可写
        check_folder_writeable(&auth, &bucket_name, &path, pool).await?;
        create_object(
            &path,
            &bucket_name,
            &filename,
            &content,
            access,
            pool,
            &self.mongo,
        )
        .await?;
        Ok(Response::new(Empty {}))
    }
    async fn delete_object(
        &self,
        request: Request<DeleteObjectRequest>,
    ) -> Result<Response<Empty>, Status> {
        let pool = &self.pool;
        let DeleteObjectRequest {
            path,
            filename,
            bucket_name,
            auth,
        } = request.into_inner();
        // 判断对象是否可写
        check_object_writeable(&auth, &bucket_name, &path, &filename, pool).await?;
        let ObjectModal { object_id, .. } =
            ObjectModal::find_one(&path, &bucket_name, &filename, pool).await?;
        futures::future::try_join(
            ObjectModal::delete(&path, &bucket_name, &filename, pool),
            self.mongo.delete_file(bucket_name.clone(), &object_id),
        )
        .await?;
        Ok(Response::new(Empty {}))
    }
    async fn update_object(
        &self,
        request: Request<UpdateObjectRequest>,
    ) -> Result<Response<ObjectInfo>, Status> {
        // 验证
        request.get_ref().validate().to_status()?;
        let access = request.get_ref().access();
        let pool = &self.pool;
        let UpdateObjectRequest {
            path,
            filename,
            bucket_name,
            new_filename,
            auth,
            headers,
            ..
        } = request.into_inner();
        // 判断对象是否可写
        check_object_writeable(&auth, &bucket_name, &path, &filename, pool).await?;
        // 判断新文件是否存在
        if new_filename != filename
            && ObjectModal::exist(&path, &bucket_name, &new_filename, &self.pool)
                .await
                .is_ok()
        {
            return Err(Status::already_exists("新文件名重复"));
        }
        let data = ObjectModal::update(
            &path,
            access,
            &bucket_name,
            &filename,
            &new_filename,
            &headers,
            pool,
        )
        .await?;
        Ok(Response::new(data.try_into()?))
    }

    async fn get_object_list(
        &self,
        request: Request<GetFolderListRequest>,
    ) -> Result<Response<GetObjectListReply>, Status> {
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
        // 判断文件夹是否可读
        check_folder_readable(&auth, &bucket_name, &path, pool).await?;
        let (count, list) = futures::future::try_join(
            ObjectModal::count_by_father_path(&bucket_name, &path, pool),
            ObjectModal::find_many_by_path(*limit, *offset, &path, &bucket_name, pool),
        )
        .await?;
        let mut data = vec![];
        for item in list {
            data.push(item.try_into()?)
        }
        Ok(Response::new(GetObjectListReply { data, total: count }))
    }

    async fn get_object_count(
        &self,
        request: Request<GetFolderRequest>,
    ) -> Result<Response<CountReply>, Status> {
        let pool = &self.pool;
        let GetFolderRequest {
            path,
            bucket_name,
            auth,
        } = request.into_inner();
        // 判断文件夹是否可读
        check_folder_readable(&auth, &bucket_name, &path, pool).await?;
        let count = ObjectModal::count_by_father_path(&bucket_name, &path, pool).await?;
        Ok(Response::new(CountReply { total: count }))
    }

    async fn get_object(
        &self,
        request: Request<GetObjectRequest>,
    ) -> Result<Response<ObjectInfo>, Status> {
        let pool = &self.pool;
        let GetObjectRequest {
            auth,
            bucket_name,
            path,
            filename,
        } = request.into_inner();
        // 判断对象是否可读
        check_object_readable(&auth, &bucket_name, &path, &filename, pool).await?;
        let object = ObjectModal::find_one(&path, &bucket_name, &filename, pool).await?;
        Ok(Response::new(object.try_into()?))
    }

    async fn get_object_content(
        &self,
        request: Request<GetObjectRequest>,
    ) -> Result<Response<GetObjectContentReply>, Status> {
        let pool = &self.pool;
        let GetObjectRequest {
            auth,
            bucket_name,
            path,
            filename,
        } = request.into_inner();
        // 判断对象是否可读
        check_object_readable(&auth, &bucket_name, &path, &filename, pool).await?;
        let ObjectModal { object_id, .. } =
            ObjectModal::find_one(&path, &bucket_name, &filename, pool).await?;
        let content = self.mongo.read_file(bucket_name, &object_id).await?;
        Ok(Response::new(GetObjectContentReply { content }))
    }

    async fn get_total_by_folder(
        &self,
        request: Request<GetFolderRequest>,
    ) -> Result<Response<CountReply>, Status> {
        let pool = &self.pool;
        let GetFolderRequest {
            auth,
            bucket_name,
            path,
        } = request.into_inner();
        check_folder_readable(&auth, &bucket_name, &path, pool).await?;
        let total = ObjectModal::count_by_path(&bucket_name, &path, pool).await?;
        Ok(Response::new(CountReply { total }))
    }

    async fn get_size_by_total(
        &self,
        request: Request<GetFolderRequest>,
    ) -> Result<Response<SizeReply>, Status> {
        let pool = &self.pool;
        let GetFolderRequest {
            auth,
            bucket_name,
            path,
        } = request.into_inner();
        check_folder_readable(&auth, &bucket_name, &path, pool).await?;
        let size = ObjectModal::size_by_path(&bucket_name, &path, pool).await?;
        Ok(Response::new(SizeReply { size }))
    }
}

/// 创建一个 object
async fn create_object(
    path: &str,
    bucket_name: &str,
    filename: &str,
    content: &[u8],
    access: impl Into<ObjectAccess>,
    pool: &Pool<Postgres>,
    mongo: &Mongo,
) -> Result<(), Status> {
    // 判断该文件是否存在
    if ObjectModal::exist(path, bucket_name, filename, pool)
        .await
        .is_ok()
    {
        return Err(Status::already_exists("文件名重复"));
    }
    // 获取数据
    let blake3 = file_hash(content);
    let size = content.len();
    let object_id = mongo
        .upload_file(bucket_name.to_string(), filename, content)
        .await?;
    let input = ObjectCreateInput {
        path,
        access: &access.into(),
        bucket_name,
        filename,
        blake3: &blake3,
        object_id: &object_id,
        size: size as i64,
        headers: &headers_from(filename),
    };
    ObjectModal::create(input, pool).await?;
    Ok(())
}
