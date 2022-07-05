use reqwest::header;
use serde::{Deserialize, Serialize};
use std::{error::Error as StdError, str::FromStr};

#[derive(Serialize, Deserialize)]
pub enum Method {
    #[serde(rename(serialize = "get", deserialize = "get"))]
    Get,
    #[serde(rename(serialize = "post", deserialize = "post"))]
    Post,
}
#[derive(Serialize, Deserialize)]
pub enum PostType {
    #[serde(rename(serialize = "json", deserialize = "json"))]
    Json,
    #[serde(rename(serialize = "form", deserialize = "form"))]
    Form,
}

#[derive(Serialize, Deserialize)]
pub struct Req {
    pub url: String,
    pub method: Method,
    pub post_type: Option<PostType>,
    pub body: Option<serde_json::Value>,
    pub headers: Option<serde_json::Value>,
}
pub struct HttpClient {
    pub client: reqwest::Client,
}
pub struct HttpClientOpts {
    pub ua: Option<String>,
    pub proxy_url: Option<String>,
}
impl HttpClient {
    pub fn new(opts: HttpClientOpts) -> Self {
        // let proxy_url = "http://127.0.0.1:10809";
        let chrome_ua = String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36");
        let ua: String = match opts.ua {
            None => chrome_ua,
            Some(ua) => ua,
        };
        if let Some(proxy_url) = opts.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).unwrap();
            Self {
                client: reqwest::Client::builder()
                    .user_agent(ua)
                    .proxy(proxy)
                    .build()
                    .unwrap(),
            }
        } else {
            Self {
                client: reqwest::Client::builder().user_agent(ua).build().unwrap(),
            }
        }
    }
    pub async fn fetch_text(&self, url: &str) -> Result<String, Box<dyn StdError>> {
        Ok(self
            .client
            .get(url)
            .send()
            .await?
            .text()
            //.text_with_charset("utf-8")
            .await?)
    }
    pub async fn fetch_info(&self, req: Req) -> Result<String, Box<dyn StdError>> {
        let headers = req.headers.map_or(header::HeaderMap::new(), |v| {
            let mut headers = header::HeaderMap::new();
            let obj = v.as_object().unwrap();
            for (key, val) in obj {
                let str = val.as_str().unwrap();
                headers.insert(
                    header::HeaderName::from_str(key).unwrap(),
                    // 这里使用 from_static 报错 2021-07-06
                    header::HeaderValue::from_str(str).unwrap(),
                );
            }
            headers
        });
        let res = match req.method {
            Method::Get => {
                self.client
                    .get(req.url)
                    .headers(headers)
                    .send()
                    .await?
                    .text()
                    .await?
            }
            Method::Post => {
                let builder = self.client.post(req.url).headers(headers);
                match req.post_type {
                    None => builder.send().await?.text().await?,
                    Some(PostType::Form) => {
                        builder
                            .form(req.body.unwrap().as_object().unwrap())
                            .send()
                            .await?
                            .text()
                            .await?
                    }
                    Some(PostType::Json) => {
                        builder
                            .json(req.body.unwrap().as_object().unwrap())
                            .send()
                            .await?
                            .text()
                            .await?
                    }
                }
            }
        };
        Ok(res)
    }
    pub async fn fetch_json(&self, url: &str) -> Result<serde_json::Value, Box<dyn StdError>> {
        Ok(self
            .client
            .get(url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_req() {
        let req = Req {
            url: "https://v2ex.com/mission/daily".to_string(),
            method: Method::Get,
            headers: Some(json!({
                "referer": "https://v2ex.com/",
                // @TODO cookie
            })),
            body: None,
            post_type: None,
        };
        println!("{}", serde_json::to_string(&req).unwrap());
    }
    #[tokio::main]
    #[test]
    async fn t_fetch_text() -> Result<(), Box<dyn StdError>> {
        // gbk 编码能够识别. 不像 nodejs 需要转码
        let url = "https://bbs.kafan.cn/";
        let client = HttpClient::new(HttpClientOpts {
            ua: None,
            proxy_url: None,
        });
        let content = client.fetch_text(url).await?;
        println!("{}", content);
        Ok(())
    }
}
