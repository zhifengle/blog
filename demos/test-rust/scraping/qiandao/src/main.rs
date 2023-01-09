mod sites;

use std::fs;

use ajax::{Ajax, Method};
use once_cell::sync::{Lazy, OnceCell};
use sites::{discuz_apply_task, dsu_paulsign };

static AJAX_INSTANCE: OnceCell<Ajax> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init();
    let _ajax = AJAX_INSTANCE.get_or_init(|| Ajax::new());

    // discuz_apply_task("https://bbs.acgrip.com/").await?;

    // let e = anyhow::anyhow!("some err");
    // log::error!("{}", e);
    Ok(())
}

fn init() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}
