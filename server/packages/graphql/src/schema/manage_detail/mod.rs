use crate::errors::GraphqlResult;
use async_graphql::{Context, Object};
use proto::core::{
    CountChartItem, CountChartReply, CountDurationItem, CountDurationReply, GetTimeRequest,
    SizeChartItem, SizeChartReply, SizeDurationItem, SizeDurationReply, SizeReply,
};
use proto::middleware::client::{
    object_client, request_client, storage_client, user_manage_client,
};
use proto::user::{CountReply, Empty};

pub struct ManageDetail;

#[Object]
impl ManageDetail {
    /// 统计信息

    /// 对象大小
    async fn object_size<'ctx>(&self, ctx: &Context<'ctx>) -> GraphqlResult<String> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = object_client(auth).await?;
        let SizeReply { size } = client.get_size(Empty {}).await?.into_inner();
        Ok(size)
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
            .get_upload_size(GetTimeRequest {
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
            .get_download_size(GetTimeRequest {
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
            .get_count(GetTimeRequest {
                start_time,
                end_time,
            })
            .await?
            .into_inner();
        Ok(total)
    }
    /// 用户数量
    async fn user_count<'ctx>(&self, ctx: &Context<'ctx>) -> GraphqlResult<i64> {
        let auth = ctx.data::<String>().ok().cloned();
        let mut client = user_manage_client(auth).await?;
        let CountReply { total } = client.get_count(Empty {}).await?.into_inner();
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
            .get_size_chart(GetTimeRequest {
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
            .get_count_chart(GetTimeRequest {
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
            .get_upload_duration(GetTimeRequest {
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
            .get_download_duration(GetTimeRequest {
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
            .get_count_duration(GetTimeRequest {
                start_time,
                end_time,
            })
            .await?
            .into_inner();
        Ok(data)
    }
}
