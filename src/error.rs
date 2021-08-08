use reqwest::Error as ReqError;
use serde::Serialize;
use serde_json::Error as JsonError;

#[derive(Debug, Serialize, thiserror::Error)]
#[error("Wechat API {}: {:?}.", code, msg)]
pub struct WxApiError {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum WxClientError {
    #[error("Wechat server returned an error: {0}")]
    Api(WxApiError),
    #[error("Error while requesting: {0}")]
    Inner(Box<dyn std::error::Error>),
}

#[macro_export]
macro_rules! convert_inner_errors {
    ($src_err_type: ident) => {
        impl From<$src_err_type> for WxClientError {
            fn from(sub_err: $src_err_type) -> Self {
                return Self::Inner(Box::from(sub_err));
            }
        }
    };
}

convert_inner_errors!(ReqError);
convert_inner_errors!(JsonError);
convert_inner_errors!(String);

impl WxApiError {
    pub fn new(code: i32, msg: String) -> Self {
        WxApiError { code, msg: Some(msg) }
    }
}
