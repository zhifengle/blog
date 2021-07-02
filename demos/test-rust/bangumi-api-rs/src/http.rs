use std::error::Error as StdError;

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
        // let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36";
        let ua: String = match opts.ua {
            None => "bangumi-api-rs/0.1".to_string(),
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
    #[tokio::main]
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
    #[tokio::main]
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
