use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const INITIAL_NBUCKETS: usize = 1;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn resize(&mut self) {
        let target_buckets = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => n * 2,
        };

        todo!("more functionality ");
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut hasher = DefaultHasher::new();
        // feed value of key into the hasher
        key.hash(&mut hasher);
        // finish the hasher to get the final hash value
        let bucket: usize = (hasher.finish() % self.buckets.len() as u64) as usize;
        let bucket = &mut self.buckets[bucket];

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                use std::mem;
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key, value));

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn insert() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
    }
}
