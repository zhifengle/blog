use lazy_static::lazy_static;

use crate::http::{HttpClient, HttpClientOpts};

lazy_static! {
    static ref HTTP_CLIENT: HttpClient = HttpClient::new(HttpClientOpts {
        ua: None,
        proxy_url: None,
    });
    static ref HTTP_CLIENT_PROXY: HttpClient = HttpClient::new(HttpClientOpts {
        ua: None,
        proxy_url: Some(String::from("http://127.0.0.1:10809")),
    });
}

pub enum ExitCode {
    NotLogin,
    Success,
}

pub struct Site {
    name: String,
    config: serde_json::Value,
    client: &'static HttpClient,
}

impl Site {
    pub fn new(name: String, config: &mut serde_json::Value) -> Self {
        let config = config[&name].take();
        match config.get("httpsAgent") {
            Some(_) => Self {
                name,
                config,
                client: &HTTP_CLIENT_PROXY,
            },
            None => Self {
                name,
                config,
                client: &HTTP_CLIENT,
            },
        }
    }
    pub async fn sign(&self) -> anyhow::Result<ExitCode> {
        Ok(ExitCode::Success)
    }
    pub async fn check(&self, url: &str, pattern: &str) -> anyhow::Result<bool> {
        let contents = self.client.fetch_text(url).await?;

        Ok(contents.contains(pattern))
    }
}

#[test]
fn t_site() {
    let filename = "node-site-config.json";
    let filename = dirs::home_dir().unwrap().join(filename);
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut v: serde_json::Value = serde_json::from_str(&contents).unwrap();

    let _s = Site::new("v2ex".to_string(), &mut v);
}
