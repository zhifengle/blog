use lazy_static::lazy_static;
use serde_json::Value;
use std::{error::Error as StdError, fs::OpenOptions, io::Read};

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

fn read_config(filename: &str) -> Result<serde_json::Value, Box<dyn StdError>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.is_empty() {
        contents = "{}".to_string();
    }
    // let contents = fs::read_to_string(filename)?;
    let v: serde_json::Value = serde_json::from_str(&contents)?;
    Ok(v)
}

async fn qiandao_by_host(host: &str) -> Result<(), Box<dyn StdError>> {
    let current_dir = std::env::current_dir().unwrap();
    let p = current_dir.join("qd-config.json");
    let config = read_config(p.to_str().unwrap()).unwrap();
    match config.get(host) {
        Some(site) => {
            let req_arr = site.get("reqs").unwrap().as_array().unwrap();
            for req in req_arr.iter() {
                let mut req = req.clone();
                let headers = {
                    let default_headers = site.get("headers").unwrap().as_object().unwrap();
                    let req_headers = req.get("headers").unwrap();
                    let headers = match req_headers.clone() {
                        Value::Object(mut h) => {
                            h.extend(default_headers.clone());
                            h
                        }
                        Value::Null => default_headers.clone(),
                        _ => panic!("invalid headers"),
                    };
                    headers
                };
                let req_map = req.as_object_mut().unwrap();
                req_map.insert("headers".to_string(), serde_json::Value::Object(headers));
                let req =
                    serde_json::from_value(serde_json::Value::Object(req_map.clone())).unwrap();
                let content = HTTP_CLIENT.fetch_info(req).await?;
                if content.contains("注册[Register]") {
                    // return Err(Box::new("需要登录"));
                    println!("{}", "需要登录");
                    return Ok(());
                } else if content.contains("抱歉，本期您已申请过此任务") {
                    println!("{}", "已经申请");
                    return Ok(());
                }
            }
        }
        None => {
            println!("{}: {}", host, "配置不存在");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::main]
    #[test]
    async fn test_qd() {
        qiandao_by_host("www.52pojie").await;
    }

    #[test]
    fn test_read_config() {
        let current_dir = std::env::current_dir().unwrap();
        let p = current_dir.join("qd-config.json");
        let config = read_config(p.to_str().unwrap()).unwrap();
        println!("{:?}:", config);
    }
}
