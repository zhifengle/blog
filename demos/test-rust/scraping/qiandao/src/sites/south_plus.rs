use anyhow::anyhow;

use super::fetch_text;

async fn checkin_south_task(task_id: u8) -> anyhow::Result<()> {
    let url_str = format!(
        "https://www.south-plus.net/plugin.php?H_name=tasks&action=ajax&actions=job&cid={}",
        task_id
    );
    let apply_url = format!(
        "https://www.south-plus.net/plugin.php?H_name=tasks&action=ajax&actions=job2&cid={}",
        task_id
    );
    let res = fetch_text(&url_str).await?;
    if res.contains("登录") {
        return Err(anyhow!("south 需要登录"));
    }
    if res.contains("success") {
        fetch_text(&apply_url).await?;
        return Ok(());
    } else if res.contains("拒离上次申请") {
        return Ok(());
    }
    Ok(())
}

pub async fn bonus() {
    // 每周
    let res = checkin_south_task(14).await;
    // 每日
    let res = checkin_south_task(15).await;
}