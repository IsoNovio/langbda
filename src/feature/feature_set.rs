use super::feature::Feature;

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeatureSet<'a>(BTreeMap<&'a str, Option<&'a str>>);

impl<'a> FeatureSet<'a> {
    pub fn new() -> FeatureSet<'a> {
        FeatureSet(BTreeMap::new())
    }

    pub fn from_feature(feature: Feature<'a>) -> FeatureSet<'a> {
        let mut fs = FeatureSet::new();
        fs.insert_feature(feature).unwrap();
        fs
    }

    pub fn contains_key_value(&self, key: &'a str, value: Option<&'a str>) -> bool {
        match self.0.get(key) {
            Some(v) => v == &value,
            None => false,
        }
    }

    pub fn remove(&mut self, key: &'a str) {
        self.0.remove(key);
    }

    pub fn remove_key_value(&mut self, key: &'a str, value: Option<&'a str>) {
        if self.contains_key_value(key, value) {
            self.remove(key);
        }
    }

    pub fn insert(&mut self, key: &'a str, value: Option<&'a str>) {
        self.0.insert(key, value);
    }

    pub fn insert_if_absent(&mut self, key: &'a str, value: Option<&'a str>) -> Result<(), String> {
        match self.0.get(key) {
            Some(v) => {
                Err(format!("Feature {} already exists", key))
            }
            None => {
                self.0.insert(key, value);
                Ok(())
            },
        }
    }

    pub fn insert_feature(&mut self, feature: Feature<'a>) -> Result<(), String> {
        match feature {
            Feature::Cat(c) => Err(format!("Inserted feature cannot be a category: {}", c)),
            Feature::Val(v) => self.insert_if_absent(v, None),
            Feature::Feat { category, value } => self.insert_if_absent(category, Some(value)),
        }
    }

    pub fn project(from: &FeatureSet<'a>, onto: &mut FeatureSet<'a>, ignore: &FeatureSet<'a>) -> Result<(), String> {
        for (&category, &value) in from.0.iter() {
            if !ignore.contains_key_value(category, value) {
                onto.insert_if_absent(category, value)?;
            }
        }
        Ok(())
    }

    pub fn is_subset(&self, other: &FeatureSet<'a>) -> bool {
        for (&category, &value) in self.0.iter() {
            if !other.contains_key_value(category, value) {
                return false;
            }
        }
        true
    }

    pub fn iter(&'a self) -> std::collections::btree_map::Iter<'a, &'a str, Option<&'a str>> {
        self.0.iter()
    }
}

impl<'a> std::fmt::Display for FeatureSet<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut entries = Vec::new();

        for (category, value) in self.0.iter() {
            if let Some(value) = value {
                entries.push(format!("{}:{}", category, value))
            } else {
                entries.push(format!("{}", category))
            }
        }
        
        write!(f, "{}", entries.join(", "))
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
        assert_eq!(onto, FeatureSet::from_feature(Feature::new_both("a", "b")));
    }

    #[test]
    fn project_feature_set_conflict() {
        let mut from = FeatureSet::new();
        from.insert("a", Some("b"));
        from.insert("c", Some("d"));
        let ignore: FeatureSet<'_> = FeatureSet::new();
        let mut onto = FeatureSet::new();
        onto.insert("c", Some("e"));
        assert!(FeatureSet::project(&from, &mut onto, &ignore).is_err());
    }
}
