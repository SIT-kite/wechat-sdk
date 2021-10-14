use chrono::prelude::*;
use reqwest::Client;
use crate::error::WxClientError;
use serde::{Deserialize, Serialize};

pub use login::Login;
pub use token::GetAccessToken;
pub use check::Check;

mod login;
mod token;
mod check;

pub struct WxSession {
    pub session_key: String,
    pub openid: String,
}

#[derive(Default, Clone)]
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

#[derive(thiserror::Error, Debug)]
pub enum CheckError {
    #[error("无该用户openid,用户需在近两小时访问过小程序")]
    NoUserOpenId = 410,
}

#[derive(Serialize,Debug,Deserialize)]
pub struct Detail {
    //策略类型
    pub strategy: String,
    //错误码，仅当该值为0时，该项结果有效
    pub errcode: i32,
    //建议，有risky、pass、review三种值
    pub suggest: Option<String>,
    //命中标签枚举值，100 正常；10001 广告；20001 时政；20002 色情；20003 辱骂；20006 违法犯罪；20008 欺诈；20012 低俗；20013 版权；21000 其他
    pub label: Option<i64>,
    //0-100，代表置信度，越高代表越有可能属于当前返回的标签（label）
    pub prob: Option<i32>,
    //命中的自定义关键词
    pub keyword: Option<String>,
}

#[derive(Serialize,Debug,Deserialize)]
pub struct MulResult {
    //建议，有risky、pass、review三种值
    pub suggest: String,
    //命中标签枚举值，100 正常；10001 广告；20001 时政；20002 色情；20003 辱骂；20006 违法犯罪；20008 欺诈；20012 低俗；20013 版权；21000 其他
    pub label: i64,
}

#[derive(Serialize,Debug,Deserialize)]
pub struct CheckResult {
    //错误码
    pub errcode: i32,
    //错误信息
    pub errmsg: String,
    //详细检测结果
    pub detail: Vec<Detail>,
    //综合结果
    pub result: MulResult,
    //唯一请求标识，标记单次请求
    pub trace_id: String,
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

#[macro_export]
macro_rules! make_body {
    ( $( $key:expr => $val: expr),* ) => {
        {
            let mut body = HashMap::<String,String>::new();
            $(
                body.insert($key, $val);
            )*
            body
        }
    };
}

#[macro_export]
macro_rules! wx_function_post {
    ($fn_name: ident, $structure: ident, $addr: expr) => {
        async fn $fn_name(client: &Client, param: &str, body:HashMap<String,String>)-> Result<$structure, WxClientError> {
            let url = format!("{}?{}", $addr, param);

            let response = client.post(url)
            .json(&body)
            .send()
            .await;

            match response {
                Ok(r) => {
                    let body_json = r.json::<$structure>().await?;
                    Ok(body_json)
                }
                Err(e) => Err(WxClientError::Inner(Box::from(e))),
            }
        } // End of function.
    }; // End of pattern.
} // End of macro_rules.