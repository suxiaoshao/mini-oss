use std::sync::Arc;

use proto::{
    async_trait,
    auth::Empty,
    core::{
        object_server::Object, CreateObjectRequest, DeleteObjectRequest, GetFolderListRequest,
        GetObjectListReply, ObjectInfo, UpdateObjectRequest,
    },
    Request, Response, Status,
};
use utils::{
    database::{
        object::{ObjectCreateInput, ObjectModal},
        Pool, Postgres,
    },
    mongo::Mongo,
    validation::hash::file_hash,
};

use crate::utils::{
    check::{check_object, check_path},
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
    ) -> Result<Response<ObjectInfo>, Status> {
        let access = request.get_ref().access();
        let pool = &self.pool;
        let CreateObjectRequest {
            path,
            filename,
            bucket_name,
            content,
            auth,
            ..
        } = request.into_inner();
        // 验证文件夹
        check_path(&auth, &bucket_name, &path, pool).await?;
        // 判断该文件是否存在
        if ObjectModal::exist(&path, &bucket_name, &filename, &self.pool)
            .await
            .is_ok()
        {
            return Err(Status::already_exists("文件名重复"));
        }
        // 获取数据
        let blake3 = file_hash(content.as_slice());
        let size = content.len();
        let object_id = self
            .mongo
            .upload_file(bucket_name.clone(), &filename, content.as_slice())
            .await?;
        let input = ObjectCreateInput {
            path: &path,
            access: &access.into(),
            bucket_name: &bucket_name,
            filename: &filename,
            blake3: &blake3,
            object_id: &object_id,
            size: size as i64,
            headers: &headers_from(&filename),
        };
        let data = ObjectModal::create(input, pool).await?;
        Ok(Response::new(data.try_into()?))
    }
    async fn update_object(
        &self,
        request: Request<UpdateObjectRequest>,
    ) -> Result<Response<ObjectInfo>, Status> {
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
        // 判断对象是否存在
        check_object(&auth, &bucket_name, &path, &filename, pool).await?;
        // 判断新文件是否存在
        if ObjectModal::exist(&path, &bucket_name, &new_filename, &self.pool)
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
        // 判断对象是否存在
        check_object(&auth, &bucket_name, &path, &filename, pool).await?;
        let ObjectModal { object_id, .. } =
            ObjectModal::find_one(&path, &bucket_name, &filename, pool).await?;
        futures::future::try_join(
            ObjectModal::delete(&path, &bucket_name, &filename, pool),
            self.mongo.delete_file(bucket_name.clone(), &object_id),
        )
        .await?;
        Ok(Response::new(Empty {}))
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
        // 判断文件夹
        check_path(&auth, &bucket_name, &path, pool).await?;
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
}
