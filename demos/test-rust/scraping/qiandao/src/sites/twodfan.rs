use ajax::HeaderMap;
use anyhow::anyhow;

use super::{fetch_text, post_data, log_checkin_res};

async fn check_in() -> anyhow::Result<()> {
    let content = fetch_text("https://galge.fun").await?;
    if content.contains("已连续签到") {
        return Ok(());
    }
    if content.contains("/users/not_authenticated") {
        return Err(anyhow!("2dfan 需要登录"));
    }
    let mut headers = HeaderMap::new();
    headers.append("X-Requested-With", "XMLHttpRequest".parse()?);
    let fd = serde_json::Value::Null;
    let res = post_data("https://galge.fun/checkins", &fd, Some(headers)).await?;
    if res["checkins_count"].is_null() {
        // log::error!("2dfan 签到失败");
        return Err(anyhow!("2dfan 签到接口失败"));
    }
    Ok(())
}

pub async fn bonus() {
    log_checkin_res("2dfan", check_in().await);
}
