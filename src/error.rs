use std::io::Error as StdIoError;

use serde_json::Error as JsonError;
use reqwest::Error as ReqError;
use num_traits::ToPrimitive;
use serde::Serialize;
use crate::wechat::WxErr;

pub type Result<T> = std::result::Result<T, ApiError>;
pub type Error = ApiError;

#[derive(Debug, Serialize, PartialEq)]
pub struct ApiError {
    pub code: u16,
    // TODO: Add inner error handler and the uncomment following line.
    #[serde(skip_serializing)]
    pub inner_msg: Option<String>,
    #[serde(rename(serialize = "msg"), skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ServerError {{code: {}, msg: {}}} ",
            self.code,
            self.error_msg.as_ref().unwrap_or(&String::from(""))
        )
    }
}

impl ApiError {
    pub fn new<T: ToPrimitive + std::error::Error>(sub_err: T) -> Self {
        Self {
            code: sub_err.to_u16().unwrap(),
            inner_msg: None,
            error_msg: Some(sub_err.to_string()),
        }
    }
}

impl From<WxErr> for ApiError {
    fn from(e: WxErr) -> Self {
        ApiError {
            code: e.errcode,
            inner_msg: Some(e.errmsg),
            error_msg: None,
        }
    }
}

#[macro_export]
macro_rules! convert_inner_errors {
    ($src_err_type: ident) => {
        impl From<$src_err_type> for ApiError {
            fn from(sub_err: $src_err_type) -> Self {
                Self {
                    code: 1,
                    inner_msg: None,
                    error_msg: Some(sub_err.to_string()),
                }
            }
        }
    };
}

convert_inner_errors!(JsonError);
convert_inner_errors!(String);
convert_inner_errors!(ReqError);