use async_trait::async_trait;

use crate::client::WeChatClient;
use crate::error::WxApiError;
use crate::make_parameter;

use super::*;

#[derive(Debug, serde::Deserialize)]
struct SessionResponse {
    // When error occurred
    pub errcode: Option<u16>,
    pub errmsg: Option<String>,
    // Successful.
    pub session_key: Option<String>,
    pub openid: Option<String>,
    // TODO: support union id in wechat.
    // unionid: Option<String>,
}

#[async_trait]
pub trait Login {
    async fn code2session(&self, wechat_code: &str) -> Result<WxSession, WxClientError>;
}

crate::wx_function!(
    _get_session_key,
    SessionResponse,
    "https://api.weixin.qq.com/sns/jscode2session"
);

#[async_trait]
impl Login for WeChatClient {
    async fn code2session(&self, wechat_code: &str) -> Result<WxSession, WxClientError> {
        let resp: SessionResponse = _get_session_key(
            &self.client,
            make_parameter!(
                "appid" => &self.appid,
                "secret" => &self.secret,
                "js_code" => wechat_code,
                "grant_type" => "authorization_code"
            )
                .as_str(),
        )
            .await?;

        // TODO:
        // 每个函数中的这段 match 代码可以放到 wx_function 宏里面去提前处理错误
        // 但是考虑到需要处理所有 Response 的字段，以后可以精简下这块代码
        match resp {
            SessionResponse {
                session_key: Some(session_key),
                openid: Some(openid),
                ..
            } => return Ok(WxSession { session_key, openid }),
            SessionResponse {
                errcode: Some(errcode),
                errmsg: Some(errmsg),
                ..
            } => Err(WxClientError::Api(WxApiError::new(errcode, errmsg))),
            _ => Err(WxClientError::Api(WxApiError::new(0, "Unknown".to_string()))),
        }
    }
}
