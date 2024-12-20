use std::collections::{hash_map::IntoIter, HashMap};
use std::hash::Hash;

pub(crate) struct HashMapChunks<K, V> {
    iter: IntoIter<K, V>,
    chunk_size: usize,
}

impl<K, V> HashMapChunks<K, V> {
    pub(crate) fn new(map: HashMap<K, V>, size: usize) -> Self {
        Self { iter: map.into_iter(), chunk_size: size }
    }
}

impl<K, V> Iterator for HashMapChunks<K, V>
where
    K: Eq + Hash
{
    type Item = HashMap<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.len() == 0 {
            None
        } else {
            let chunk_size = std::cmp::min(self.iter.len(), self.chunk_size);
            Some(HashMap::from_iter(
                (0..chunk_size)
                    .map(|_| self.iter.next().unwrap())
            ))
        }
    }
}
