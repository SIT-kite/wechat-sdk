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
    async fn get_access_token_anyway(&self) -> Result<WxAccessToken, WxClientError>;
    async fn get_access_token(&self) -> Result<WxAccessToken, WxClientError>;
    async fn refresh_access_token(&self) -> Result<(), WxClientError>;
}

crate::wx_function!(
    _get_access_token,
    AccessTokenResponse,
    "https://api.weixin.qq.com/cgi-bin/token"
);

#[async_trait]
impl GetAccessToken for WeChatClient {
    async fn get_access_token_anyway(&self) -> Result<WxAccessToken, WxClientError> {
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
                let now_millis = Utc::now().timestamp_millis();
                let expire_ts = now_millis + expires_in as i64 * 1000;

                Ok(WxAccessToken {
                    access_token,
                    expire_ts,
                })
            }
            AccessTokenResponse {
                errcode: Some(errcode),
                errmsg: Some(errmsg),
                ..
            } => Err(WxClientError::Api(WxApiError::new(errcode, errmsg))),
            _ => Err(WxClientError::Api(WxApiError::new(
                0,
                "Unexpected response from wechat server.".to_string(),
            ))),
        }
    }

    async fn get_access_token(&self) -> Result<WxAccessToken, WxClientError> {
        let token = self.token.read().await;

        if !token.is_expired() {
            Ok(WxAccessToken {
                access_token: String::from(&*token.access_token),
                expire_ts: token.expire_ts,
            })
        } else {
            self.refresh_access_token().await?;
            let token = self.token.read().await;
            Ok(token.clone())
        }
    }

    async fn refresh_access_token(&self) -> Result<(), WxClientError> {
        let new_token = self.get_access_token_anyway().await?;
        let mut old_token = self.token.write().await;
        *old_token = new_token;

        Ok(())
    }
}
