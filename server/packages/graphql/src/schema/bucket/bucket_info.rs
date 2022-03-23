use async_graphql::{ComplexObject, Context, SimpleObject};

use proto::core::{
    CountChartItem, CountChartReply, CountDurationItem, CountDurationReply, GetBucketRequest,
    GetBucketWithTimeRequest, SizeChartItem, SizeChartReply, SizeDurationItem, SizeDurationReply,
    SizeReply,
};
use proto::middleware::client::{request_client, storage_client};
use proto::user::CountReply;
use proto::{
    core::{BucketAccess, GetBucketListReply},
    middleware::client::object_client,
};

use crate::errors::GraphqlResult;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct BucketInfo {
    /// 名字
    pub name: String,
    /// 创建时间
    pub create_time: i64,
    /// 更新时间
    pub update_time: i64,
    /// 访问权限
    pub access: BucketAccess,
    /// 用户名
    pub username: String,
}
/// 统计信息
#[ComplexObject]
impl BucketInfo {
    /// 统计信息

    /// 对象大小
    async fn object_size<'ctx>(&self, ctx: &Context<'ctx>) -> GraphqlResult<String> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = object_client(auth).await?;
        let SizeReply { size } = client
            .get_size_by_bucket(GetBucketRequest {
                bucket_name: self.name.clone(),
            })
            .await?
            .into_inner();
        Ok(size)
    }
    /// 对象数量
    async fn object_count<'ctx>(&self, ctx: &Context<'ctx>) -> GraphqlResult<i64> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = object_client(auth).await?;
        let CountReply { total } = client
            .get_total_by_bucket(GetBucketRequest {
                bucket_name: self.name.clone(),
            })
            .await?
            .into_inner();
        Ok(total)
    }
    /// 上传大小
    async fn upload_size<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        start_time: i64,
        end_time: i64,
    ) -> GraphqlResult<String> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = request_client(auth).await?;
        let SizeReply { size } = client
            .get_upload_size_by_bucket(GetBucketWithTimeRequest {
                bucket_name: self.name.clone(),
                start_time,
                end_time,
            })
            .await?
            .into_inner();
        Ok(size)
    }
    /// 下载大小
    async fn download_size<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        start_time: i64,
        end_time: i64,
    ) -> GraphqlResult<String> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = request_client(auth).await?;
        let SizeReply { size } = client
            .get_download_size_by_bucket(GetBucketWithTimeRequest {
                bucket_name: self.name.clone(),
                start_time,
                end_time,
            })
            .await?
            .into_inner();
        Ok(size)
    }
    /// 请求数量
    async fn request_count<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        start_time: i64,
        end_time: i64,
    ) -> GraphqlResult<i64> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = request_client(auth).await?;
        let CountReply { total } = client
            .get_count_by_bucket(GetBucketWithTimeRequest {
                bucket_name: self.name.clone(),
                start_time,
                end_time,
            })
            .await?
            .into_inner();
        Ok(total)
    }

    /// 图表信息

    /// 对象大小
    async fn object_size_chart<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        start_time: i64,
        end_time: i64,
    ) -> GraphqlResult<Vec<SizeChartItem>> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = storage_client(auth).await?;
        let SizeChartReply { data } = client
            .get_size_chart_by_bucket(GetBucketWithTimeRequest {
                bucket_name: self.name.clone(),
                start_time,
                end_time,
            })
            .await?
            .into_inner();
        Ok(data)
    }
    /// 对象数量
    async fn object_count_chart<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        start_time: i64,
        end_time: i64,
    ) -> GraphqlResult<Vec<CountChartItem>> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = storage_client(auth).await?;
        let CountChartReply { data } = client
            .get_count_chart_by_bucket(GetBucketWithTimeRequest {
                bucket_name: self.name.clone(),
                start_time,
                end_time,
            })
            .await?
            .into_inner();
        Ok(data)
    }
    /// 上传大小
    async fn upload_size_chart<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        start_time: i64,
        end_time: i64,
    ) -> GraphqlResult<Vec<SizeDurationItem>> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = request_client(auth).await?;
        let SizeDurationReply { data } = client
            .get_upload_duration_by_bucket(GetBucketWithTimeRequest {
                bucket_name: self.name.clone(),
                start_time,
                end_time,
            })
            .await?
            .into_inner();
        Ok(data)
    }
    /// 下载大小
    async fn download_size_chart<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        start_time: i64,
        end_time: i64,
    ) -> GraphqlResult<Vec<SizeDurationItem>> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = request_client(auth).await?;
        let SizeDurationReply { data } = client
            .get_download_duration_by_bucket(GetBucketWithTimeRequest {
                bucket_name: self.name.clone(),
                start_time,
                end_time,
            })
            .await?
            .into_inner();
        Ok(data)
    }
    /// 请求数量
    async fn request_count_chart<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        start_time: i64,
        end_time: i64,
    ) -> GraphqlResult<Vec<CountDurationItem>> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = request_client(auth).await?;
        let CountDurationReply { data } = client
            .get_count_duration_by_bucket(GetBucketWithTimeRequest {
                bucket_name: self.name.clone(),
                start_time,
                end_time,
            })
            .await?
            .into_inner();
        Ok(data)
    }
}

impl From<proto::core::BucketInfo> for BucketInfo {
    fn from(user: proto::core::BucketInfo) -> Self {
        let access = user.access();
        let proto::core::BucketInfo {
            name,
            create_time,
            update_time,
            username,
            ..
        } = user;
        Self {
            name,
            create_time,
            update_time,
            access,
            username,
        }
    }
}
#[derive(SimpleObject)]
pub struct BucketList {
    /// 总数
    pub total: i64,
    /// 数据
    pub data: Vec<BucketInfo>,
}

impl From<GetBucketListReply> for BucketList {
    fn from(list_user: GetBucketListReply) -> BucketList {
        Self {
            total: list_user.total,
            data: list_user.data.into_iter().map(|x| x.into()).collect(),
        }
    }
}
