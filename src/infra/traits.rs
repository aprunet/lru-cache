//! Module trait pour la définition des méthodes requises par un cache LRU.
//!
//! Ce trait décrit les opérations fondamentales attendues pour tout cache LRU.

pub trait LruCache<K, V> {
    /// Crée un nouveau cache LRU avec une capacité maximale donnée.
    fn new(capacity: usize) -> Self;

    /// Récupère la valeur associée à la clé donnée (si elle existe),
    /// et met à jour la position de la clé comme étant récemment utilisée.
    fn get(&mut self, key: &K) -> Option<&V>;

    /// Insère ou met à jour la valeur associée à la clé donnée,
    /// et met à jour la position de la clé comme étant récemment utilisée.
    /// Si la taille maximale est atteinte, la clé la moins récemment utilisée sera évincée.
    fn set(&mut self, key: K, value: V);
}
