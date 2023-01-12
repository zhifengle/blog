use anyhow::anyhow;

use super::fetch_text;

pub async fn checkin_south_task(task_id: u8) -> anyhow::Result<()> {
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
    if res.contains("未开始") {
        return Err(anyhow!("任务 {} 未开始", task_id));
    }
    if res.contains("success") {
        fetch_text(&apply_url).await?;
        return Ok(());
    } else if res.contains("拒离上次申请") {
        return Ok(());
    }
    Ok(())
}
pub async fn bonus() -> anyhow::Result<()> {
    let res = checkin_south_task(15).await;
    if res.is_err() {
        if res.unwrap_err().to_string().contains("需要登录") {
            return Err(anyhow!("south 需要登录"));
        }
    }
    checkin_south_task(14).await;
    checkin_south_task(19).await;
    Ok(())
}

pub async fn bonus14() -> anyhow::Result<()> {
    // 每周
    checkin_south_task(14).await
}

pub async fn bonus15() -> anyhow::Result<()> {
    // 每周
    checkin_south_task(14).await
}
