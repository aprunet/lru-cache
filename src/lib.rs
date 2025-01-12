//! Crate pour un cache LRU en Rust.
//!
//! Ce crate contient :
//! - Un module `cache` qui fournit deux implémentations de cache LRU : 
//!   - `MemoryCache` (en mémoire) 
//!   - `PersistentCache` (persistant via fichier)
//! - Un module `core` illustrant un exemple d’utilisation (avec `run()`).
//! - Un module `node` et un module `traits` (définition du trait `LruCache`).
//!
//! # Exemple
//! ```ignore
//! use lru_cache::cache::memory::MemoryCache;
//! use lru_cache::core::traits::LruCache;
//!
//! fn main() {
//!     let mut cache = MemoryCache::new(2);
//!     cache.set("clé1", "val1");
//!     cache.set("clé2", "val2");
//!     assert_eq!(cache.get(&"clé1"), Some(&"val1"));
//!     cache.set("clé3", "val3");
//!     assert_eq!(cache.get(&"clé2"), None); 
//! }
//! ```

pub mod caches;
pub mod infra;
