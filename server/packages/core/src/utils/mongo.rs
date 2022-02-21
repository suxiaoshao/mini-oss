use std::{env::var, str::FromStr};

use futures::{AsyncRead, StreamExt};
use mongodb::{bson::oid::ObjectId, options::ClientOptions, Client, Database};
use mongodb_gridfs::{options::GridFSBucketOptions, GridFSBucket};

use errors::TonicResult;

pub struct Mongo {
    database: Database,
}

impl Mongo {
    /// 新建 mongo连接
    pub async fn new(url: &str) -> TonicResult<Self> {
        let client_options = ClientOptions::parse(url).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(&var("bucket_database")?);
        Ok(Self { database })
    }
}

impl Mongo {
    /// 创建同名 bucket
    fn bucket(&self, name: String) -> GridFSBucket {
        let options = GridFSBucketOptions::builder().bucket_name(name).build();
        GridFSBucket::new(self.database.clone(), Some(options))
    }
    /// 删除 bucket
    pub async fn drop_self(&self, name: String) -> TonicResult<()> {
        let bucket = self.bucket(name);
        Ok(bucket.drop().await?)
    }
    /// 添加文件
    pub async fn upload_file(
        &self,
        bucket_name: String,
        filename: &str,
        source: impl AsyncRead + Unpin,
    ) -> TonicResult<String> {
        let mut bucket = Self::bucket(self, bucket_name);
        let object_id = bucket.upload_from_stream(filename, source, None).await?;
        Ok(object_id.to_string())
    }
    /// 删除文件
    pub async fn delete_file(&self, bucket_name: String, id: &str) -> TonicResult<()> {
        let id = ObjectId::from_str(id)?;
        let bucket = self.bucket(bucket_name);
        bucket.delete(id).await?;
        Ok(())
    }
    /// 读取文件内容
    pub async fn read_file(&self, bucket_name: String, id: &str) -> TonicResult<Vec<u8>> {
        let id = ObjectId::from_str(id)?;
        let bucket = self.bucket(bucket_name);
        let stream = bucket.open_download_stream(id).await?;
        let mut content = vec![];
        stream
            .for_each(|mut x| {
                content.append(&mut x);
                futures::future::ready(())
            })
            .await;
        Ok(content)
    }
}

#[cfg(test)]
mod test {
    use errors::TonicResult;
    use mongodb_gridfs::options::GridFSBucketOptions;

    #[tokio::test]
    async fn test_write() -> TonicResult<()> {
        use mongodb::{options::ClientOptions, Client};

        // Parse a connection string into an options struct.
        let client_options =
            ClientOptions::parse("mongodb://sushao:sushao@localhost:27017").await?;

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;
        let db = client.database("testss");

        use mongodb_gridfs::GridFSBucket;
        let mut bucket = GridFSBucket::new(db.clone(), Some(GridFSBucketOptions::default()));
        let id = bucket
            .upload_from_stream("sushao/test.txt", "stream your data here".as_bytes(), None)
            .await?;
        println!("{id}");
        bucket.drop().await?;
        Ok(())
    }
}
