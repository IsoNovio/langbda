use super::error::{Error, Result};
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FeatureSet<K> {
    map: BTreeMap<K, Option<K>>,
}

impl<K> Default for FeatureSet<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K> FeatureSet<K> {
    pub fn new() -> Self {
        FeatureSet {
            map: BTreeMap::new(),
        }
    }
}

impl<K> FeatureSet<K>
where
    K: Clone + Ord,
{
    pub fn from_category(category: K) -> Self {
        FeatureSet {
            map: BTreeMap::from([(category, None)]),
        }
    }
}

impl<K> FeatureSet<K>
where
    K: Clone + Ord,
{
    pub fn get(&self, key: &K) -> Option<&Option<K>> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: K, value: Option<K>) {
        self.map.insert(key, value);
    }

    pub fn remove(&mut self, key: &K) {
        self.map.remove(key);
    }

    pub fn insert_if_absent(&mut self, key: K, value: Option<K>) -> Result<()> {
        match self.get(&key) {
            Some(_) => Err(Error::CategoryAlreadyHasValue),
            None => {
                self.insert(key, value);
                Ok(())
            }
        }
    }

    pub fn contains_key_value(&self, key: &K, value: &Option<K>) -> bool {
        match self.get(key) {
            Some(v) => v == value,
            None => false,
        }
    }

    pub fn remove_key_value(&mut self, key: &K, value: &Option<K>) {
        if self.contains_key_value(key, value) {
            self.remove(key);
        }
    }

    pub fn project(
        from: &FeatureSet<K>,
        onto: &mut FeatureSet<K>,
        ignore: &FeatureSet<K>,
    ) -> Result<()> {
        for (category, value) in from.map.iter() {
            if !ignore.contains_key_value(category, value) {
                onto.insert_if_absent(category.clone(), value.clone())?;
            }
        }
        Ok(())
    }

    pub fn is_subset(&self, other: &FeatureSet<K>) -> bool {
        for (category, value) in self.map.iter() {
            if !other.contains_key_value(category, value) {
                return false;
            }
        }
        true
    }
}

impl<K> FeatureSet<K> {
    pub fn iter(&self) -> FeatureSetIter<K> {
        FeatureSetIter {
            inner: self.map.iter(),
        }
    }
}
#[derive(Clone)]
pub struct FeatureSetIter<'a, K> {
    inner: std::collections::btree_map::Iter<'a, K, Option<K>>,
}
impl<'a, K> Iterator for FeatureSetIter<'a, K> {
    type Item = (&'a K, &'a Option<K>);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
impl<'a, K> IntoIterator for &'a FeatureSet<K> {
    type Item = (&'a K, &'a Option<K>);
    type IntoIter = FeatureSetIter<'a, K>;
    fn into_iter(self) -> Self::IntoIter {
        FeatureSetIter {
            inner: self.map.iter(),
        }
    }
}
impl<K> IntoIterator for FeatureSet<K> {
    type Item = (K, Option<K>);
    type IntoIter = std::collections::btree_map::IntoIter<K, Option<K>>;
    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
impl<K> FromIterator<(K, Option<K>)> for FeatureSet<K>
where
    K: Ord,
{
    fn from_iter<I: IntoIterator<Item = (K, Option<K>)>>(iter: I) -> Self {
        FeatureSet {
            map: BTreeMap::from_iter(iter),
        }
    }
}

use std::fmt::Display;
impl<K> Display for FeatureSet<K>
where
    K: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut entries = Vec::new();

        for (category, value) in self.iter() {
            match value {
                Some(value) => entries.push(format!("{category}:{value}")),
                None => entries.push(format!("{category}")),
            }
        }

        entries.sort();
        write!(f, "{}", entries.join("--"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_feature_set() {
        let mut from = FeatureSet::new();
        from.insert("a", Some("b"));
        from.insert("c", Some("d"));
        let mut ignore = FeatureSet::new();
        ignore.insert("c", Some("d"));
        let mut onto = FeatureSet::new();

        FeatureSet::project(&from, &mut onto, &ignore).unwrap();
        let mut expected = FeatureSet::new();
        expected.insert("a", Some("b"));
        assert_eq!(onto, expected);
    }

    #[test]
    fn project_feature_set_conflict() {
        let mut from = FeatureSet::new();
        from.insert("a", Some("b"));
        from.insert("c", Some("d"));
        let ignore = FeatureSet::new();
        let mut onto = FeatureSet::new();
        onto.insert("c", Some("e"));
        assert!(FeatureSet::project(&from, &mut onto, &ignore).is_err());
    }

    #[test]
    fn print_feature_set() {
        use crate::interner::GlobalKey;
        use std::str::FromStr;
        let a = GlobalKey::from_str("a").unwrap();
        let b = GlobalKey::from_str("b").unwrap();
        let c = GlobalKey::from_str("c").unwrap();
        let d = GlobalKey::from_str("d").unwrap();
        let mut feature_set = FeatureSet::new();
        feature_set.insert(a, Some(b));
        feature_set.insert(c, Some(d));
        assert_eq!(format!("{feature_set}"), "a:b--c:d");
    }
}
