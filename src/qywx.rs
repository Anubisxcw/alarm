use crate::config::AppConfig;
use reqwest::{Client, Proxy};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

/// 配置代理
pub fn setup_proxies() -> HashMap<&'static str, String> {
    HashMap::from([
        ("http", "http://proxy-idc2.intra.yiducloud.cn:80".to_string()),
        ("https", "http://proxy-idc2.intra.yiducloud.cn:80".to_string()),
    ])
}

/// 获取微信访问令牌的工具函数，支持代理
pub async fn get_wechat_token_with_proxy(config: &AppConfig) -> Result<String, Box<dyn Error>> {
    tracing::info!("Getting WeChat access token with proxy...");
    let corpid = config.get_corpid();
    let corpsecret = config.get_corpsecret();
    let url_wx_root = "https://qyapi.weixin.qq.com";
    let url_wx_token = format!(
        "{}/cgi-bin/gettoken?corpid={}&corpsecret={}",
        url_wx_root, corpid, corpsecret
    );

    // 配置代理
    let proxies = setup_proxies();
    let http_proxy = proxies.get("http").unwrap();
    let https_proxy = proxies.get("https").unwrap();

    // let client = Client::builder()
    //     .proxy(Proxy::http(http_proxy.clone())?)
    //     .proxy(Proxy::https(https_proxy.clone())?)
    //     .build()?;

    let client = Client::builder()
        .build()?;

    // 发出 GET 请求
    let response = client.get(&url_wx_token).send().await?;

    // 定义 TokenResponse 结构体
    #[derive(Deserialize)]
    struct TokenResponse {
        errcode: i32,
        errmsg: String,
        access_token: Option<String>,
    }

    // 解析 JSON 响应
    let token_response: TokenResponse = response.json().await?;

    // 检查返回结果
    if token_response.errcode == 0 {
        if let Some(access_token) = token_response.access_token {
            tracing::info!("Access token: {}", access_token);
            Ok(access_token)
        } else {
            tracing::error!("No access token in response");
            Err("No access token in response".into())
        }
    } else {
        Err(format!("Error {}: {}", token_response.errcode, token_response.errmsg).into())
    }
}