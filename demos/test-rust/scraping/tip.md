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
------------------------------------------

## south-plus
ua 变化会丢失登录状态