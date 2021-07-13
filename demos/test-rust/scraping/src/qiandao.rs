use regex::Regex;
use reqwest::Url;
use serde_json::{json, Map};
use std::{collections::HashMap, error::Error as StdError};

use crate::http::{HttpClient, HttpClientOpts, Method, PostType, Req};

#[derive(Clone)]
struct PageReq {
    req: Req,
    auth_str: String,
    success_str: String,
    finish_str: String,
    // 下一步操作的数据: url,  body
    redirect_dict: Option<serde_json::Value>,
}
enum PageStatus {
    Success,
    Finish,
    Redirect(Option<serde_json::Value>),
    Unauthorized,
    NotFound,
}
struct Site {
    name: String,
    // href: String,
    enable_proxy: bool,
    pages: Vec<PageReq>,
}
pub struct SignError {
    error: String,
    error_desc: String,
}

fn gen_url(href: &str, pathname: &str) -> String {
    let url = Url::parse(href).unwrap();
    url.join(pathname).unwrap().to_string()
}
fn get_match_str(contents: &str, target_str: &str) -> Option<String> {
    let re = Regex::new(target_str).unwrap();
    if let Some(cap) = re.captures(contents) {
        // 第二个捕获是目标值
        return Some(cap.get(1).unwrap().as_str().to_string());
    }
    None
}
// 正则匹配数据，返回 json
// body 数据为 object
fn gen_redirect_data(contents: &str, dict: serde_json::Value) -> serde_json::Value {
    let mut res = Map::new();
    let mut body_map: HashMap<String, String> = HashMap::new();
    for (key, dict_val) in dict.as_object().unwrap().iter() {
        if key.eq("body") {
            for (key, val) in dict_val.as_object().unwrap().iter() {
                let m_str = get_match_str(contents, val.as_str().unwrap());
                if let Some(t_str) = m_str {
                    body_map.insert(key.clone(), t_str);
                }
            }
            res.insert("body".to_string(), json!(body_map));
        } else if let Some(target_str) = dict_val.as_str() {
            println!("other: {} and str: {}", key, target_str);
            if let Some(val) = get_match_str(contents, target_str) {
                res.insert(key.clone(), json!(val));
            }
        }
    }
    json!(res)
}
async fn handle_req(client: &HttpClient, page: PageReq) -> PageStatus {
    let req = page.req;
    let content = client.fetch_info(req).await.unwrap();
    let auth_re = Regex::new(&page.auth_str).unwrap();
    let success_re = Regex::new(&page.success_str).unwrap();
    let finish_re = Regex::new(&page.finish_str).unwrap();
    if auth_re.is_match(&content) {
        // return Err(StdError::new());
        return PageStatus::Unauthorized;
    } else if success_re.is_match(&content) {
        if let Some(dict) = page.redirect_dict {
            return PageStatus::Redirect(Some(gen_redirect_data(&content, dict)));
        }
        return PageStatus::Redirect(None);
    } else if finish_re.is_match(&content) {
        return PageStatus::Finish;
    }
    PageStatus::NotFound
}
async fn sign(site: Site) -> Result<PageStatus, Box<dyn StdError>> {
    let proxy_url = if site.enable_proxy {
        Some(String::from("http://127.0.0.1:10809"))
    } else {
        None
    };
    let client = HttpClient::new(HttpClientOpts {
        ua: None,
        proxy_url,
    });
    let mut prev_payload: Option<serde_json::Value> = None;
    for page in site.pages.clone().iter_mut() {
        if let Some(val) = prev_payload.as_ref() {
            if let Some(url_value) = val.get("url") {
                page.req.url = url_value.as_str().unwrap().to_string();
            }
            if let Some(body_value) = val.get("body") {
                let obj = body_value.as_object().unwrap();
                match page.req.body.clone() {
                    None => page.req.body = Some(json!(obj)),
                    Some(body_val) => {
                        let mut new_body = Map::new();
                        for (key, origin_val) in body_val.as_object().unwrap().iter() {
                            if let Some(target_val) = obj.get(key) {
                                new_body.insert(key.clone(), target_val.clone());
                            } else {
                                new_body.insert(key.clone(), origin_val.clone());
                            }
                        }
                        page.req.body = Some(json!(new_body));
                    }
                }
            }
        }
        let status = handle_req(&client, page.clone()).await;
        match status {
            PageStatus::Success => {
                //
            }
            PageStatus::Unauthorized => {
                //
            }
            PageStatus::Finish => {
                //
            }
            PageStatus::Redirect(payload) => {
                prev_payload = payload;
            }
            PageStatus::NotFound => {
                //
            }
        }
    }
    Ok(PageStatus::NotFound)
}

