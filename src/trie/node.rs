use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug)]
pub struct Node<K: Hash, V: Hash + Eq> {
    key: HashSet<K>,
    values: HashSet<V>,
    children: HashMap<K, Node<K, V>>,
}

impl<K: Clone + Ord + Hash, V: Hash + Eq> Node<K, V> {
    pub fn new() -> Self {
        Node {
            key: HashSet::new(),
            values: HashSet::new(),
            children: HashMap::new(),
        }
    }
    fn from_key(key: HashSet<K>) -> Self {
        Node {
            key,
            values: HashSet::new(),
            children: HashMap::new(),
        }
    }
    fn add_value(&mut self, value: V) -> bool {
        self.values.insert(value)
    }
    fn get_child_or_default(&mut self, key: K) -> &mut Node<K, V> {
        let mut child_key = self.key.clone();
        child_key.insert(key.clone());
        self.children
            .entry(key)
            .or_insert_with(|| Node::from_key(child_key))
    }
    pub fn insert_child<I>(&mut self, sorted_keys: I, value: V) -> bool
    where
        I: IntoIterator<Item = K>,
    {
        let mut iter = sorted_keys.into_iter();
        match iter.next() {
            None => self.add_value(value),
            Some(key) => {
                let child = self.get_child_or_default(key);
                child.insert_child(iter, value)
            }
        }
    }
}
impl<K: Clone + Ord + Hash + std::fmt::Debug, V: Hash + Eq + Clone + std::fmt::Debug> Node<K, V> {
    pub fn get_subsets<I>(&self, iter_keys: I) -> Vec<(HashSet<K>, V)>
    where
        I: Iterator<Item = K>,
    {
        let mut nodes = vec![self];
        let mut results = Vec::new();
        for key in iter_keys {
            let mut new_nodes = Vec::new();
            for node in nodes {
                new_nodes.push(node);
                if let Some(child) = node.children.get(&key) {
                    new_nodes.push(child);
                    results.extend(
                        child
                            .values
                            .clone()
                            .into_iter()
                            .map(|value| (child.key.clone(), value)),
                    );
                }
            }
            nodes = new_nodes;
        }
        results
    }
}

impl<K: Ord + Hash + Clone, V: Hash + Eq + Clone> Node<K, V> {
    pub fn to_vec(&self, prefix: Vec<K>) -> Vec<(Vec<K>, V)> {
        let mut result = Vec::new();
        result.extend(
            self.values
                .iter()
                .map(|value| (prefix.clone(), value.clone())),
        );
        for (key, child) in &self.children {
            let mut new_prefix = prefix.clone();
            new_prefix.push(key.clone());
            result.extend(child.to_vec(new_prefix));
        }
        result
    }
    pub fn iter(&self) -> impl Iterator<Item = (Vec<K>, V)> {
        self.to_vec(Vec::new()).into_iter()
    }
}

impl<K: Ord + Hash + Display, V: Hash + Eq + Display> Node<K, V> {
    pub fn fmt(&self, f: &mut std::fmt::Formatter<'_>, prefix: &str) -> std::fmt::Result {
        for value in self.values.iter() {
            writeln!(f, "{} = {}", prefix, value)?;
        }
        for (key, child) in self.children.iter() {
            let new_prefix = if prefix.is_empty() {
                format!("{key}")
            } else {
                format!("{}-{}", prefix, key)
            };
            child.fmt(f, &new_prefix)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_child() {
        let mut map = Node::new();
        map.insert_child(vec!["1", "2", "3"], 123);
        map.insert_child(vec!["1", "2", "4"], 124);
        map.insert_child(vec!["1", "5", "6"], 156);
        assert_eq!(map.children.len(), 1);
        assert_eq!(map.children[&"1"].children.len(), 2);
        assert_eq!(map.children[&"1"].children[&"2"].children.len(), 2);
        assert_eq!(
            map.children[&"1"].children[&"2"].children[&"3"].values,
            HashSet::from([123])
        );
        assert_eq!(
            map.children[&"1"].children[&"2"].children[&"4"].values,
            HashSet::from([124])
        );
        assert_eq!(
            map.children[&"1"].children[&"5"].children[&"6"].values,
            HashSet::from([156])
        );
    }

    #[test]
    fn test_get_subsets() {
        let mut map = Node::new();
        map.insert_child(vec!["1", "2", "3"], 123);
        map.insert_child(vec!["1", "2", "4"], 124);
        map.insert_child(vec!["1", "5", "6"], 156);

        let keys = vec!["1", "2"];
        let results = map.get_subsets(keys.into_iter());
        assert_eq!(results, Vec::new());

        let keys = vec!["1", "2", "3", "4", "5", "6"];
        let results = map.get_subsets(keys.into_iter());
        assert_eq!(
            results,
            Vec::from([
                (HashSet::from(["1", "2", "3"]), 123),
                (HashSet::from(["1", "2", "4"]), 124),
                (HashSet::from(["1", "5", "6"]), 156)
            ])
        );
    }
}
