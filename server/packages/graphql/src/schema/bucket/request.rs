use async_graphql::InputObject;
use proto::core::{self, BucketAccess};
use proto::{IntoRequest, Request};

#[derive(InputObject)]
pub struct UpdateBucketRequest {
    pub name: String,
    pub access: BucketAccess,
    pub auth: String,
}

impl IntoRequest<core::UpdateBucketRequest> for UpdateBucketRequest {
    fn into_request(self) -> Request<core::UpdateBucketRequest> {
        Request::new(self.into())
    }
}

impl From<UpdateBucketRequest> for core::UpdateBucketRequest {
    fn from(value: UpdateBucketRequest) -> core::UpdateBucketRequest {
        let UpdateBucketRequest { name, access, auth } = value;
        let mut request = core::UpdateBucketRequest {
            name,
            access: 0,
            auth,
        };
        request.set_access(access);
        request
    }
}
#[derive(InputObject)]
pub struct CreateBucketRequest {
    pub name: String,
    pub access: BucketAccess,
    pub auth: String,
}

impl IntoRequest<core::CreateBucketRequest> for CreateBucketRequest {
    fn into_request(self) -> Request<core::CreateBucketRequest> {
        Request::new(self.into())
    }
}

impl From<CreateBucketRequest> for core::CreateBucketRequest {
    fn from(value: CreateBucketRequest) -> core::CreateBucketRequest {
        let CreateBucketRequest { name, access, auth } = value;
        let mut request = core::CreateBucketRequest {
            name,
            access: 0,
            auth,
        };
        request.set_access(access);
        request
    }
}
