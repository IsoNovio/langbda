use super::node::Node;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug)]
pub struct TrieMultiMap<K: Ord + Hash, V: Hash + Eq> {
    root: Node<K, V>,
}

impl<K: Ord + Hash + Clone, V: Hash + Eq> TrieMultiMap<K, V> {
    pub fn new() -> Self {
        TrieMultiMap { root: Node::new() }
    }
}

impl<K: Ord + Hash + Clone, V: Hash + Eq> TrieMultiMap<K, V> {
    pub fn insert<I>(&mut self, sorted_keys: I, value: V) -> bool
    where
        I: IntoIterator<Item = K>,
    {
        self.root.insert_child(sorted_keys, value)
    }
}

impl<K: Ord + Hash + Clone + std::fmt::Debug, V: Hash + Eq + Clone + std::fmt::Debug>
    TrieMultiMap<K, V>
{
    pub fn get_subsets<I>(&self, sorted_keys: &I) -> Vec<(HashSet<K>, V)>
    where
        I: IntoIterator<Item = K> + Clone,
    {
        self.root.get_subsets(sorted_keys.clone().into_iter())
    }
    pub fn iter(&self) -> impl Iterator<Item = (Vec<K>, V)> {
        self.root.iter()
    }
}

impl<K: Ord + Hash + Display, V: Hash + Eq + Display> Display for TrieMultiMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root.fmt(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_subsets() {
        let mut map = TrieMultiMap::new();
        map.insert(vec!['1', '2', '3'], 123);
        map.insert(vec!['1', '2', '4'], 124);
        map.insert(vec!['1', '5', '6'], 156);
        map.insert(vec!['1', '5', '7'], 157);

        let subsets = map.get_subsets(&vec!['1', '2', '3', '5', '6']);
        assert_eq!(
            subsets,
            Vec::from([
                (HashSet::from(['1', '2', '3']), 123),
                (HashSet::from(['1', '5', '6']), 156)
            ])
        );
    }

    #[test]
    fn test_display() {
        let mut map = TrieMultiMap::new();
        map.insert(vec!['1', '2', '3'], 123);
        map.insert(vec!['1', '2', '4'], 124);
        map.insert(vec!['1', '5', '6'], 156);
        map.insert(vec!['1', '5', '7'], 157);
        println!("{}", map);
    }
}
