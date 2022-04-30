use validator::ValidationError;

pub fn validate_folder(input: &str) -> Result<(), ValidationError> {
    let len = input.chars().count();
    if len > 255 {
        return Err(ValidationError::new("不能大于255字符"));
    }
    if len < 1 {
        return Err(ValidationError::new("不能小于1个字符"));
    }
    if input == "." || input == ".." {
        return Err(ValidationError::new("不能为\"..\" \".\""));
    }
    if input.chars().all(|x| x == ' ') {
        return Err(ValidationError::new("不能为空"));
    }
    if input
        .chars()
        .any(|x| x == '/' || x == '\n' || x == '\r' || x == '\t')
    {
        Err(ValidationError::new("不能含有/和换行符等特殊字符"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::validate_folder;
    #[test]
    fn test_validate_folder() {
        let input = " abc";
        assert!(validate_folder(input).is_ok());
        let input = "2a中sadsd ";
        assert!(validate_folder(input).is_ok());
        let input = "1234 5678";
        assert!(validate_folder(input).is_ok());
        let input = "asGk22_Q";
        assert!(validate_folder(input).is_ok());

        // 不能含有/和换行符等特殊字符
        let input = "!@#$/%^&*";
        assert!(validate_folder(input).is_err());
        // 不能为.
        let input = ".";
        // 不能为..
        assert!(validate_folder(input).is_err());
        let input = "..";
        assert!(validate_folder(input).is_err());
        // 不能为空
        let input = "   ";
        assert!(validate_folder(input).is_err());
        // 字符小于1
        let input = "";
        assert!(validate_folder(input).is_err());
        // 字符大于255
        let input = "abcdeabcdeabcdeabcdeabcdecabcdeab";
        let input = format!(
            "{}{}{}{}{}{}{}{}",
            input, input, input, input, input, input, input, input
        );
        assert!(validate_folder(input.as_str()).is_err());
    }
}
