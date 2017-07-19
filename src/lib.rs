use std::mem;
use std::collections::HashMap;
use std::ptr;
use std::hash::{Hasher, Hash};


// A reference to the key. Built a lot in an ad-hoc way, but still
// pretty cheap.
struct LruCacheKey<K> {
    key: *const K,
}


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


impl<K: Eq> Eq for LruCacheKey<K> {}


pub trait ArbSized {
    fn size(&self) -> usize;
}


impl ArbSized for String {
    fn size(&self) -> usize {
        self.len()
    }
}


impl ArbSized for str {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<'a> ArbSized for &'a str {
    fn size(&self) -> usize {
        (*self).len()
    }
}

struct LruCacheItem<K, V> {
    key: K,
    value: V,
    prev: *mut LruCacheItem<K, V>,
    next: *mut LruCacheItem<K, V>,
}


impl<K, V> LruCacheItem<K, V> {
    fn new(key: K, value: V) -> Self {
        LruCacheItem {
            key: key,
            value: value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}


pub struct LruCritCache<K, V> {
    cache: HashMap<LruCacheKey<K>, Box<LruCacheItem<K, V>>>,
    maxitemsize: usize,
    maxtotalsize: usize,
    cursize: usize,
    head: *mut LruCacheItem<K, V>,
    tail: *mut LruCacheItem<K, V>,
}


impl<K: Hash + Eq, V: ArbSized> LruCritCache<K, V> {
    pub fn new(maxcount: usize, maxsize: usize) -> LruCritCache<K, V> {
        let mut lrucache = LruCritCache {
            cache: HashMap::new(),
            maxitemsize: maxsize,
            maxtotalsize: maxsize * maxcount,
            cursize: 0,
            /* I like how the documentation says "Really, reconsider
               before you do something like this." */
            head: unsafe { Box::into_raw(Box::new(mem::uninitialized::<LruCacheItem<K, V>>())) },
            tail: unsafe { Box::into_raw(Box::new(mem::uninitialized::<LruCacheItem<K, V>>())) },
        };

        // The Oroborous Condition!
        unsafe {
            (*lrucache.head).next = lrucache.tail;
            (*lrucache.tail).prev = lrucache.head;
        }

        lrucache
    }


    pub fn accepts(&mut self, value: &V) -> bool {
        value.size() <= self.maxitemsize
    }


    /* From the oldest upward, discard objects until there's enough
       room for the requested object.
    */

    fn eject(&mut self, value: &V, node_ptr: &Option<*mut LruCacheItem<K, V>>) {
        match *node_ptr {
            Some(node_ptr) => {
                // Remove the size of the value for an existing candidate node.
                unsafe { self.cursize = self.cursize - (*node_ptr).value.size() };
            }
            None => {}
        }

        while self.cursize + value.size() > self.maxtotalsize {
            let old_key = LruCacheKey { key: unsafe { &(*(*self.tail).prev).key } };
            let old_node = self.cache.remove(&old_key).unwrap();
            self.cursize = self.cursize - old_node.value.size();
        }
    }


    /// Put a key-value pair into the cache, ejecting older entries as
    /// necessary until the new value "fits" according to the ArbSize
    /// trait.

    pub fn insert(&mut self, key: K, value: V) -> bool {
        if !self.accepts(&value) {
            return false;
        }

        let node_ptr = self.cache.get_mut(&LruCacheKey { key: &key }).map(|node| {
            let node_ptr: *mut LruCacheItem<K, V> = &mut **node;
            node_ptr
        });

        /* Eject until there's enough room to store the value. Pass
           the node_ptr so eject can avoid over-ejection by knowing to
           calculate the candidate node value size, if a candidate
           node is already present.
        */
        self.eject(&value, &node_ptr);

        match node_ptr {
            Some(node_ptr) => {
                unsafe {
                    self.cursize = self.cursize + value.size();
                    (*node_ptr).value = value;
                }
                self.promote(node_ptr);
            }
            None => {
                self.cursize += value.size();
                let mut node = Box::new(LruCacheItem::new(key, value));
                let node_ptr: *mut LruCacheItem<K, V> = &mut *node;
                self.attach(node_ptr);
                let keyref = unsafe { &(*node_ptr).key };
                self.cache.insert(LruCacheKey { key: keyref }, node);
            }
        }
        true
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let lkey = LruCacheKey { key: key };
        match self.cache.get(&lkey) {
            Some(v) => Some(&v.value),
            None => None,
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let key = LruCacheKey { key: key };
        match self.cache.remove(&key) {
            None => None,
            Some(lru_entry) => {
                self.cursize -= (*lru_entry).value.size();
                self.detach(&(*lru_entry));
                Some(lru_entry.value)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn size(&self) -> usize {
        self.cursize
    }

    #[inline]
    fn detach(&mut self, node: *const LruCacheItem<K, V>) {
        unsafe {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
        }
    }

    #[inline]
    fn attach(&mut self, node: *mut LruCacheItem<K, V>) {
        unsafe {
            (*node).next = (*self.head).next;
            (*node).prev = self.head;
            (*self.head).next = node;
            (*(*node).next).prev = node;
        }
    }

    #[inline]
    fn promote(&mut self, node: *mut LruCacheItem<K, V>) {
        self.detach(node);
        self.attach(node);
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let key = LruCacheKey { key: key };
        self.cache.contains_key(&key)
    }
}

impl<K, V> Drop for LruCritCache<K, V> {
    fn drop(&mut self) {
        unsafe {
            let head = *Box::from_raw(self.head);
            let tail = *Box::from_raw(self.tail);

            /* The key and value in these were never used.  Tell the compiler we're
               forgetting about them without "dropping" them. */

            let LruCacheItem { next: _, prev: _, key: head_key, value: head_val } = head;
            let LruCacheItem { next: _, prev: _, key: tail_key, value: tail_val } = tail;

            mem::forget(head_key);
            mem::forget(head_val);
            mem::forget(tail_key);
            mem::forget(tail_val);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::LruCritCache;
    use super::LruCacheItem;

    #[test]
    fn build_an_entry() {
        let entry = LruCacheItem::new("test", "value");
        assert_eq!(entry.key, "test");
        assert_eq!(entry.value, "value");
    }

    #[test]
    fn build_an_empty_cache() {
        let cache: LruCritCache<&str, &str> = LruCritCache::new(5, 2);
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn add_to_the_cache() {
        let mut cache: LruCritCache<&str, &str> = LruCritCache::new(5, 2);
        cache.insert("foo", "aa");
        cache.insert("bar", "bb");
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.size(), 4);
    }

    #[test]
    fn replace_in_the_cache() {
        let mut cache: LruCritCache<&str, &str> = LruCritCache::new(5, 2);
        cache.insert("foo", "aa");
        cache.insert("bar", "bb");
        cache.insert("bar", "c");
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.size(), 3);
    }

    #[test]
    fn delete_in_the_cache() {
        let mut cache: LruCritCache<&str, &str> = LruCritCache::new(5, 2);
        cache.insert("foo", "aa");
        cache.insert("bar", "bb");
        cache.remove(&"bar");
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.size(), 2);
    }

}
