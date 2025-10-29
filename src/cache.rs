use moka::future::Cache;

pub struct CacheManager {
    cache: Cache<String, String>,
}

impl CacheManager {
    pub fn new() -> Self {
        // Create cache with 1000 max capacity and 1 hour TTL
        let cache = Cache::builder()
            .max_capacity(1000)
            .time_to_live(std::time::Duration::from_secs(3600)) // 1 hour
            .build();

        Self { cache }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        self.cache.get(key).await
    }

    pub async fn set(&self, key: String, value: String) {
        self.cache.insert(key, value).await;
    }

    pub fn make_key(&self, path: &str) -> String {
        path.to_string()
    }

    // Maybe doesn't work?
    pub async fn invalidate(&self) {
        self.cache.invalidate_all();
    }
}
