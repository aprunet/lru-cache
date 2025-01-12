//! Module qui définit une structure de nœud pour une éventuelle gestion chaînée du cache.
//!
//! Dans cette implémentation, on n'utilise pas spécifiquement une liste chaînée
//! double, mais ce module pourrait servir à gérer une structure plus avancée
//! si nécessaire.

#[derive(Debug)]
pub struct Node<K, V> {
    key: K,
    value: V,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
}
