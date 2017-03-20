use std::collections::HashMap;
use std::hash::Hash;

pub struct LruCacheItem<V> {
    value: V,
}

pub struct LruCache<K, V> {
    cache: HashMap<K, LruCacheItem<V>>,
}

impl<V: Sized> LruCacheItem<V> {
    fn new(value: V) -> Self {
        LruCacheItem { value: value }
    }
}

impl<K: Hash + Eq, V: Sized> LruCache<K, V> {
    pub fn new() -> Self {
        LruCache { cache: HashMap::new() }
    }

    pub fn insert(&mut self, key: K, v: V) -> &mut Self {
        self.cache.insert(key, LruCacheItem::new(v));
        self
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.cache.remove(key) {
            Some(v) => Some(v.value),
            None => None,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        match self.cache.get(key) {
            Some(v) => Some(&v.value),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LruCache;

    #[test]
    fn it_works() {
        let mut cache = LruCache::new();
        cache.insert('a', 'A');
        cache.insert('b', 'B');
        assert_eq!(cache.get(&'a'), Some(&'A'));
        assert_eq!(cache.get(&'c'), None);
        assert_eq!(cache.remove(&'b'), Some('B'));
    }
}
