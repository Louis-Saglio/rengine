use array_init::array_init;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

const STACK_HASH_MAP_SIZE: usize = 100;

pub struct StackHashMap<K, V> {
    buckets: [[Option<(K, V)>; STACK_HASH_MAP_SIZE]; STACK_HASH_MAP_SIZE],
    hasher_builder: RandomState,
}

impl<K: Hash + Eq, V> StackHashMap<K, V> {
    pub fn new() -> Self {
        StackHashMap {
            buckets: array_init(|_| array_init(|_| None)),
            hasher_builder: RandomState::new(),
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = self.hasher_builder.build_hasher();
        key.hash(&mut hasher);
        hasher.finish() as usize % STACK_HASH_MAP_SIZE
    }

    pub fn push(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        let bucket = &mut self.buckets[index];
        for slot in bucket.iter_mut() {
            if slot.is_none() {
                *slot = Some((key, value));
                break;
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key);
        let bucket = &self.buckets[index];
        for data in bucket.iter() {
            match data {
                None => {}
                Some((stored_key, value)) => {
                    if stored_key == key {
                        return Some(value);
                    }
                }
            }
        }
        return None;
    }
}
