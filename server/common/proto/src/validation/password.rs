use validator::ValidationError;

pub fn validate_password(input: &str) -> Result<(), ValidationError> {
    if input.chars().count() < 8 {
        return Err(ValidationError::new("必须大于7个字符"));
    }
    if input.chars().count() > 25 {
        return Err(ValidationError::new("必须小于26个字符"));
    }
    let mut num_exist = 0;
    let mut alpha_exist = 0;
    let mut other_exist = 0;
    for c in input.chars() {
        match c {
            'a'..='z' | 'A'..='Z' => alpha_exist = 1,
            '0'..='9' => num_exist = 1,
            '`' | '~' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '-' | '_'
            | '=' | '+' | '|' | '\\' | '[' | '{' | ']' | '}' | ';' | ':' | '\'' | '"' | ','
            | '<' | '.' | '>' | '/' | '?' => other_exist = 1,
            _ => return Err(ValidationError::new("不能含有其他字符")),
        };
    }
    if num_exist + alpha_exist + other_exist <= 1 {
        return Err(ValidationError::new("必须含有字母,数字,特殊字符其中两种"));
    }
    Ok(())
}
#[cfg(test)]
mod test {
    use super::validate_password;

    #[test]
    fn test_validate_password() {
        let input = "asGk22_Q";
        assert!(validate_password(input).is_ok());
        let input = "asGk22Qa";
        assert!(validate_password(input).is_ok());

        // 字符小于8
        let input = "abc";
        assert!(validate_password(input).is_err());
        // 字符大于25
        let input = "abcdeabcdeabcdeabcdeabcdec";
        assert!(validate_password(input).is_err());
        // 包括字母,数字,键盘上存在的特殊字符之外的字符
        let input = "2a中sadsd";
        assert!(validate_password(input).is_err());
        // 只含有字母,数字,特殊字符之一
        let input = "12345678";
        // 只含有字母,数字,特殊字符之一
        assert!(validate_password(input).is_err());
        let input = "qwertyui";
        // 只含有字母,数字,特殊字符之一
        assert!(validate_password(input).is_err());
        let input = "!@#$%^&*";
        assert!(validate_password(input).is_err());
    }
}
