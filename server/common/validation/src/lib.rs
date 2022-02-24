use errors::TonicResult;
use proto::validation::Validate;

pub mod check_auth;

pub trait TonicValidate: Validate {
    fn validate(&self) -> TonicResult<()> {
        Validate::validate(self)?;
        Ok(())
    }
}
impl<T: Validate> TonicValidate for T {}
