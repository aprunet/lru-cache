//! Module rassemblant les différentes implémentations concrètes de cache.
//!
//! # Contenu
//!
//! - [`memory`] : Implémentation d'un cache LRU en mémoire (LRU « classique »)  
//! - [`persistent`] : Implémentation d'un cache LRU persistant (sauvegarde/chargement depuis un fichier)
//!
//! Chaque implémentation repose sur les concepts définis dans le module [`infra`](crate::infra),
//! notamment le trait `LruCache`.

pub mod memory;
pub mod persistent;