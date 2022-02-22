use errors::TonicResult;
use proto::validation::Validate;

pub mod check_auth;

pub fn validate<T: Validate>(data: &T) -> TonicResult<()> {
    data.validate()?;
    Ok(())
}
