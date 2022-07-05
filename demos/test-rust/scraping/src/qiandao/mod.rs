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
