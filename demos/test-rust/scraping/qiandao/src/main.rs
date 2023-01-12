mod kv;
mod sites;

use std::future::Future;

use ajax::Ajax;
use anyhow::Ok;
use kv::KvExpiration;
use once_cell::sync::OnceCell;
use serde_json::json;
use sites::{south_plus, twodfan};

static AJAX_INSTANCE: OnceCell<Ajax> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init();
    let _ajax = AJAX_INSTANCE.get_or_init(|| Ajax::new());

    let mut kv = KvExpiration::json_engine("qd-record.json".to_string(), "QD_".to_string());
    let sites = kv.get("sites");
    if sites.is_none() {
        kv.set("sites", json!([]), None);
        log::warn!("no sites");
        return Ok(());
    }
    for site in sites.unwrap().as_array().unwrap() {
        let site = site.as_str().unwrap();
        let err_key = format!("{}_err", site);
        if kv.get(&err_key) == Some(json!(1)) {
            log::error!("[{}] 需要登录", site);
            continue;
        }
        match site {
            "2dfan" => {
                qiandao(site, twodfan::check_in, &mut kv).await;
            }
            "south-plus" => {
                qiandao("south-plus", south_plus::bonus14, &mut kv).await;
                qiandao("south-plus", south_plus::bonus15, &mut kv).await;
            }
            _ => log::warn!("unknown site"),
        }
    }
    // kv.flush_expired();
    Ok(())
}

fn init() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}

pub async fn qiandao<F, Fut>(name: &str, check_in: F, engine: &mut KvExpiration)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let err_key = format!("{}_err", name);
    let is_err = engine.get(&err_key);
    if is_err.is_some() {
        log::error!("[{}] 出错了", name);
        return;
    }
    let result = engine.get(name);
    if result.is_some() {
        engine.set_next_day(name, json!(1));
        log::info!("[{}] 已签到", name);
        return;
    }
    let res = check_in().await;
    if res.is_ok() {
        engine.set_next_day(name, json!(1));
        log::info!("[{}] 签到成功", name);
    } else {
        let err_str = res.unwrap_err().to_string();
        if err_str.contains("需要登录") {
            engine.set(&err_key, json!(1), None);
        }
        log::error!("[{}] {}", name, err_str);
    }
}

fn is_err_site(engine: &KvExpiration, site: &str) -> bool {
    engine.get(site).is_some()
}
