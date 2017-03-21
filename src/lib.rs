use std::collections::HashMap;
use std::hash::Hasher;
use std::hash::Hash;

// Took me FOREVER to understand this.  The Cache Item is taking
// ownership of the key.  The HashMap needs to be able to find, cache,
// and compare the keys, while having ownership of something for which
// it can determine the lifetime.  THIS sets up something the HashMap
// can *own*, which satisfies its needs, but since it's a
// pointer-to-a-K, your next step is to tell the Hash function how to
// hash and compare.  Yay.

struct LruCacheKey<K> {
    key: *const K
}

// I thought I could get away without the unsafes this until I got the
// 'dereference of raw pointer requires unsafe function or block'.
// I'm sure I'll get this someday.

impl<K: Hash> Hash for LruCacheKey<K> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { (*self.key).hash(state) }
    }
}

impl<K: PartialEq> PartialEq for LruCacheKey<K> {
    fn eq(&self, other: &LruCacheKey<K>) -> bool {
        unsafe { (*self.key).eq(&*other.key) }
    }
}

// The compiler complained.  Yes, I cheated and looked it up.

impl<K: Eq> Eq for LruCacheKey<K> {}

struct LruCacheItem<K, V> {
    key: K,
    value: V,
}

impl<K, V> LruCacheItem<K, V> {
    fn new(key: K, value: V) -> Self {
        LruCacheItem {
            key: key,
            value: value
        }
    }
}

pub struct LruCache<K, V> {
    cache: HashMap<LruCacheKey<K>, LruCacheItem<K, V>>,
    max_item_size: usize,
    max_items: usize,
    current_size: usize
}

impl<K: Hash + Eq, V: Sized> LruCache<K, V> {
    pub fn new(max_item_size: usize, max_items: usize) -> Self {
        LruCache {
            cache: HashMap::new(),
            max_item_size: max_item_size,
            max_items: max_items,
            current_size: 0
        }
    }

    pub fn insert(&mut self, key: K, v: V) -> &mut Self {
        let item = LruCacheItem::new(key, v);
        let key = LruCacheKey { key: &item.key };
        self.cache.insert(key, item);
        self
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let lkey = LruCacheKey { key: key };
        match self.cache.remove(&lkey) {
            Some(v) => Some(v.value),
            None => None,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let lkey = LruCacheKey { key: key };
        match self.cache.get(&lkey) {
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
        let mut cache = LruCache::new(2, 3);
        cache.insert("a", "A");
        cache.insert("b", "B");
        println!("Size: {}", cache.len());
        assert_eq!(cache.get(&"a"), Some(&"A"));
        assert_eq!(cache.get(&"c"), None);
        assert_eq!(cache.remove(&"b"), Some("B"));
    }
}
