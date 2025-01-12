//! Module regroupant les éléments fondamentaux du projet.
//!
//! # Contenu
//!
//! - [`node`] : Structure de base (nœud) pouvant servir de maillon pour des listes chaînées ou pour 
//!   représenter un élément stocké dans le cache.
//! - [`traits`] : Trait définissant les méthodes requises pour un cache LRU.
//!
//! Les modules présents ici fournissent les briques de base nécessaires sur lesquelles s'appuient
//! les implémentations de cache dans le module [`caches`](crate::caches).
//! 

pub mod node;
pub mod traits;