use validator::ValidationError;

pub fn validate_bucket(input: &str) -> Result<(), ValidationError> {
    let len = input.chars().count();
    if len > 60 {
        return Err(ValidationError::new("不能大于60字符"));
    }
    if len < 1 {
        return Err(ValidationError::new("不能小于1个字符"));
    }
    if input
        .chars()
        .all(|x| matches!(x, 'a'..='z' | '0'..='9' | '-'))
    {
        Ok(())
    } else {
        Err(ValidationError::new("仅支持小写字母、数字和 - 的组合"))
    }
}
#[cfg(test)]
mod test {
    use super::validate_bucket;

    #[test]
    fn test_validate_bucket() {
        let input = "ab-1c";
        assert!(validate_bucket(input).is_ok());
        let input = "abcdeabcdeabcdeabcdeabcdec";
        assert!(validate_bucket(input).is_ok());
        let input = "12345678";
        assert!(validate_bucket(input).is_ok());
        let input = "qwertyui";
        assert!(validate_bucket(input).is_ok());

        // 字符大于60
        let input = "abcdeabcdeabcdeabcdeabcdecabcdeabcdeabcdeabcdeabcdecabcdeabcd";
        assert!(validate_bucket(input).is_err());

        // 字符小于1
        let input = "";
        assert!(validate_bucket(input).is_err());

        // 有小写字母、数字和 - 之外的字符
        let input = "2a中sadsd";
        assert!(validate_bucket(input).is_err());

        // 有小写字母、数字和 - 之外的字符
        let input = "!@#$/%^&*";
        assert!(validate_bucket(input).is_err());
        // 有小写字母、数字和 - 之外的字符
        let input = "asGk22_Q";
        assert!(validate_bucket(input).is_err());
    }
}
