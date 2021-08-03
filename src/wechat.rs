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
        async fn $fn_name(param: &str) -> Result<$structure, WxClientError> {
            // create actix-web client for request.
            let client = Client::new();
            let url = format!("{}?{}", $addr, param);
            // return Err(ApiError::from(url));
            let response = client.get(url).send().await;

            match response {
                // Note: Sending successfully, not receiving.
                Ok(mut r) => {
                    // Wechat services always return HTTP 200, with errcode field when parameter error.
                    // Decode json string or give an empty json.
                    let body_string = r.text().await?;
                    let body_json: $structure = serde_json::from_slice(body_string.as_ref())?;
                    return Ok(body_json);
                }
                Err(e) => Err(ApiError::from(format!(
                    "While connecting to wechat services: {}",
                    e
                ))),
            }
        } // End of function.
    }; // End of pattern.
} // End of macro_rules.
