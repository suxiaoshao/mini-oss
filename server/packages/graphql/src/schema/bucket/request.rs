use async_graphql::InputObject;
use proto::core::{self, Access};
#[derive(InputObject)]
pub struct UpdateBucketRequest {
    pub name: String,
    pub access: Access,
    pub auth: String,
}

#[allow(clippy::from_over_into)]
impl Into<core::UpdateBucketRequest> for UpdateBucketRequest {
    fn into(self) -> core::UpdateBucketRequest {
        let Self { name, access, auth } = self;
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
    pub access: Access,
    pub auth: String,
}

#[allow(clippy::from_over_into)]
impl Into<core::CreateBucketRequest> for CreateBucketRequest {
    fn into(self) -> core::CreateBucketRequest {
        let Self { name, access, auth } = self;
        let mut request = core::CreateBucketRequest {
            name,
            access: 0,
            auth,
        };
        request.set_access(access);
        request
    }
}
