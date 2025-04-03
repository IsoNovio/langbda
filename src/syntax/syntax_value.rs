use super::FeatureSet;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SyntaxValue<K> {
    Item(K),
    Features(FeatureSet<K>),
}

impl<K: Display> Display for SyntaxValue<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxValue::Item(item) => write!(f, "{}", item),
            SyntaxValue::Features(features) => write!(f, "{}", features),
        }
    }
}
