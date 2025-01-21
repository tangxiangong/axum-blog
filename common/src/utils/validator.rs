use crate::{impl_into_bad_request_error, AppError, AppResult};
use axum::http::StatusCode;
use regex::Regex;
use std::sync::LazyLock;
use thiserror::Error;

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\w[-\w.+]*@([A-Za-z0-9][-A-Za-z0-9]+\.)+[A-Za-z]{2,14}$").unwrap()
});

fn is_all_alphanumeric_or_dash(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

/// 用户名长度 2-16
pub fn is_valid_username(name: &str) -> AppResult<()> {
    if !is_all_alphanumeric_or_dash(name) {
        return Err(ValidationError::InvalidUsername(
            "用户名只能包含字母、数字、下划线和短横线".to_string(),
        )
        .into());
    }
    if !(2..=16).contains(&name.len()) {
        return Err(
            ValidationError::InvalidUsername("用户名长度必须在 2 到 16 之间".to_string()).into(),
        );
    }
    Ok(())
}

/// 密码长度 6-32
pub fn is_valid_password(password: &str) -> AppResult<()> {
    if !is_all_alphanumeric_or_dash(password) {
        return Err(ValidationError::InvalidPassword(
            "密码只能包含字母、数字、下划线和短横线".to_string(),
        )
        .into());
    }
    if !(6..=32).contains(&password.len()) {
        return Err(
            ValidationError::InvalidPassword("密码长度必须在 6 到 32 位之间".to_string()).into(),
        );
    }
    Ok(())
}

/// 昵称长度 2-12
pub fn is_valid_nickname(name: &str) -> AppResult<()> {
    let invalid_chars = [
        '/', '(', ')', '"', '<', '>', '\\', '{', '}', '#', '%', '&', '*', '!', '@', '$',
    ];
    if name.chars().any(|c| invalid_chars.contains(&c)) {
        return Err(ValidationError::InvalidNickname(
            "昵称不能包含 /()\"<>\\{}#%&*!@$".to_string(),
        )
        .into());
    }
    if !(2..=12).contains(&name.len()) {
        return Err(
            ValidationError::InvalidNickname("昵称长度必须在 2 到 10 之间".to_string()).into(),
        );
    }
    Ok(())
}

pub fn is_valid_email(email: &str) -> AppResult<()> {
    if !EMAIL_REGEX.is_match(email) {
        return Err(ValidationError::InvalidEmail("邮箱格式错误".to_string()).into());
    }
    Ok(())
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("用户名非法: {0}")]
    InvalidUsername(String),
    #[error("昵称非法: {0}")]
    InvalidNickname(String),
    #[error("邮箱非法: {0}")]
    InvalidEmail(String),
    #[error("密码非法: {0}")]
    InvalidPassword(String),
}

impl_into_bad_request_error!(ValidationError);
