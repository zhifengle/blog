use serde_json::Value;
use std::fs::{OpenOptions, self};
use std::io::Read;
use std::path::Path;

trait KvEngine {
    fn set(&mut self, key: &str, value: Value) -> bool;
    fn get(&self, key: &str) -> Option<Value>;
    fn remove(&mut self, key: &str);
    fn keys(&self) -> Vec<&String>;
}

struct JsonEngine<T: AsRef<Path>> {
    filename: T,
    config: Value,
}

fn read_json_file(filename: impl AsRef<Path>) -> anyhow::Result<Value> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.is_empty() {
        contents = "{}".to_string();
    }
    let v: serde_json::Value = serde_json::from_str(&contents)?;
    Ok(v)
}

impl<T: AsRef<Path>> JsonEngine<T> {
    pub fn new(filename: T) -> Self {
        Self {
            config: read_json_file(filename.as_ref()).unwrap(),
            filename,
        }
    }
    fn write_file(&self, contents: &str) -> anyhow::Result<()> {
        fs::write(&self.filename, contents)?;
        Ok(())
    }
}

impl<T: AsRef<Path>> KvEngine for JsonEngine<T> {
    fn set(&mut self, key: &str, value: Value) -> bool {
        let config = self.config.as_object_mut().unwrap();
        config.insert(key.to_string(), value);
        // let config = serde_json::to_value(config).unwrap();
        // let s = serde_json::to_string_pretty(&config)?;
        // self.write_file(s.as_str())?;
        // todo!()
        true
    }

    fn get(&self, key: &str) -> Option<Value> {
        match self.config.get(key) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }

    fn remove(&mut self, key: &str) {
        let config = self.config.as_object_mut().unwrap();
        config.remove(key);
    }

    // @TODO convert &String to &str
    fn keys(&self) -> Vec<&String> {
        self.config.as_object().unwrap().keys().collect()
    }
}

// 实现 Drop 后;是否需要清理
// 使用 libc::atexit 更好。暂时利用 Drop
impl<T: AsRef<Path>> Drop for JsonEngine<T> {
    fn drop(&mut self) {
        let s = serde_json::to_string_pretty(&self.config).unwrap();
        self.write_file(s.as_str()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn t_get() {
        let mut s = JsonEngine::new("test-drop.json");
        s.set("11", json!("sfs"));
    }
}