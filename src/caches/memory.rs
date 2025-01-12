//! Cache LRU en mémoire (LRU "classique").
//!
//! Implémentation d'un cache LRU stocké en mémoire,
//! utilisant une `HashMap` et une `VecDeque` pour la gestion de l'ordre d'utilisation.
//!
//! - `get` remet la clé accédée en fin de file, la rendant la plus récemment utilisée.
//! - `set` remet ou insère la clé en fin de file. Si la capacité est atteinte, on évince
//!   la première clé (la plus ancienne) dans la file.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use crate::infra::traits::LruCache;

pub struct MemoryCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    usage: VecDeque<K>,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> for MemoryCache<K, V> {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            usage: VecDeque::new(),
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            if let Some(pos) = self.usage.iter().position(|k| k == key) {
                self.usage.remove(pos);
            }
            self.usage.push_back(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    fn set(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            if let Some(pos) = self.usage.iter().position(|k| *k == key) {
                self.usage.remove(pos);
            }
            self.map.insert(key.clone(), value);
        } else {
            if self.map.len() == self.capacity {
                if let Some(removed_key) = self.usage.pop_front() {
                    self.map.remove(&removed_key);
                }
            }
            self.map.insert(key.clone(), value);
        }
        self.usage.push_back(key);
    }
}
