use anyhow::Error;
use regex::Regex;

// 校验邮箱格式
pub(crate) fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[\w\.-]+@[a-zA-Z\d\.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

// 校验密码规则：长度至少8个字符，包含大写字母、小写字母、数字、特殊字符
pub(crate) fn validate_password(password: &str) -> Result<(), Error> {
    let length_check = password.len() >= 8;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special_char = password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;':\",./<>?".contains(c));

    if !length_check {
        return Err(Error::msg("密码长度至少为8个字符"));
    }
    if !has_uppercase {
        return Err(Error::msg("密码必须包含至少一个大写字母"));
    }
    if !has_lowercase {
        return Err(Error::msg("密码必须包含至少一个小写字母"));
    }
    if !has_digit {
        return Err(Error::msg("密码必须包含至少一个数字"));
    }
    if !has_special_char {
        return Err(Error::msg("密码必须包含至少一个特殊字符"));
    }

    Ok(())
}
