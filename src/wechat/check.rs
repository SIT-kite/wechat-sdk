use async_trait::async_trait;

use crate::wechat::CheckResult;
use crate::WeChatClient;
use std::collections::HashMap;
use crate::make_body;
use crate::make_parameter;

use super::*;

//创建msg_sec_check函数
crate::wx_function_post!(
    msg_sec_check,
    CheckResult,
    "https://api.weixin.qq.com/wxa/msg_sec_check"
);

#[async_trait]
pub trait Check {
    //openid: 用户的openid（用户需在近两小时访问过小程序）
    //scene: 场景枚举值（1 资料；2 评论；3 论坛；4 社交日志）
    //content: 需检测的文本内容，文本字数的上限为2500字，需使用UTF-8编码
    async fn msg_sec_check(&self, openid:String, scene:String, content:String)-> Result<CheckResult, WxClientError>;
}

#[async_trait]
impl Check for WeChatClient{
    async fn msg_sec_check(&self, openid: String, scene: String, content: String) -> Result<CheckResult, WxClientError> {
        //获取微信接口调用凭证
        let token = &self.get_access_token().await.unwrap().access_token.clone();

        let response = msg_sec_check(
            &self.client,
            make_parameter!(
                "access_token" => token
            ).as_str(),
            make_body!(
                "openid".to_string() => openid.to_string(),
                "scene".to_string() => scene.to_string(),
                "version".to_string() => "2".to_string(),
                "content".to_string() => content
            )
        ).await?;

        Ok(response)
    }
}