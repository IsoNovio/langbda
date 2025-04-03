use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct FeatureTopology<K> {
    cat_map: HashMap<K, HashSet<K>>,
    val_map: HashMap<K, K>,
}

impl<K> FeatureTopology<K> {
    pub fn new() -> FeatureTopology<K> {
        FeatureTopology {
            cat_map: HashMap::new(),
            val_map: HashMap::new(),
        }
    }
}

impl<K> FeatureTopology<K>
where
    K: Clone + Eq + Hash,
{
    pub fn insert(&mut self, category: K, value: K) {
        self.cat_map
            .entry(category.clone())
            .or_default()
            .insert(value.clone());
        self.val_map.insert(value, category);
    }

    pub fn is_category(&self, category: &K) -> bool {
        self.cat_map.contains_key(category)
    }

    pub fn get_from_category(&self, category: &K) -> Option<&HashSet<K>> {
        self.cat_map.get(category)
    }

    pub fn get_from_value(&self, value: &K) -> Option<K> {
        self.val_map.get(value).cloned()
    }
}
