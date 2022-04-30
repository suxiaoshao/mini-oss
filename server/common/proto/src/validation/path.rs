use validator::ValidationError;

pub fn validate_path(input: &str) -> Result<(), ValidationError> {
    if input == "/" {
        return Err(ValidationError::new("不能为'/'"));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::validate_path;

    #[test]
    fn test_validate_path() {
        let input = "/abc";
        assert!(validate_path(input).is_ok());
        let input = "/";
        assert!(validate_path(input).is_err());
    }
}
