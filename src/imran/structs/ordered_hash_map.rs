use std::collections::HashMap;

pub struct OrderedHashMap<K, V> {
    _order: Vec<String>,
    _map: HashMap<K, V>,
}

impl<K, V> OrderedHashMap<K, V> {
    fn new() -> OrderedHashMap<K, V> {
        OrderedHashMap {
            _order: vec![],
            _map: HashMap::new(),
        }
    }
    fn insert(&mut self, key: K, value: V) {
        self.insert(key, value);
    }
}