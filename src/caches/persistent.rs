//! Module pour la gestion d'un cache LRU persistant.
//!
//! Cette implémentation permet de stocker les données et l'ordre d'usage
//! dans un fichier, afin de persister l'état du cache entre deux exécutions.
//!
//! **Format du fichier :**
//! 1. Première ligne : la capacité (nombre maximum d'entrées)
//! 2. Deuxième ligne : la liste des clés dans l'ordre de leur utilisation récente (séparées par ",")
//! 3. Les lignes suivantes : chaque ligne correspond à une paire `clé\tvaleur`
//!
//! **Remarques :**
//! - Pour la sérialisation, nous utilisons simplement des chaînes de caractères,
//!   car l'utilisation de librairies externes (comme Serde) n'est pas autorisée.
//! - Les clés et les valeurs doivent donc être convertibles en String et
//!   parsables depuis une String (via `ToString` et `FromStr`).
//! - Si la lecture/écriture dans le fichier échoue, l'erreur n'est pas gérée
//!   finement : le cache sera simplement vide ou pourra planter selon le contexte.

use std::collections::{HashMap, VecDeque};
use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;
use crate::infra::traits::LruCache;

pub struct PersistentCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    usage: VecDeque<K>,
    file_path: String,
}

impl<K, V> PersistentCache<K, V> 
where
    K: Eq + Hash + Clone + ToString + FromStr,
    V: Clone + ToString + FromStr,
{
    pub fn new_persistent(capacity: usize, file_path: &str) -> Self {
        let mut cache = Self {
            capacity,
            map: HashMap::new(),
            usage: VecDeque::new(),
            file_path: file_path.to_string(),
        };
        cache.load_from_file();
        cache
    }

    fn load_from_file(&mut self) {
        let file = match File::open(&self.file_path) {
            Ok(f) => f,
            Err(_) => return,
        };
        let mut buf_reader = BufReader::new(file);

        let mut capacity_line = String::new();
        if buf_reader.read_line(&mut capacity_line).is_ok() {
            if let Ok(parsed_cap) = capacity_line.trim().parse::<usize>() {
                self.capacity = parsed_cap;
            }
        }

        let mut usage_line = String::new();
        if buf_reader.read_line(&mut usage_line).is_ok() {
            let keys: Vec<&str> = usage_line.trim().split(',').collect();
            for k_str in keys {
                if !k_str.is_empty() {
                    if let Ok(k_parsed) = K::from_str(k_str) {
                        self.usage.push_back(k_parsed);
                    }
                }
            }
        }

        let mut lines = String::new();
        if buf_reader.read_to_string(&mut lines).is_ok() {
            for line in lines.lines() {
                let mut kv = line.split('\t');
                if let (Some(k_str), Some(v_str)) = (kv.next(), kv.next()) {
                    if let (Ok(k_parsed), Ok(v_parsed)) = (
                        K::from_str(k_str.trim()),
                        V::from_str(v_str.trim()),
                    ) {
                        self.map.insert(k_parsed, v_parsed);
                    }
                }
            }
        }
    }

    fn save_to_file(&self) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.file_path);

        let mut file = match file {
            Ok(f) => f,
            Err(_) => return,
        };

        let _ = writeln!(file, "{}", self.capacity);

        let usage_str = self
            .usage
            .iter()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let _ = writeln!(file, "{}", usage_str);

        for (k, v) in &self.map {
            let _ = writeln!(file, "{}\t{}", k.to_string(), v.to_string());
        }
    }
}

impl<K, V> LruCache<K, V> for PersistentCache<K, V> 
where
    K: Eq + Hash + Clone + ToString + FromStr,
    V: Clone + ToString + FromStr,
{
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            usage: VecDeque::new(),
            file_path: "default_lru_cache.txt".to_string(),
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            let mut index = None;
            for (i, k) in self.usage.iter().enumerate() {
                if k == key {
                    index = Some(i);
                    break;
                }
            }
            if let Some(i) = index {
                self.usage.remove(i);
            }
            self.usage.push_back(key.clone());
            self.save_to_file();
            self.map.get(key)
        } else {
            None
        }
    }

    fn set(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            let mut index = None;
            for (i, k) in self.usage.iter().enumerate() {
                if *k == key {
                    index = Some(i);
                    break;
                }
            }
            if let Some(i) = index {
                self.usage.remove(i);
            }
            self.usage.push_back(key.clone());
            self.map.insert(key, value);
        } else {
            if self.map.len() == self.capacity {
                if let Some(removed_key) = self.usage.pop_front() {
                    self.map.remove(&removed_key);
                }
            }
            self.usage.push_back(key.clone());
            self.map.insert(key, value);
        }
        self.save_to_file();
    }
}
