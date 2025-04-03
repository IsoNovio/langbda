use super::LexiconNode;
use crate::syntax::FeatureSet;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexiconEntry<K> {
    Lexical(LexiconNode<K>),
    Functional {
        from: FeatureSet<K>,
        to: LexiconNode<K>,
        project: bool,
    },
}

impl<K: Display> Display for LexiconEntry<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexiconEntry::Lexical(node) => write!(f, "token = {}", node),
            LexiconEntry::Functional { from, to, project } => {
                if *project {
                    write!(f, "{} => {}", from, to)
                } else {
                    write!(f, "{} = {}", from, to)
                }
            }
        }
    }
}
