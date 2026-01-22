use std::mem;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const INITIAL_NBUCKETS: usize = 1;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items >= self.buckets.len() / 3 {
            self.resize();
        }

        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];

        self.items += 1;
        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key, value));

        None
    }
    fn bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        // feed value of key into the hasher
        key.hash(&mut hasher);
        // finish the hasher to get the final hash value
        let bucket: usize = (hasher.finish() % self.buckets.len() as u64) as usize;
        bucket
    }
    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => n * 2,
        };

        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));
        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            // feed value of key into the hasher
            key.hash(&mut hasher);
            // finish the hasher to get the final hash value
            let bucket: usize = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }
        let _ = mem::replace(&mut self.buckets, new_buckets);
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket = self.bucket(key);
        let bucket = &mut self.buckets[bucket];

        let i: usize = bucket.iter().position(|&(ref ekey, _)| ekey == key)?;
        self.items -= 1;
        Some(bucket.swap_remove(i).1)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket = self.bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|&(ekey, _)| ekey == key)
            .map(|&(_, ref evalue)| evalue)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let bucket = self.bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|&(ekey, _)| ekey == key)
            .is_some()
    }

    pub fn len(&self) -> usize {
        self.items
    }
    pub fn is_empty(&self) -> bool {
        self.items == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn insert_and_get() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        map.insert("foo", 42);
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
        assert_eq!(map.get(&"foo"), Some(&42));
        assert_eq!(map.remove(&"foo"), Some(42));
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        assert_eq!(map.get(&"foo"), None);
        map.insert("foo", 42);
        assert!(map.contains_key(&"foo"))
    }
}
