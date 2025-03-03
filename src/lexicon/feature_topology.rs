use crate::feature::Feature;

use std::collections::HashMap;
use multimap::MultiMap;

#[derive(Debug)]
pub struct FeatureTopology<'a> {
    cat_map: MultiMap<&'a str, &'a str>,
    val_map: HashMap<&'a str, &'a str>
}

impl<'a> FeatureTopology<'a> {
    pub fn new() -> FeatureTopology<'a> {
        FeatureTopology {
            cat_map: MultiMap::new(),
            val_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, category: &'a str, value: &'a str) {
        self.cat_map.insert(category, value);
        self.val_map.insert(value, category);
    }

    pub fn is_category(&self, category: &'a str) -> bool {
        self.cat_map.contains_key(category)
    }

    pub fn get_from_category(&self, category: &'a str) -> Option<&Vec<&'a str>> {
        self.cat_map.get_vec(category)
    }

    pub fn get_from_value(&self, value: &'a str) -> Option<&'a str> {
        self.val_map.get(value).map(|c| *c)
    }

    pub fn to_feature(&self, s: &'a str) -> Feature<'a> {
        if let Some(feature) = self.get_from_value(s) {
            Feature::new_both(feature, s)
        } else if self.is_category(s) {
            Feature::new_category(s)
        } else {
            Feature::new_value(s)
        }
    }
}

