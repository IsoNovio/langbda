use super::LexiconNode;
use crate::syntax::FeatureSet;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexiconEntry<K> {
    Lexical(LexiconNode<K>),
    Functional {
        to: LexiconNode<K>,
        project: Option<FeatureSet<K>>,
    },
}

impl<K: Display> Display for LexiconEntry<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexiconEntry::Lexical(node) => write!(f, "token = {}", node),
            LexiconEntry::Functional { to, project: _ } => write!(f, "interpretation = {}", to),
        }
    }
}
