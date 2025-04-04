use crate::syntax::{FeatureSet, SyntaxValue};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LexiconNode<K> {
    Value {
        value: SyntaxValue<K>,
    },
    Lambda {
        from: Box<LexiconNode<K>>,
        to: Box<LexiconNode<K>>,
        project: bool,
    },
    Moved {
        from: FeatureSet<K>,
    },
}

impl<K> Display for LexiconNode<K>
where
    K: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexiconNode::Value { value } => write!(f, "{}", value),
            LexiconNode::Lambda { from, to, project } => {
                let project = if *project { ">>" } else { ">" };
                write!(f, "({from} {project} {to})")
            }
            LexiconNode::Moved { from } => write!(f, "MOVED({from})"),
        }
    }
}
