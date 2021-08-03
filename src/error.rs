use reqwest::Error as ReqError;
use serde::Serialize;
use serde_json::Error as JsonError;

pub type Result<T> = std::result::Result<T, WxApiError>;
pub type Error = WxApiError;

#[derive(Debug, Serialize, thiserror::Error)]
#[error("Wechat interface error {}: {}.", errcode, errmsg)]
pub struct WxApiError {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

// TODO: Add conversion from ReqError, JsonError, ... to WxClientError.
#[derive(Debug, Serialize)]
pub enum WxClientError {
    Api(WxApiError),
    Inner(Box<dyn std::error::Error>),
}

impl WxApiError {
    pub fn new(code: u16, msg: String) -> Self {
        WxApiError {
            code,
            msg: Some(msg),
        }
    }
}
