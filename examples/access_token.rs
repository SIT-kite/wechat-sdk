use wechat_sdk::wechat::GetAccessToken;

#[tokio::main]
async fn main() {
    let client = wechat_sdk::WeChatClientBuilder::new()
        .appid("no_such_appid")
        .secret("secret")
        .build();

    for _ in 1..=5 {
        let client = client.clone();

        tokio::spawn(async move {
            let result = client.get_access_token().await;
            match result {
                Ok(token) => println!("Get token successfully, token = {}", token.access_token),
                Err(e) => println!("{}", e),
            }
        });
    }

    println!("Wait 5 seconds to exit.");

    let duration = tokio::time::Duration::from_secs(5);
    tokio::time::sleep(duration).await;
}
