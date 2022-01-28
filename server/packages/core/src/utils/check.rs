use proto::Status;
use utils::{
    database::{bucket::BucketModal, folder::FolderModal, object::ObjectModal, Pool, Postgres},
    validation::check_auth::check_user,
};
pub async fn check_bucket(
    auth: &str,
    bucket_name: &str,
    pool: &Pool<Postgres>,
) -> Result<(), Status> {
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
pub async fn check_path(
    auth: &str,
    bucket_name: &str,
    path: &str,
    pool: &Pool<Postgres>,
) -> Result<(), Status> {
    // 判断 bucket
    check_bucket(auth, bucket_name, pool).await?;
    // 判断父文件夹是否存在
    FolderModal::exist(path, bucket_name, pool)
        .await
        .map_err(|_| Status::not_found("文件夹不存在"))?;
    Ok(())
}
pub async fn check_object(
    auth: &str,
    bucket_name: &str,
    path: &str,
    filename: &str,
    pool: &Pool<Postgres>,
) -> Result<(), Status> {
    // 判断 目录
    check_path(auth, bucket_name, path, pool).await?;
    // 判断父文件夹是否存在
    ObjectModal::exist(path, bucket_name, filename, pool)
        .await
        .map_err(|_| Status::not_found("该对象不存在"))?;
    Ok(())
}
