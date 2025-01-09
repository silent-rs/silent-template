use serde::Serialize;
use silent::StatusCode;

/// 数据统一返回格式
#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: Option<u16>,
    pub data: Option<T>,
    pub msg: Option<String>,
}

/// 填入到extensions中的数据
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ResJsonString(pub String);

impl<T: Serialize> Res<T> {
    #[allow(dead_code)]
    pub fn with_data(data: T) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some("success".to_string()),
        }
    }
    pub fn with_err(err: &str) -> Self {
        Self {
            code: Some(500),
            data: None,
            msg: Some(err.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn with_err_code(code: u16, err: &str) -> Self {
        Self {
            code: Some(code),
            data: None,
            msg: Some(err.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn with_err_status(status: StatusCode, err: &str) -> Self {
        Self {
            code: Some(status.as_u16()),
            data: None,
            msg: Some(err.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn with_msg(msg: &str) -> Self {
        Self {
            code: Some(200),
            data: None,
            msg: Some(msg.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn with_data_msg(data: T, msg: &str) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some(msg.to_string()),
        }
    }
}
