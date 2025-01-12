use lru_cache::caches::memory::MemoryCache;
use lru_cache::infra::traits::LruCache;

pub struct Cache {
    inner: MemoryCache<String, String>,
}

impl Cache {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: MemoryCache::new(capacity),
        }
    }
    
    pub fn put(&mut self, key: &str, value: String) {
        self.inner.set(key.to_string(), value);
    }
    
    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.inner.get(&key.to_string())
    }
}

#[test]
fn test_lru_cache() {
    let mut cache = Cache::new(3); // Taille de 3

    cache.put("A", String::from("value_a"));
    cache.put("B", String::from("value_b"));
    cache.put("C", String::from("value_c"));
    cache.put("D", String::from("value_d"));
    // Premier élément moins récemment utilisé et dernier le plus récent
    // Cache == [B, C, D]

    let my_value = cache.get("A");
    assert_eq!(my_value, None);

    let my_value = cache.get("D");
    assert_eq!(my_value, Some(&String::from("value_d")));
    // Cache == [B, C, D]

    let my_value = cache.get("B");
    assert_eq!(my_value, Some(&String::from("value_b")));
    // Cache == [C, D, B]

    let my_value = cache.get("C");
    assert_eq!(my_value, Some(&String::from("value_c")));
    // Cache == [D, B, C]

    let my_value = cache.get("X");
    assert_eq!(my_value, None);
    // Cache == [D, B, C]

    cache.put("A", String::from("value_a"));
    // Cache == [B, C, A]

    cache.put("X", String::from("value_x"));
    // Cache == [C, A, X]

    let my_value = cache.get("B");
    assert_eq!(my_value, None);
    // Cache == [C, A, X]

    let my_value = cache.get("D");
    // Cache == [C, A, X]
    assert_eq!(my_value, None);
}
