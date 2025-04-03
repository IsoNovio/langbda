use crate::syntax::{FeatureSet, SyntaxValue};

#[derive(Debug, Clone)]
pub enum Node<K> {
    Value {
        value: FeatureSet<K>,
    },
    Lambda {
        from: SyntaxValue<K>,
        to: FeatureSet<K>,
    },
    Projection {
        ignore: FeatureSet<K>,
    },
}
