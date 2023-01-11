mod kv;
mod sites;

use std::future::Future;

use ajax::Ajax;
use kv::{KvExpiration};
use once_cell::sync::OnceCell;
use serde_json::json;
use sites::{south_plus, twodfan};

static AJAX_INSTANCE: OnceCell<Ajax> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init();
    let _ajax = AJAX_INSTANCE.get_or_init(|| Ajax::new());

    let mut kv = KvExpiration::json_engine("qd-record.json".to_string(), "QD_".to_string());
    // kv.flush_expired();
    qiandao("2dfan", twodfan::check_in, &mut kv).await;
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
    let result = engine.get(name);
    if result.is_some() {
        engine.set_next_day(name, json!(1));
        log::info!("[{}] 已签到", name);
        return;
    }
    let res = check_in().await;
    if res.is_ok() {
        engine.set_next_day(name, json!(1));
    }
    log_checkin_res(name, res);
}

fn log_checkin_res(name: &str, res: anyhow::Result<()>) {
    if res.is_ok() {
        log::info!("[{}] 签到成功", name);
    } else {
        log::error!("[{}] {}", name, res.unwrap_err());
    }
}