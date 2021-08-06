use reqwest::Client;
use chrono::prelude::*;

use crate::error::WxClientError;

pub use login::Login;
pub use token::GetAccessToken;

mod login;
mod token;

pub struct WxSession {
    pub session_key: String,
    pub openid: String,
}

pub struct WxAccessToken {
    pub access_token: String,
    pub expires_in: i32,
}

#[derive(Default)]
pub struct AccessToken {
    pub access_token: String,
    pub time: i64,
}

impl AccessToken {
    pub fn store_token<T: ToString>(&mut self, token: &T, expires_in: i64){
        self.access_token = token.to_string();
        let now= Utc::now();
        let now_millis = now.timestamp_millis();
        let next_update_time = now_millis + expires_in * 1000;
        self.time = next_update_time;
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
