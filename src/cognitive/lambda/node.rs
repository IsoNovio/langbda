use crate::syntax::{FeatureSet, SyntaxValue};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Node<K> {
    Value {
        value: SyntaxValue<K>,
    },
    Lambda {
        from: SyntaxValue<K>,
        to: Box<Node<K>>,
    },
    Projection {
        ignore: FeatureSet<K>,
    },
}

impl<K> Node<K> {
    pub fn get_features_left_mut(&mut self) -> Option<&mut FeatureSet<K>> {
        match self {
            Node::Value {
                value: SyntaxValue::Features(fs),
            } => Some(fs),
            Node::Lambda {
                from: SyntaxValue::Features(fs),
                ..
            } => Some(fs),
            _ => None,
        }
    }
}

impl<K> From<FeatureSet<K>> for Node<K> {
    fn from(fs: FeatureSet<K>) -> Self {
        Node::Value {
            value: SyntaxValue::Features(fs),
        }
    }
}

use super::valid_entry::ValidEntry;
impl<K> TryFrom<ValidEntry<K>> for Node<K> {
    type Error = super::Error;

    fn try_from(value: ValidEntry<K>) -> Result<Self, Self::Error> {
        match value {
            ValidEntry::Features(fs) => Ok(Node::from(fs)),
            ValidEntry::Lambda { from, to, .. } => {
                let from = ValidEntry::try_into(*from)?;
                let to = Box::new(Node::from(to));
                Ok(Node::Lambda { from, to })
            }
        }
    }
}

impl<K: Display> Display for Node<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Value { value } => {
                write!(f, "{}", value)
            }
            Node::Lambda { from, to } => {
                write!(f, "λ({from} -> {to})")
            }
            Node::Projection { ignore } => {
                write!(f, ">>(ignore: {ignore})")
            }
        }
    }
}
