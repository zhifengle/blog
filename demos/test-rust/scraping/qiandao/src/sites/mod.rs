use std::time::Duration;

use ajax::{HeaderMap, Method};
use anyhow::anyhow;
use once_cell::sync::Lazy;
use rand::Rng;
use regex::Regex;
use serde_json::{json, Value};
use tokio::time::sleep;
use url::Url;

use crate::AJAX_INSTANCE;

pub mod south_plus;
pub mod twodfan;

// #[derive(Default)]
// struct Req {
//     url: String,
//     method: Method,
//     headers: Option<Value>,
//     body: Option<Value>,
// }

// #[derive(Default)]
// struct PageReq {
//     req: Req,
//     auth_str: String,
//     success_str: String,
//     finish_str: String,
//     // 下一步操作的数据: url,  body
//     redirect_dict: Option<Value>,
// }

async fn fetch_text(url_str: &str) -> anyhow::Result<String> {
    let ajax = AJAX_INSTANCE.get().unwrap();

    let res = ajax
        .gen_req(Method::GET, &url_str)
        .send()
        .await?
        .text()
        .await?;
    return Ok(res);
}
async fn post_form(url_str: &str, fd: &Value) -> anyhow::Result<String> {
    let ajax = AJAX_INSTANCE.get().unwrap();

    let res = ajax
        .gen_req(Method::POST, &url_str)
        .form(fd)
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

async fn post_data(url_str: &str, fd: &Value, headers: Option<HeaderMap>) -> anyhow::Result<Value> {
    let ajax = AJAX_INSTANCE.get().unwrap();
    let headers = headers.unwrap_or_default();

    let res = ajax
        .gen_req(Method::POST, &url_str)
        .headers(headers)
        .json(fd)
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

fn log_checkin_res(name: &str, res: anyhow::Result<()>) {
    if res.is_ok() {
        log::info!("[{}] 签到成功", name);
    } else {
        log::error!("[{}] {}", name, res.unwrap_err());
    }
}

pub async fn discuz_apply_task(home: &str) -> anyhow::Result<()> {
    let url_obj = Url::parse(home)?.join("home.php?mod=task&do=apply&id=1")?;
    let host = url_obj.host().unwrap().to_string();

    let res = fetch_text(url_obj.as_str()).await?;
    if res.contains("下期再") {
        log::info!("{} 已经完成", host);
        return Ok(());
    }
    if res.contains("本操作") {
        log::error!("{} 需要登录", host);
        return Err(anyhow!("{} 需要登录", host));
    }
    Ok(())
}
pub async fn dsu_paulsign(home: &str) -> anyhow::Result<()> {
    let url_obj = Url::parse(home)?.join("dsu_paulsign-sign.html")?;
    let host = url_obj.host().unwrap().to_string();

    let sign_page_url = url_obj.as_str();

    let content = fetch_text(sign_page_url).await?;
    if content.contains("您需要先登录才能继续本操作") {
        log::error!("{} 需要登录", host);
        return Err(anyhow!("{} 需要登录", host));
    }
    if content.contains("您今天已经签到过了或者签到时间还未开始") {
        return Ok(());
    }
    static FORM_HASH: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"<input\s*type="hidden"\s*name="formhash"\s*value="([^"]+?)"\s*/?>"#).unwrap()
    });
    static RE_FORM: Lazy<Regex> =
        Lazy::new(|| Regex::new(r#"<form\s*id="qiandao"\s*method="post""#).unwrap());
    let caps = FORM_HASH.captures(&content);
    // println!("{}", caps.unwrap().get(1).unwrap().as_str());
    if caps.is_some() && RE_FORM.is_match(&content) {
        // ss
        let arr = ["kx", "ym", "wl", "nu", "ch", "fd", "yl", "shuai"];
        let mut rng = rand::thread_rng();
        let fd = json!({
            "formhash": caps.unwrap().get(1).unwrap().as_str(),
            "qdxq": arr[rng.gen_range(0..arr.len())],
            "fastreply": 0,
            "qdmode": 3,
        });
        let url_obj = url_obj
            .join("/plugin.php?id=dsu_paulsign:sign&operation=qiandao&infloat=1&inajax=1")?;
        let res = post_form(url_obj.as_str(), &fd).await?;
        if res.contains("未定义操作") {
            log::error!("{} 未定义操作", host);
            return Err(anyhow!("{} 未定义操作", host));
        }
        if res.contains("恭喜你签到成功") {
            sleep(Duration::from_millis(rng.gen_range(200..400))).await;
            fetch_text(sign_page_url).await?;
            return Ok(());
        }
    }

    Ok(())
}

#[test]
fn t_join_url() {
    let home = "https://bbs.acgrip.com/";
    let url_obj = Url::parse(home)
        .unwrap()
        .join("dsu_paulsign-sign.html")
        .unwrap();

    let sign_page_url = url_obj.as_str();
    println!("{}", sign_page_url);
    let url_obj = url_obj
        .join("/plugin.php?id=dsu_paulsign:sign&operation=qiandao&infloat=1&inajax=1")
        .unwrap();
    let sign_page_url = url_obj.as_str();
    println!("{}", sign_page_url);
}
