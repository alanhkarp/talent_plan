use std::collections::HashMap;

/// `KvStore` stores string key/value pairs in memory
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Creates an instance of `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }
    /// get - Get the value stored under key or None if key not found
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }
    /// set - Set the value at key
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    /// remove - Remove the entry at key
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
