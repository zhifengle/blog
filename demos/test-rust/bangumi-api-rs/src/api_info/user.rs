use lazy_static::lazy_static;
use serde_json::json;

lazy_static! {
    static ref DATA: serde_json::Value = json!({
        "user_info": {
            "url": "/user/{username}",
            "method": "GET",
            "verify": true,
            "path": {
                "username": "用户名"
            },
            "comment": "用户信息"
        },
        "user_collection": {
            "url": "/user/{username}/collection",
            "method": "GET",
            "verify": true,
            "path": {
                "username": "用户名"
            },
            "query": {
                "cat": "watching 或者 all_watching",
                "ids": "批量查询收藏状态，将条目 ID 以半角逗号分隔，如 1,2,4,6"
            },
            "comment": "用户收藏"
        }
    });
}
