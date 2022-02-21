use database::{bucket::BucketModal, folder::FolderModal, object::ObjectModal, Pool, Postgres};
use errors::{TonicError, TonicResult};
use proto::Status;
use validation::check_auth::check_user;
pub async fn check_bucket(auth: &str, bucket_name: &str, pool: &Pool<Postgres>) -> TonicResult<()> {
    // 判断用户
    let username = check_user(auth).await?;
    // 判断该存储桶是否存在
    BucketModal::exist(bucket_name, pool)
        .await
        .map_err(|_| TonicError::BucketNotFound(bucket_name.to_string()))?;
    // 判断用户是否一致
    let BucketModal {
        username: modal_username,
        ..
    } = BucketModal::find_one(bucket_name, pool).await?;
    if username != modal_username {
        return Err(TonicError::BucketPermission);
    }
    Ok(())
}

/// 判断 bucket 权限
async fn check_bucket_permission(
    auth: &str,
    bucket_name: &str,
    pool: &Pool<Postgres>,
) -> TonicResult<()> {
    // 判断用户
    let username = check_user(auth).await?;
    // 判断用户是否一致
    let BucketModal {
        username: modal_username,
        ..
    } = BucketModal::find_one(bucket_name, pool).await?;
    if username != modal_username {
        return Err(TonicError::BucketPermission);
    }
    Ok(())
}

/// 判断文件夹是否存在
async fn check_folder_exits(
    bucket_name: &str,
    path: &str,
    pool: &Pool<Postgres>,
) -> Result<(), Status> {
    // 判断该存储桶是否存在
    BucketModal::exist(bucket_name, pool)
        .await
        .map_err(|_| TonicError::BucketNotFound(bucket_name.to_string()))?;
    // 判断该文件夹是否存在
    FolderModal::exist(path, bucket_name, pool)
        .await
        .map_err(|_| TonicError::FolderNotFound(bucket_name.to_string()))?;
    Ok(())
}

/// 验证该文件夹是否可以读
pub async fn check_folder_readable(
    auth: &Option<String>,
    bucket_name: &str,
    path: &str,
    pool: &Pool<Postgres>,
) -> TonicResult<()> {
    // 判断该文件夹是否存在
    check_folder_exits(bucket_name, path, pool).await?;
    if FolderModal::read_open(path, bucket_name, pool).await? {
        // 如果文件夹是开放的话不用判断是否是
        Ok(())
    } else {
        match auth {
            Some(auth) => check_bucket_permission(auth, bucket_name, pool).await,
            None => Err(TonicError::NoneAuth),
        }
    }
}

/// 验证该文件夹是否可以操作
pub async fn check_folder_writeable(
    auth: &Option<String>,
    bucket_name: &str,
    path: &str,
    pool: &Pool<Postgres>,
) -> TonicResult<()> {
    // 判断该文件夹是否存在
    check_folder_exits(bucket_name, path, pool).await?;
    if FolderModal::write_open(path, bucket_name, pool).await? {
        // 如果文件夹是开放的话不用判断是否是
        Ok(())
    } else {
        match auth {
            Some(auth) => check_bucket_permission(auth, bucket_name, pool).await,
            None => Err(TonicError::NoneAuth),
        }
    }
}

/// 判断对象是否存在
async fn check_object_exits(
    bucket_name: &str,
    path: &str,
    filename: &str,
    pool: &Pool<Postgres>,
) -> Result<(), Status> {
    // 判断该文件夹是否存在
    check_folder_exits(bucket_name, path, pool).await?;
    // 判断对象是否存在
    ObjectModal::exist(path, bucket_name, filename, pool)
        .await
        .map_err(|_| TonicError::ObjectNotFound(filename.to_string()))?;
    Ok(())
}
/// 判断对象是否可读
pub async fn check_object_readable(
    auth: &Option<String>,
    bucket_name: &str,
    path: &str,
    filename: &str,
    pool: &Pool<Postgres>,
) -> TonicResult<()> {
    // 判断对象是否存在
    check_object_exits(bucket_name, path, filename, pool).await?;
    if ObjectModal::read_open(path, bucket_name, filename, pool).await? {
        // 如果对象是开放的话不用判断是否是
        Ok(())
    } else {
        match auth {
            Some(auth) => check_bucket_permission(auth, bucket_name, pool).await,
            None => Err(TonicError::NoneAuth),
        }
    }
}

/// 判断对象是否可写
pub async fn check_object_writeable(
    auth: &Option<String>,
    bucket_name: &str,
    path: &str,
    filename: &str,
    pool: &Pool<Postgres>,
) -> TonicResult<()> {
    // 判断对象是否存在
    check_object_exits(bucket_name, path, filename, pool).await?;
    if ObjectModal::write_open(path, bucket_name, filename, pool).await? {
        // 如果文件夹是开放的话不用判断是否是
        Ok(())
    } else {
        match auth {
            Some(auth) => check_bucket_permission(auth, bucket_name, pool).await,
            None => Err(TonicError::NoneAuth),
        }
    }
}
