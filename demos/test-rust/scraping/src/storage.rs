use std::error::Error as StdError;
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::path::Path;

pub struct Storage<T: AsRef<Path>> {
    filename: T,
}

impl<T: AsRef<Path>> Storage<T> {
    pub fn new(filename: T) -> Self {
        Storage { filename }
    }
    pub fn write_file(&self, contents: &str) -> Result<(), Box<dyn StdError>> {
        fs::write(&self.filename, contents)?;
        Ok(())
    }
    pub fn get_config(&self) -> Result<serde_json::Value, Box<dyn StdError>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        if contents.is_empty() {
            contents = "{}".to_string();
        }
        // let contents = fs::read_to_string(self.filename.as_str())?;
        let v: serde_json::Value = serde_json::from_str(&contents)?;
        Ok(v)
    }
    pub fn get_value(&self, key: &str) -> Option<serde_json::Value> {
        let config = self.get_config().unwrap();
        match config.get(key) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }
    pub fn set_value(&self, key: &str, val: serde_json::Value) -> Result<(), Box<dyn StdError>> {
        let mut config: serde_json::Value = self.get_config()?;
        let config = config.as_object_mut().unwrap();
        config.insert(key.to_string(), val);
        let config = serde_json::to_value(config)?;
        let s = serde_json::to_string_pretty(&config)?;
        self.write_file(s.as_str())?;
        Ok(())
    }
    pub fn delete_value(&self, key: &str) -> Result<(), Box<dyn StdError>> {
        let mut config: serde_json::Value = self.get_config()?;
        let config = config.as_object_mut().unwrap();
        config.remove(key);
        let config = serde_json::to_value(config)?;
        let s = serde_json::to_string_pretty(&config)?;
        self.write_file(s.as_str())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test() {
        let key = "test";
        let filename = "test-config.json";
        let storage = Storage::new(filename);
        let v = json!({ "a": { "nested": true }, "b": ["an", "aa"] });
        storage.set_value(key, v.clone()).unwrap();
        // storage.delete_value("test").unwrap();
        let val = storage.get_value(key).unwrap();
        assert_eq!(val, v);
        assert_eq!(storage.get_value("not-exist"), None);
        fs::remove_file(filename).unwrap();
    }
    #[test]
    fn test_create() -> Result<(), Box<dyn StdError>> {
        let filename = "create-config.json";
        let storage = Storage::new(filename);
        let v = storage.get_config()?;
        let empty_json = json!({});
        assert_eq!(v, empty_json);
        fs::remove_file(filename)?;
        Ok(())
    }
}
