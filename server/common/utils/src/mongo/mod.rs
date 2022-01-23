use std::env::var;

use mongodb::{options::ClientOptions, Client, Database};
use mongodb_gridfs::{options::GridFSBucketOptions, GridFSBucket};
use proto::Status;

use crate::errors::grpc::ToStatusResult;

pub struct Mongo {
    database: Database,
}

impl Mongo {
    /// 新建 mongo连接
    pub async fn new(url: &str) -> Result<Self, Status> {
        let client_options = ClientOptions::parse(url).await.to_status()?;
        let client = Client::with_options(client_options).to_status()?;
        let database = client.database(&var("bucket_database").to_status()?);
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
    pub async fn drop_self(&self, name: String) -> Result<(), Status> {
        let bucket = self.bucket(name);
        bucket.drop().await.to_status()
    }
}

#[cfg(test)]
mod test {
    use mongodb_gridfs::options::GridFSBucketOptions;
    use proto::Status;

    use crate::errors::grpc::ToStatusResult;

    #[tokio::test]
    async fn test() -> Result<(), Status> {
        use mongodb::{options::ClientOptions, Client};

        // Parse a connection string into an options struct.
        let client_options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .to_status()?;

        // Get a handle to the deployment.
        let client = Client::with_options(client_options).to_status()?;
        let db = client.database("testss");

        use mongodb_gridfs::GridFSBucket;
        let mut bucket = GridFSBucket::new(db.clone(), Some(GridFSBucketOptions::default()));
        let id = bucket
            .upload_from_stream("sushao/test.txt", "stream your data here".as_bytes(), None)
            .await
            .to_status()?;
        println!("{id}");
        // bucket.drop().await.to_status()?;
        Ok(())
    }
}
