use errors::TonicResult;
use proto::validation::Validate;

pub mod check_auth;
#[cfg(all(feature = "jwt", feature = "password"))]
pub mod claims;
#[cfg(feature = "password")]
pub mod hash;
#[cfg(feature = "jwt")]
pub mod jwt_decode;

pub fn validate<T: Validate>(data: &T) -> TonicResult<()> {
    data.validate()?;
    Ok(())
}
