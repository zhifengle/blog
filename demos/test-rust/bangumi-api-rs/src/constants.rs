use lazy_static::lazy_static;

use crate::http::{HttpClient, HttpClientOpts};

lazy_static! {
    pub static ref HTTP_CLIENT: HttpClient = HttpClient::new(HttpClientOpts {
        ua: None,
        proxy_url: None,
    });
}
