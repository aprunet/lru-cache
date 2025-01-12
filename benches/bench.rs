use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use lru_cache::infra::traits::LruCache;

use lru_cache::caches::memory::MemoryCache;

fn lru_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("LRU Cache");
    group.bench_function(BenchmarkId::new("Cache set/get", 3), |b| {
        b.iter(|| {
            let mut cache = MemoryCache::new(3);
            for i in 0..1000 {
                let key = black_box(format!("key{}", i % 4));
                cache.set(key.clone(), i);
                let _ = cache.get(&key);
            }
        })
    });
    group.finish();
}

criterion_group!(benches, lru_bench);
criterion_main!(benches);
