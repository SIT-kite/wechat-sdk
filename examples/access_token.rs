use wechat_sdk::wechat::GetAccessToken;
use wechat_sdk::WxClientError;

#[tokio::main]
async fn main() {
    let mut client = wechat_sdk::WeChatClientBuilder::new()
        .appid("no_such_appid")
        .secret("secret")
        .build();

    let result = client.get_access_token().await;
    match result {
        Ok(token) => println!("Get token successfully, token = {}", token.access_token),
        Err(e) => match e {
            WxClientError::Api(api_err) => println!(
                "Failed to get token, error code is {}, and reason is {:?}",
                api_err.code, api_err.msg
            ),
            WxClientError::Inner(other_err) => println!("Client error: {:?}", other_err.to_string()),
        },
    }
}
