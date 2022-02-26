use std::thread::park;

use anyhow::Result;
use delay_timer::prelude::*;

use database::PgPoolOptions;
use database::object::ObjectModal;
use database::storage::StorageModal;

#[tokio::main]
async fn main() -> Result<()> {
    let delay_timer = DelayTimerBuilder::default().build();

    delay_timer.insert_task(build_storage_task()?)?;
    park();
    delay_timer.stop_delay_timer()?;

    Ok(())
}

fn build_storage_task() -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();
    task_builder
        .set_frequency_repeated_by_minutes(5)
        .spawn_async_routine(storage_task)
}

async fn storage_task() -> Result<()> {
    // 获取数据库连接池
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("postgres")?)
        .await?;
    let storage_data = ObjectModal::size_count_by_bucket(&pool).await?;

    let future_task = storage_data
        .iter()
        .map(|(size, num, bucket)| StorageModal::create(bucket, size, *num, &pool))
        .collect::<Vec<_>>();
    futures::future::try_join_all(future_task).await?;
    Ok(())
}
