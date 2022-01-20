use lazy_static::lazy_static;
use regex::Regex;
use validator::ValidationError;

pub fn validate_name(input: &str) -> Result<(), ValidationError> {
    if input.chars().count() < 4 {
        return Err(ValidationError::new("必须大于3个字符"));
    }
    if input.chars().count() > 25 {
        return Err(ValidationError::new("必须小于26个字符"));
    }
    match input.chars().next() {
        Some('a'..='z' | 'A'..='Z') => {}
        _ => {
            return Err(ValidationError::new("必须以字母开头"));
        }
    };
    lazy_static! {
        static ref RE: Regex = Regex::new("^[0-9a-zA-Z_]{4,25}$").unwrap();
    }
    if !RE.is_match(input) {
        return Err(ValidationError::new("只允许字母数字和_"));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::validate_name;

    #[test]
    fn test_validate_name() {
        let input = "abc";
        assert!(validate_name(input).is_err());
        let input = "abcdeabcdeabcdeabcdeabcdec";
        assert!(validate_name(input).is_err());
        let input = "2asad";
        assert!(validate_name(input).is_err());
        let input = "_hjkvb";
        assert!(validate_name(input).is_err());
        let input = "asd*g";
        assert!(validate_name(input).is_err());
        let input = "asGk22_";
        assert!(validate_name(input).is_ok());
    }
}
