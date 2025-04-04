use crate::lexicon::{
    LexiconNode,
    LexiconNode::{Lambda, Moved, Value},
};
use crate::syntax::{FeatureSet, SyntaxValue::Features};

pub enum ValidEntry<K> {
    Features(FeatureSet<K>),
    Lambda {
        from: Box<ValidEntry<K>>,
        to: FeatureSet<K>,
        project: bool,
    },
}

impl<K> ValidEntry<K> {
    pub fn get_features_right(&self) -> &FeatureSet<K> {
        match self {
            ValidEntry::Features(fs) => fs,
            ValidEntry::Lambda { to, .. } => to,
        }
    }
    pub fn get_features_right_mut(&mut self) -> &mut FeatureSet<K> {
        match self {
            ValidEntry::Features(fs) => fs,
            ValidEntry::Lambda { to, .. } => to,
        }
    }
}

impl<K> From<FeatureSet<K>> for ValidEntry<K> {
    fn from(fs: FeatureSet<K>) -> Self {
        ValidEntry::Features(fs)
    }
}

use crate::syntax::SyntaxValue;
impl<K> TryInto<SyntaxValue<K>> for ValidEntry<K> {
    type Error = super::Error;
    fn try_into(self) -> Result<SyntaxValue<K>, Self::Error> {
        match self {
            ValidEntry::Features(fs) => Ok(Features(fs)),
            ValidEntry::Lambda { .. } => Err(Self::Error::TypeConversion),
        }
    }
}

impl<K> TryFrom<LexiconNode<K>> for ValidEntry<K> {
    type Error = super::Error;

    fn try_from(node: LexiconNode<K>) -> Result<Self, Self::Error> {
        let err = Self::Error::TypeConversion;
        match node {
            Value {
                value: Features(fs),
            } => Ok(ValidEntry::Features(fs)),
            Moved { from } => Ok(ValidEntry::Features(from)),
            Lambda { from, to, project } => {
                let from = Self::try_from(*from)?;
                let to = match *to {
                    Value {
                        value: Features(fs),
                    } => fs,
                    Moved { from } => from,
                    _ => Err(err)?,
                };
                Ok(ValidEntry::Lambda {
                    from: Box::new(from),
                    to,
                    project,
                })
            }
            _ => Err(err),
        }
    }
}
