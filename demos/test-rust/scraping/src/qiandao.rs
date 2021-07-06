use regex::Regex;
use reqwest::Url;
use serde_json::json;
use std::error::Error as StdError;

use crate::http::{HttpClient, HttpClientOpts, Method, PostType, Req};

struct PageReq {
    req: Req,
    auth_str: String,
    success_str: String,
    finish_str: String,
}
enum PageStatus {
    Success,
    Finish,
    Redirect(String),
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
    } else if finish_re.is_match(&content) {
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
    for page in site.pages {
        handle_req(&client, page).await;
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
            name: String::from("south-plus"),
            enable_proxy: true,
            pages: vec![PageReq {
                req: Req {
                    url: "https://www.south-plus.net/plugin.php?H_name-tasks.html".to_string(),
                    method: Method::Get,
                    headers: Some(json!({
                        "referer": "https://www.south-plus.net/",
                        // @TODO cookie
                    })),
                    body: None,
                    post_type: None,
                },
                auth_str: "您还不是论坛会员,请先登录论坛".to_string(),
                success_str: "选择你要进行的任务项目".to_string(),
                finish_str: "xxx finish".to_string(),
            }],
        };
        sign(south).await;
    }
}
