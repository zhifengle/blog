mod json_engine;

use chrono::{prelude::*, Duration};
use serde_json::{json, Value};
pub use json_engine::JsonEngine;

pub trait KvEngine {
    fn set(&mut self, key: &str, value: Value) -> bool;
    fn get(&self, key: &str) -> Option<Value>;
    fn remove(&mut self, key: &str);
    fn keys(&self) -> Vec<String>;
}


pub enum TimeOpt {
    Day(u8),
    // DD h min sec; 毫秒就不要了
    Dhms(u8, u8, u8, u8),
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
    pub fn json_engine(filename: String ,prefix: String) -> Self {
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
    pub fn set(&mut self, key: &str, value: Value, opt: Option<TimeOpt>) -> bool {
        self.engine.set(&self.gen_key(key), value);
        if opt.is_some() {
            let opt = opt.unwrap();
            let d = match opt {
                TimeOpt::Day(d) => Duration::days(d.into()),
                TimeOpt::Dhms(d, h, m, s) => {
                    Duration::days(d.into())
                        + Duration::hours(h.into())
                        + Duration::minutes(m.into())
                        + Duration::seconds(s.into())
                }
            };
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
        // self.engine.get(&self.gen_key(key))
        let v = self.engine.get(&self.gen_key(key));
        println!("{:?}", v);
        v
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
        kv.set("foo", json!(22), Some(TimeOpt::Dhms(0, 0, 0, 1)));
        let v = kv.get("foo");
        assert_eq!(Some(json!(22)), v);
        thread::sleep(time::Duration::from_millis(1100));
        let v = kv.get("foo");
        assert_eq!(None, v);
        kv.flush_expired();
    }
    #[test]
    fn t_time() {
        let now: DateTime<Utc> = Utc::now();
        let s = now.to_string();
        let s2 = now.to_rfc2822();
        let s3 = now.to_rfc3339();
        let s4 = s.parse::<DateTime<Utc>>().unwrap();
        println!("{}\n{}\n{}", &s, &s2, &s3);
        assert_eq!(now, s4);
    }
}
