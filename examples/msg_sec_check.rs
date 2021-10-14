use wechat_sdk::wechat::Check;

#[tokio::main]
async fn main() {
    let client = wechat_sdk::WeChatClientBuilder::new()
        .appid("no_such_appid")
        .secret("secret")
        .build();

    let response = client.msg_sec_check("no_such_openid".to_string(),"1".to_string(),"hello world".to_string()).await;

    println!("{:#?}",response);
}