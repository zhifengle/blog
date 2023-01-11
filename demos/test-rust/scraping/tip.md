## 随机数字
```rust
// ref: fn dsu_paulsign
let mut rng = rand::thread_rng();
rng.gen_range(0..arr.len())
```

## regex
```rust
// 不需要像js那样转义 "/"
// ref: fn dsu_paulsign;
Regex::new(r#"<input\s*type="hidden"\s*name="formhash"\s*value="([^"]+?)"\s*/?>"#).unwrap()
```

## panic cleanup
https://stackoverflow.com/questions/43441047/whats-the-best-way-to-register-a-function-to-run-during-an-unexpected-exit-of-a

使用 ctrl+c 退出时，Drop 写入文件不再可靠。

[Detegr/rust-ctrlc](https://github.com/Detegr/rust-ctrlc)

## chrono
```rust
use chrono::{prelude::*, Duration};

let time: DateTime<Utc> = Utc::now() + d;
```

## reqwest cookies
[pfernie/reqwest_cookie_store](https://github.com/pfernie/reqwest_cookie_store)

有的站点响应里面有 Set-Cookie
- 使用 cookie_store

------------------------------------------

## south-plus
ua 变化会丢失登录状态