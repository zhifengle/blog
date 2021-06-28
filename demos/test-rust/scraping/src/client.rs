#![allow(unused_imports)]
#![allow(dead_code)]

use std::borrow::Borrow;

use reqwest::{header, Client};

pub struct FetchOption {
    pub method: String,
}

impl FetchOption {
    pub fn new() -> FetchOption {
        FetchOption {
            method: String::from("get"),
        }
    }
}

impl Default for FetchOption {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_client(enable_proxy: bool) -> Client {
    let proxy = reqwest::Proxy::all("http://127.0.0.1:10809").unwrap();
    // 自定义头 参考官方文档
    // 使用变量后，判断是否有代理时，会出现 move。暂时不知道解决办法
    /*
    let builder = reqwest::Client::builder().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36");
    if enable_proxy {
        builder.proxy(proxy);
    }
    return builder.build().unwrap();
    */
    if enable_proxy {
        reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36")
        .proxy(proxy)
        .build()
        .unwrap()
    } else {
        reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36")
        .build()
        .unwrap()
    }
}

#[tokio::main]
pub async fn fetch_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = get_client(true);
    let resp = client
        .get(url)
        .send()
        .await?
        .text()
        //.text_with_charset("utf-8")
        .await?;
    Ok(resp)
}
