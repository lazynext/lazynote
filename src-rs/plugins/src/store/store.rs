use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    ffi::OsStr,
    io::Write,
    path::PathBuf,
    sync::{Arc, LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use serde_json::Value;

use crate::path::paths;

pub struct Store {
    file_path: PathBuf,
    file_name: String,
    data: HashMap<String, Value>,
    pretty: bool,
}

impl Store {
    pub fn new<S: AsRef<OsStr>>(file_path: S, pretty: bool) -> Store {
        let file_path = PathBuf::from(file_path.as_ref());
        let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        // 如果是相对目录, 则指向程序目录
        let file_path = if file_path.is_relative() {
            paths().resource_dir().join(file_path)
        } else {
            file_path
        };
        let mut result = Self {
            file_name,
            file_path,
            pretty,
            data: HashMap::new(),
        };
        result.reload().unwrap_or_else(|err| {
            log::error!("init store [{}] error: {}", &result.file_name, err);
        });
        result
    }
    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
    pub fn file_name(&self) -> &String {
        &self.file_name
    }
    pub fn reload(&mut self) -> Result<()> {
        self.data.clear();
        if self.file_path.exists() {
            let s = std::fs::read_to_string(&self.file_path)?;
            let v = serde_json::from_str::<Value>(s.as_str())?;
            match v.as_object() {
                Some(v) => {
                    for (key, val) in v.iter() {
                        self.data
                            .entry(key.clone())
                            .and_modify(|e| *e = val.clone())
                            .or_insert(val.clone());
                    }
                }
                None => self.data.clear(),
            }
        }
        Ok(())
    }
    pub fn save(&self) -> Result<()> {
        std::fs::create_dir_all(self.file_path.parent().expect("invalid file path"))?;
        let json_str = if self.pretty {
            serde_json::to_string_pretty(&self.data)?
        } else {
            serde_json::to_string(&self.data)?
        };
        let mut f = std::fs::File::create(&self.file_path)?;
        f.write_all(json_str.as_bytes())?;
        Ok(())
    }
    pub fn clear(&mut self) {
        self.data.clear()
    }
    pub fn delete(&mut self, key: impl AsRef<str>) -> bool {
        self.data.remove(key.as_ref()).is_some()
    }
    pub fn has(&self, key: impl AsRef<str>) -> bool {
        self.data.contains_key(key.as_ref())
    }
    pub fn get(&self, key: impl AsRef<str>) -> Option<Value> {
        self.data.get(&key.as_ref().to_string()).cloned()
    }
    pub fn set(&mut self, key: impl AsRef<str>, val: &Value) {
        self.data
            .entry(key.as_ref().to_string())
            .and_modify(|e| *e = val.clone())
            .or_insert(val.clone());
    }
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.data.keys()
    }
    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.data.values()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.data.iter()
    }
    pub fn all(&self) -> HashMap<String, Value> {
        self.data.clone()
    }
}

fn get_data_path() -> PathBuf {
    paths().data_path()
}

fn get_local_data_path() -> PathBuf {
    paths().local_data_path()
}

pub static STORE_COMMON: LazyLock<Arc<RwLock<Store>>> = LazyLock::new(|| {
    Arc::new(RwLock::new(Store::new(
        &get_data_path().join("common.json"),
        true,
    )))
});

pub static STORE_SETTINGS: LazyLock<Arc<RwLock<Store>>> = LazyLock::new(|| {
    Arc::new(RwLock::new(Store::new(
        &get_data_path().join("settings.json"),
        true,
    )))
});

pub static STORE_LOCAL_COMMON: LazyLock<Arc<RwLock<Store>>> = LazyLock::new(|| {
    Arc::new(RwLock::new(Store::new(
        &get_local_data_path().join("common.json"),
        true,
    )))
});

pub static STORE_LOCAL_SETTINGS: LazyLock<Arc<RwLock<Store>>> = LazyLock::new(|| {
    Arc::new(RwLock::new(Store::new(
        &get_local_data_path().join("settings.json"),
        true,
    )))
});

pub(crate) fn get_store_mut(store_key: &str) -> Result<RwLockWriteGuard<'static, Store>> {
    if store_key == "STORE_COMMON" {
        Ok(STORE_COMMON.write().unwrap())
    } else if store_key == "STORE_SETTINGS" {
        Ok(STORE_SETTINGS.write().unwrap())
    } else if store_key == "STORE_LOCAL_COMMON" {
        Ok(STORE_LOCAL_COMMON.write().unwrap())
    } else if store_key == "STORE_LOCAL_SETTINGS" {
        Ok(STORE_LOCAL_SETTINGS.write().unwrap())
    } else {
        Err(anyhow!(
            "get_store Error: notfound store_key: {}",
            store_key
        ))
    }
}

pub(crate) fn get_store(store_key: &str) -> Result<RwLockReadGuard<'static, Store>> {
    if store_key == "STORE_COMMON" {
        Ok(STORE_COMMON.read().unwrap())
    } else if store_key == "STORE_SETTINGS" {
        Ok(STORE_SETTINGS.read().unwrap())
    } else if store_key == "STORE_LOCAL_COMMON" {
        Ok(STORE_LOCAL_COMMON.read().unwrap())
    } else if store_key == "STORE_LOCAL_SETTINGS" {
        Ok(STORE_LOCAL_SETTINGS.read().unwrap())
    } else {
        Err(anyhow!(
            "get_store Error: notfound store_key: {}",
            store_key
        ))
    }
}

pub fn common() -> Result<RwLockReadGuard<'static, Store>> {
    get_store("STORE_COMMON")
}
pub fn common_mut() -> Result<RwLockWriteGuard<'static, Store>> {
    get_store_mut("STORE_COMMON")
}
pub fn settings() -> Result<RwLockReadGuard<'static, Store>> {
    get_store("STORE_SETTINGS")
}
pub fn settings_mut() -> Result<RwLockWriteGuard<'static, Store>> {
    get_store_mut("STORE_SETTINGS")
}
pub fn local_common() -> Result<RwLockReadGuard<'static, Store>> {
    get_store("STORE_LOCAL_COMMON")
}
pub fn local_common_mut() -> Result<RwLockWriteGuard<'static, Store>> {
    get_store_mut("STORE_LOCAL_COMMON")
}
pub fn local_settings() -> Result<RwLockReadGuard<'static, Store>> {
    get_store("STORE_LOCAL_SETTINGS")
}
pub fn local_settings_mut() -> Result<RwLockWriteGuard<'static, Store>> {
    get_store_mut("STORE_LOCAL_SETTINGS")
}
