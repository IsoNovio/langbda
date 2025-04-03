use crate::syntax::{FeatureSet, SyntaxValue};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LexiconNode<K> {
    Value {
        value: SyntaxValue<K>,
    },
    Lambda {
        from: Box<LexiconNode<K>>,
        to: FeatureSet<K>,
        project: bool,
    },
    Moved {
        from: FeatureSet<K>,
    },
}

impl<K> LexiconNode<K> {
    pub fn get_features(&self, rightmost: Option<bool>) -> Option<&FeatureSet<K>> {
        match self {
            LexiconNode::Value {
                value: SyntaxValue::Features(fs),
            } => Some(fs),
            LexiconNode::Moved { from } => Some(from),
            LexiconNode::Lambda {
                from,
                to,
                project: _,
            } => match rightmost {
                Some(false) => from.get_features(rightmost),
                Some(true) => Some(to),
                None => None,
            },
            _ => None,
        }
    }
    pub fn get_features_mut(&mut self, rightmost: Option<bool>) -> Option<&mut FeatureSet<K>> {
        match self {
            LexiconNode::Value {
                value: SyntaxValue::Features(fs),
            } => Some(fs),
            LexiconNode::Moved { from } => Some(from),
            LexiconNode::Lambda {
                from,
                to,
                project: _,
            } => match rightmost {
                Some(false) => from.get_features_mut(rightmost),
                Some(true) => Some(to),
                None => None,
            },
            _ => None,
        }
    }
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
