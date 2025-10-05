use moka::future::Cache;
use std::sync::{Arc, Mutex};

pub struct CacheManager {
    cache: Cache<String, String>,
    version: Arc<Mutex<u64>>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(1000), // max capacity
            version: Arc::new(Mutex::new(1)),
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        self.cache.get(key).await
    }

    pub async fn set(&self, key: String, value: String) {
        self.cache.insert(key, value).await;
    }

    pub fn bump_version(&self) -> u64 {
        let mut version = self.version.lock().unwrap();
        *version += 1;
        tracing::info!("Cache version bumped to {}", *version);
        *version
    }

    pub fn get_version(&self) -> u64 {
        *self.version.lock().unwrap()
    }

    pub fn make_key(&self, path: &str) -> String {
        format!("{}:v{}", path, self.get_version())
    }

    pub async fn invalidate(&self) {
        self.cache.invalidate_all();
        self.bump_version();
    }
}