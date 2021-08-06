use async_trait::async_trait;

use crate::client::WeChatClient;
use crate::error::WxApiError;
use crate::make_parameter;

use super::*;

#[derive(Debug, serde::Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    expires_in: Option<i32>,
    errcode: Option<i32>,
    errmsg: Option<String>,
}

#[async_trait]
pub trait GetAccessToken {
    async fn get_access_token_anyway(&mut self) -> Result<WxAccessToken, WxClientError>;
    async fn get_access_token(&mut self) -> Result<WxAccessToken,  WxClientError>;
}

crate::wx_function!(
    _get_access_token,
    AccessTokenResponse,
    "https://api.weixin.qq.com/cgi-bin/token"
);

#[async_trait]
impl GetAccessToken for WeChatClient {
    async fn get_access_token_anyway(&mut self) -> Result<WxAccessToken, WxClientError> {
        let resp: AccessTokenResponse = _get_access_token(
            &self.client,
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
            } => {
                self.token.store_token(&access_token, expires_in as i64);
                Ok(WxAccessToken {
                access_token,
                expires_in,
            })},
            AccessTokenResponse {
                errcode: Some(errcode),
                errmsg: Some(errmsg),
                ..
            } => Err(WxClientError::Api(WxApiError::new(errcode, errmsg))),
            _ => Err(WxClientError::Api(WxApiError::new(0, "Unknown".to_string()))),
        }
    }

    async fn get_access_token(&mut self) -> Result<WxAccessToken, WxClientError> {
        let now = Utc::now().timestamp_millis();
        if self.token.time < now  && self.token.time != 0{
            let token = String::from(&self.token.access_token);
            let time_spent = (Utc::now().timestamp_millis() - &self.token.time) / 1000;
            let time_left = 7200 - time_spent as i32;
            Ok(WxAccessToken{
                access_token: token,
                expires_in: time_left ,
            })
        }
        else {
            self.get_access_token_anyway().await
        }
    }
}