async fn run() -> Result<(), Box<dyn StdError>> {
    let zodgame = Site {
        name: String::from("zodgame"),
        enable_proxy: true,
        pages: vec![
            PageReq {
                req: Req {
                    url: "https://zodgame.xyz/plugin.php?id=dsu_paulsign:sign".to_string(),
                    method: Method::Get,
                    headers: Some(json!({
                        "referer": "https://zodgame.xyz/",
                    })),
                    body: None,
                    post_type: None,
                },
                auth_str: "您好！登录后享受更多精彩".to_string(),
                success_str: "选择你要进行的任务项目".to_string(),
                finish_str: "您今天已经签到过了或者签到时间还未开始".to_string(),
                redirect_dict: None,
            },
            PageReq {
                req: Req {
                    url: "https://zodgame.xyz/plugin.php?id=dsu_paulsign:sign&operation=qiandao&infloat=1&inajax=1".to_string(),
                    method: Method::Post,
                    headers: Some(json!({
                        "referer": "https://zodgame.xyz/plugin.php?id=dsu_paulsign:sign",
                    })),
                    body: Some(json!({
                        "qdxq": "wl",
                        // *object.get_mut("formhash").unwrap() = json!("ssss");
                        "formhash": "",
                    })),
                    post_type: Some(PostType::Form),
                },
                auth_str: "您好！登录后享受更多精彩".to_string(),
                success_str: "恭喜你签到成功".to_string(),
                finish_str: "xxx finish".to_string(),
                redirect_dict: None,
            },
        ],
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::http::Method;

    use super::*;

    #[test]
    fn test_url() {
        let u = gen_url("https://bgm.tv/rakuen", "/aaa");
        assert_eq!("https://bgm.tv/aaa".to_string(), u);
    }
    #[test]
    fn test_json() {
        let mut object = json!({ "A": 65, "B": 66, "C": null });
        *object.get_mut("C").unwrap() = json!(69);
        println!("{}", object);
    }
    #[test]
    fn test_req_print() {
        let req = Req {
            method: Method::Get,
            url: "aaa".to_string(),
            post_type: None,
            body: Some(json!({
                "name": "John",
            })),
            headers: None,
        };
        let j = serde_json::to_string(&req).unwrap();
        println!("{}", j);
    }
    #[tokio::main]
    #[test]
    async fn test_req() {
        let south = Site {
            name: String::from("v2ex"),
            enable_proxy: true,
            pages: vec![PageReq {
                req: Req {
                    url: "https://v2ex.com/mission/daily".to_string(),
                    method: Method::Get,
                    headers: Some(json!({
                        "referer": "https://v2ex.com/",
                        // @TODO cookie
                    })),
                    body: None,
                    post_type: None,
                },
                auth_str: "你是机器人么".to_string(),
                success_str: "选择你要进行的任务项目".to_string(),
                finish_str: "xxx finish".to_string(),
                redirect_dict: Some(json!({
                    "url": r"mission/daily/redeem?once=\d+"
                })),
            }],
        };
        sign(south).await;
    }
    #[test]
    fn test_regex() {
        let contents = r#"
        mission/daily/redeem?once=12312312
        aa=123
        bb=456
        cc=xyz
        "#;
        let target_str = r"mission/daily/redeem\?once=\d+";
        let re = Regex::new(target_str).unwrap();
        assert_eq!(re.is_match(contents), true);
    }
    #[test]
    fn test_gen_redirect_data() {
        let contents = r#"
        mission/daily/redeem?once=12312312
        aa=123
        bb=456
        cc=xyz
        "#;
        let dict = json!({
            "url": r"(mission/daily/redeem\?once=\d+)",
            "body": {
                "aa": "aa=(.+)",
                "cc": "cc=(.+)",
            }
        });
        let val = gen_redirect_data(contents, dict);
        assert_eq!(
            val,
            json!({
                "url": "mission/daily/redeem?once=12312312",
                "body": {
                    "aa": "123",
                    "cc": "xyz"
                }
            })
        );
        let dict = json!({});
        let val = gen_redirect_data(contents, dict);
        assert_eq!(val, json!({}));
    }
}
