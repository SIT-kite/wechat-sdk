use wechat_sdk::wechat::Login;
use wechat_sdk::WxClientError;

#[tokio::main]
async fn main() {
    let client = wechat_sdk::WeChatClientBuilder::new()
        .appid("no_such_appid")
        .secret("secret")
        .build();

    let result = client.code2session("no_such_code").await;
    match result {
        Ok(session) => println!("Get session successfully, openid = {}", session.openid),
        Err(e) => match e {
            WxClientError::Api(api_err) => println!(
                "Failed to get session, error code is {}, and reason: {:?}",
                api_err.code, api_err.msg
            ),
            WxClientError::Inner(other_err) => println!("Client error: {:?}", other_err.to_string()),
        },
    }
}
