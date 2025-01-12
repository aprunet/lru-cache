use lru_cache::caches::memory::MemoryCache;
use lru_cache::infra::traits::LruCache;

#[test]
fn test_lru_integration_classic() {
    let mut cache: MemoryCache<String, i32> = MemoryCache::new(2);

    cache.set("hello".to_string(), 123);
    cache.set("world".to_string(), 456);

    assert_eq!(cache.get(&"hello".to_string()), Some(&123));

    cache.set("foo".to_string(), 999);

    assert_eq!(cache.get(&"world".to_string()), None);

    assert_eq!(cache.get(&"hello".to_string()), Some(&123));

    assert_eq!(cache.get(&"foo".to_string()), Some(&999));
}
