use wechat_sdk::wechat::Login;

#[tokio::main]
async fn main() {
    let client = wechat_sdk::WeChatClientBuilder::new()
        .appid("no_such_appid")
        .secret("secret")
        .build();

    let result = client.code2session("no_such_code").await;
    match result {
        Ok(session) => println!("Get session successfully, openid = {}", session.openid),
        Err(e) => println!("{}", e),
    }
}
