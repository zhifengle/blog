mod json_engine;

use chrono::{prelude::*, Duration};
pub use json_engine::JsonEngine;
use serde_json::{json, Value};

pub trait KvEngine {
    fn set(&mut self, key: &str, value: Value) -> bool;
    fn get(&self, key: &str) -> Option<Value>;
    fn remove(&mut self, key: &str);
    fn keys(&self) -> Vec<String>;
}

pub struct KvExpiration {
    engine: Box<dyn KvEngine>,
    prefix: String,
    suffix: String,
    bucket: String,
}
impl KvExpiration {
    pub fn new(engine: Box<dyn KvEngine>, prefix: String) -> Self {
        Self {
            engine,
            prefix,
            suffix: "-expiration".to_string(),
            bucket: "".to_string(),
        }
    }
    pub fn json_engine(filename: String, prefix: String) -> Self {
        let engine = Box::new(JsonEngine::new(filename));
        KvExpiration::new(engine, prefix)
    }
    fn gen_expiration_key(&self, key: &str) -> String {
        format!("{}{}{}{}", &self.prefix, &self.bucket, key, &self.suffix)
    }
    fn gen_key(&self, key: &str) -> String {
        format!("{}{}{}", &self.prefix, &self.bucket, key)
    }
    pub fn flush_expired(&mut self) {
        let pre = format!("{}{}", &self.prefix, &self.bucket);
        // 使用 Vec<&String> 报错
        for key in self.engine.keys() {
            if key.starts_with(&pre) && !key.ends_with(&self.suffix) {
                let target_key = key.replace(&pre, "");
                self.flush_expired_item(&target_key);
            }
        }
    }
    fn flush_expired_item(&mut self, key: &str) -> bool {
        if self.is_expired(key) {
            self.remove(key);
            return true;
        }
        false
    }
    fn is_expired(&self, key: &str) -> bool {
        let expr_key = self.gen_expiration_key(key);
        let time = self.engine.get(&expr_key);
        if time.is_none() {
            return false;
        }
        let time = time.unwrap();
        let time = time.as_str().unwrap();
        let record_time = time.parse::<DateTime<Utc>>().unwrap();
        let now: DateTime<Utc> = Utc::now();
        now >= record_time
    }
    pub fn set_next_day(&mut self, key: &str, value: Value) -> bool {
        let now: DateTime<Utc> = Utc::now();
        let next_day = Utc.with_ymd_and_hms(now.year(), now.month(), now.day() + 1, 1, 0, 0).unwrap();
        self.set(key, value, Some(next_day - now))
    }
    pub fn set_expiration_days(&mut self, key: &str, value: Value, days: i64) -> bool {
        self.set(key, value, Some(Duration::days(days)))
    }
    pub fn set(&mut self, key: &str, value: Value, duration: Option<Duration>) -> bool {
        self.engine.set(&self.gen_key(key), value);
        if duration.is_some() {
            let d = duration.unwrap();
            let time: DateTime<Utc> = Utc::now() + d;
            self.engine
                .set(&self.gen_expiration_key(key), json!(time.to_string()));
        }
        true
    }
    // @TODO get 应该是不能修改的; 合适的时机清理过期的 key
    pub fn get(&self, key: &str) -> Option<Value> {
        if self.is_expired(key) {
            return None;
        }
        self.engine.get(&self.gen_key(key))
    }
    pub fn remove(&mut self, key: &str) {
        self.engine.remove(&self.gen_key(key));
        self.engine.remove(&self.gen_expiration_key(key));
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use std::{thread, time};

    #[test]
    fn t_get() {
        let mut s = JsonEngine::new("test-drop.json");
        s.set("11", json!("sfs"));
    }
    #[test]
    fn t_kv() {
        let engine = Box::new(JsonEngine::new("t.json"));
        let mut kv = KvExpiration::new(engine, "MY_PREFIX_".to_string());
        kv.set("foo", json!(22), Some(Duration::days(1)));
        let v = kv.get("foo");
        assert_eq!(Some(json!(22)), v);
        thread::sleep(time::Duration::from_millis(1100));
        let v = kv.get("foo");
        assert_eq!(None, v);
        kv.flush_expired();
    }
    #[test]
    fn t_kv_panic() {
        let engine = Box::new(JsonEngine::new("t-panic.json"));
        let mut kv = KvExpiration::new(engine, "MY_PREFIX_".to_string());
        kv.set("foo", json!(99), Some(Duration::days(1)));
        // let v = kv.get("foo");
        // assert_eq!(Some(json!(22)), v);
        thread::sleep(time::Duration::from_secs(60));
        panic!("error");
    }
    #[test]
    fn t_time() {
        let now: DateTime<Utc> = Utc::now();
        let s = now.to_string();
        // let s2 = now.to_rfc2822();
        // let s3 = now.to_rfc3339();
        let s4 = s.parse::<DateTime<Utc>>().unwrap();
        assert_eq!(now, s4);
    }
}
