use validator::ValidationError;

pub fn validate_folder(input: &str) -> Result<(), ValidationError> {
    if input.chars().any(|x| x == '/') {
        Err(ValidationError::new("不能含有/"))
    } else {
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use super::validate_folder;

    #[test]
    fn test_validate_password() {
        let input = "abc";
        assert!(validate_folder(input).is_ok());
        let input = "abcdeabcdeabcdeabcdeabcdec";
        assert!(validate_folder(input).is_ok());
        let input = "2a中sadsd";
        assert!(validate_folder(input).is_ok());
        let input = "12345678";
        assert!(validate_folder(input).is_ok());
        let input = "qwertyui";
        assert!(validate_folder(input).is_ok());
        let input = "!@#$/%^&*";
        assert!(validate_folder(input).is_err());
        let input = "asGk22_Q";
        assert!(validate_folder(input).is_ok());
    }
}
