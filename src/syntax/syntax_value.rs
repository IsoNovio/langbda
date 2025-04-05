use super::FeatureSet;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SyntaxValue<K> {
    Item(K),
    Features(FeatureSet<K>),
}

impl<K> From<K> for SyntaxValue<K> {
    fn from(item: K) -> Self {
        SyntaxValue::Item(item)
    }
}

impl<K> From<FeatureSet<K>> for SyntaxValue<K> {
    fn from(features: FeatureSet<K>) -> Self {
        SyntaxValue::Features(features)
    }
}

impl<'a, K> TryInto<&'a FeatureSet<K>> for &'a SyntaxValue<K> {
    type Error = super::Error;
    fn try_into(self) -> Result<&'a FeatureSet<K>, Self::Error> {
        match self {
            SyntaxValue::Features(fs) => Ok(fs),
            SyntaxValue::Item(_) => Err(Self::Error::TypeConversion),
        }
    }
}

impl<K> TryInto<FeatureSet<K>> for SyntaxValue<K> {
    type Error = super::Error;
    fn try_into(self) -> Result<FeatureSet<K>, Self::Error> {
        match self {
            SyntaxValue::Features(fs) => Ok(fs),
            SyntaxValue::Item(_) => Err(Self::Error::TypeConversion),
        }
    }
}

impl<K: Display> Display for SyntaxValue<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxValue::Item(item) => write!(f, "{}", item),
            SyntaxValue::Features(features) => write!(f, "{}", features),
        }
    }
}
