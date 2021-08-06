use chrono::prelude::*;
use reqwest::Client;

use crate::error::WxClientError;

pub use login::Login;
pub use token::GetAccessToken;

mod login;
mod token;

pub struct WxSession {
    pub session_key: String,
    pub openid: String,
}

#[derive(Default)]
pub struct WxAccessToken {
    pub access_token: String,
    pub expire_ts: i64,
}

impl WxAccessToken {
    pub fn is_expired(&self) -> bool {
        if self.expire_ts == 0 {
            return true;
        }
        let now = Utc::now();
        let now_millis = now.timestamp_millis();

        now_millis >= self.expire_ts
    }
}

#[macro_export]
macro_rules! make_parameter {
    // Concatenate web form parameters to a string.
    // Example:
    // make_parameter!("a" => "1", "b" => 2);
    // will returned "a=1&b=2&"
    ($($para: expr => $val: expr), *) => {{
        let mut url = String::new();
        $( url = url + $para + "=" + $val + "&"; )*

        url.clone()
    }}
}

#[macro_export]
macro_rules! wx_function {
    ($fn_name: ident, $structure: ident, $addr: expr) => {
        async fn $fn_name(client: &Client, param: &str) -> Result<$structure, WxClientError> {
            let url = format!("{}?{}", $addr, param);
            let response = client.get(url).send().await;

            match response {
                // Note: Sending successfully, not receiving.
                Ok(r) => {
                    // Wechat services always return HTTP 200, with errcode field when parameter error.
                    // Decode json string or give an empty json.
                    let body_string = r.text().await?;
                    let body_json: $structure = serde_json::from_slice(body_string.as_ref())?;
                    Ok(body_json)
                }
                Err(e) => Err(WxClientError::Inner(Box::from(e))),
            }
        } // End of function.
    }; // End of pattern.
} // End of macro_rules.
