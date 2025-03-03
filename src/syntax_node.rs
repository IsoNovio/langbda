
use crate::feature::{Feature, FeatureSet};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxNode<'a> {
    Lexi(&'a str),
    Feat(FeatureSet<'a>)
}

impl<'a> SyntaxNode<'a> {
    pub fn from_lexical_str(lstr: &'a str) -> SyntaxNode<'a> {
        SyntaxNode::Lexi(lstr)
    }

    pub fn from_feature_set(fset: FeatureSet<'a>) -> SyntaxNode<'a> {
        SyntaxNode::Feat(fset)
    }

    pub fn from_feature_str(fstr: &'a str) -> SyntaxNode<'a> {
        let f = Feature::new_value(fstr);
        let fs: FeatureSet<'_> = FeatureSet::from_feature(f);
        SyntaxNode::Feat(fs)
    }

    pub fn is_subset(&self, other: &SyntaxNode<'a>) -> bool {
        match (self, other) {
            (SyntaxNode::Lexi(l1), SyntaxNode::Lexi(l2)) => l1 == l2,
            (SyntaxNode::Feat(f1), SyntaxNode::Feat(f2)) => f1.is_subset(f2),
            _ => false,
        }
    }
}

impl<'a> std::fmt::Display for SyntaxNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxNode::Lexi(lexi) => write!(f, "{}", lexi),
            SyntaxNode::Feat(fset) => write!(f, "{}", fset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax_node_subset() {
        let mut f1 = FeatureSet::new();
        f1.insert("N", None);
        f1.insert("number", Some("Sg"));
        let l1 = SyntaxNode::from_feature_set(f1);

        let mut f2 = FeatureSet::new();
        f2.insert("N", None);
        let l2 = SyntaxNode::from_feature_set(f2);

        assert!(l2.is_subset(&l1));
    }
}
