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

## reqwest cookies
[pfernie/reqwest_cookie_store](https://github.com/pfernie/reqwest_cookie_store)

有的站点响应里面有 Set-Cookie
- 使用 cookie_store

------------------------------------------

## south-plus
ua 变化会丢失登录状态