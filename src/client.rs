use crate::wechat::WxAccessToken;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct WeChatClient {
    /// Wechat mini-program appid.
    pub(crate) appid: String,
    /// Wechat mini-program secret key.
    pub(crate) secret: String,

    /// Reqwest HTTP client
    pub(crate) client: Client,

    /// Wechat access token
    pub(crate) token: Arc<RwLock<WxAccessToken>>,
}

#[derive(Default)]
pub struct WeChatClientBuilder {
    /// Wechat mini-program appid.
    appid: Option<String>,
    /// Wechat mini-program secret key.
    secret: Option<String>,
}

impl WeChatClientBuilder {
    pub fn new() -> Self {
        WeChatClientBuilder::default()
    }

    pub fn appid<T: ToString>(mut self, appid: T) -> Self {
        self.appid = Some(appid.to_string());
        self
    }

    pub fn secret<T: ToString>(mut self, secret: T) -> Self {
        self.secret = Some(secret.to_string());
        self
    }

    pub fn build(self) -> WeChatClient {
        WeChatClient {
            appid: self.appid.unwrap_or_else(|| {
                panic!("Appid is required in WeChatClientBuilder, please call appid method.")
            }),
            secret: self.secret.unwrap_or_else(|| {
                panic!("Secret is required in WeChatClientBuilder, please call secret method.")
            }),
            client: Client::new(),
            token: Arc::new(RwLock::new(WxAccessToken::default())),
        }
    }
}
