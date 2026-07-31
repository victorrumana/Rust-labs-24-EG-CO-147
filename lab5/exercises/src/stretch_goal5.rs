use std::collections::HashMap;
use std::hash::Hash;

pub struct Cache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    usage_order: Vec<K>, // Tracks key recency
}

impl<K: Eq + Hash + Clone, V> Cache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            map: HashMap::new(),
            usage_order: Vec::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Update recency order
            self.usage_order.retain(|k| k != key);
            self.usage_order.push(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value);
            self.usage_order.retain(|k| k != &key);
            self.usage_order.push(key);
        } else {
            if self.map.len() >= self.capacity {
                // Evict least-recently-used key
                if !self.usage_order.is_empty() {
                    let lru_key = self.usage_order.remove(0);
                    self.map.remove(&lru_key);
                }
            }
            self.map.insert(key.clone(), value);
            self.usage_order.push(key);
        }
    }
}

pub fn run() {
    println!("=== LAB 5 STRETCH GOAL ===");
    let mut cache: Cache<&str, i32> = Cache::new(2);

    cache.insert("a", 100);
    cache.insert("b", 200);
    println!("Get 'a': {:?}", cache.get(&"a")); // Makes "a" recently used

    cache.insert("c", 300); // Evicts "b" because "a" was used recently

    println!("Get 'b' (should be evicted): {:?}", cache.get(&"b"));
    println!("Get 'c': {:?}", cache.get(&"c"));
}