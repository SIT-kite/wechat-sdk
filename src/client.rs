#[derive(Debug, serde::Deserialize)]
pub struct WeChatClient {
    pub appid: String,
    pub secret: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct WeChatClientBuilder {
    pub appid: String,
    pub secret: String,
}

impl WeChatClientBuilder {
    pub fn new() -> Self {
        WeChatClientBuilder {
            appid: String::from(""),
            secret: String::from(""),
        }
    }
    pub fn appid(mut self, appid: String) -> WeChatClientBuilder {
        self.appid = appid;
        self
    }
    pub fn secret(mut self, secret: &str) -> WeChatClientBuilder {
        self.secret = String::from(secret);
        self
    }
    pub fn build(self: Self) -> WeChatClient {
        WeChatClient {
            appid: self.appid,
            secret: self.secret,
        }
    }
}
