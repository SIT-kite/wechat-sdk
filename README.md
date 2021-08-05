# 微信小程序服务端 sdk

## 摘要

本项目为微信小程序服务端提供接口支持。

## 接口功能

1. 登入
2. 接口调用凭证

## 环境配置

请先确保系统中已预装有 rust 编程环境（rustc、cargo等），并已连接上互联网。

## 例子

创建服务端
```rust
fn build_client() {
    let client = WeChatClientBuilder::new()
        .appid("xxxx")
        .secret("xxx")
        .build();
}
```

登入接口
```rust
fn login() {
    let session = client.code2session("code...").await?;
}
```

调用凭证
```rust
fn token() {
    let token = client.get_access_token().await?;
}
```

## 如何贡献

非常欢迎你的加入！[提一个 Issue](https://github.com/SIT-Yiban/wechat-sdk/issues/new) 或者提交一个 Pull Request。

如果您有意见或建议，可以联系我们。

