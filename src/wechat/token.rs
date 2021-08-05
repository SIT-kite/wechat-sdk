use async_trait::async_trait;

use crate::client::WeChatClient;
use crate::error::WxApiError;
use crate::make_parameter;

use super::*;

#[derive(Debug, serde::Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    expires_in: Option<i32>,
    errcode: Option<u16>,
    errmsg: Option<String>,
}

#[async_trait]
pub trait GetAccessToken {
    async fn get_access_token(&self) -> Result<WxAccessToken, WxClientError>;
}

crate::wx_function!(
    _get_access_token,
    AccessTokenResponse,
    "https://api.weixin.qq.com/cgi-bin/token"
);

#[async_trait]
impl GetAccessToken for WeChatClient {
    async fn get_access_token(&self) -> Result<WxAccessToken, WxClientError> {
        let resp: AccessTokenResponse = _get_access_token(
            make_parameter!(
                "appid" => &self.appid,
                "secret" => &self.secret,
                "grant_type" => "client_credential"
            )
                .as_str(),
        )
            .await?;

        match resp {
            AccessTokenResponse {
                access_token: Some(access_token),
                expires_in: Some(expires_in),
                ..
            } => Ok(WxAccessToken {
                access_token,
                expires_in,
            }),
            AccessTokenResponse {
                errcode: Some(errcode),
                errmsg: Some(errmsg),
                ..
            } => Err(WxClientError::Api(WxApiError::new(errcode, errmsg))),
            _ => Err(WxClientError::Api(WxApiError::new(0, "Unknown".to_string()))),
        }
    }
}
