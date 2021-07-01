use std::error::Error as StdError;

pub struct HttpClient {
    client: reqwest::Client,
}
impl HttpClient {
    pub fn new() -> Self {
        let proxy_url = "http://127.0.0.1:10809";
        let proxy = reqwest::Proxy::all(proxy_url).unwrap();
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36";
        Self {
            client: reqwest::Client::builder()
                .user_agent(ua)
                .proxy(proxy)
                .build()
                .unwrap(),
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
}
