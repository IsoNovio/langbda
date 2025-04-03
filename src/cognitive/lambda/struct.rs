use super::super::CognitiveModel;
use super::error::{Error, Result};
use super::node::Node;
use crate::syntax::{FeatureSet, SyntaxValue};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct LambdaModel<K> {
    expects: Vec<Node<K>>,
}

impl<K: Clone + Ord> LambdaModel<K> {
    fn new() -> Self {
        Self {
            expects: Vec::new(),
        }
    }
    fn is_empty(&self) -> bool {
        self.expects.is_empty()
    }
    fn push(&mut self, expect: Node<K>) {
        self.expects.push(expect);
    }
    fn push_value(&mut self, value: FeatureSet<K>) {
        self.push(Node::Value { value });
    }
    fn push_lambda(&mut self, from: SyntaxValue<K>, to: FeatureSet<K>) {
        self.push(Node::Lambda { from, to });
    }
    fn push_lambda_item(&mut self, from: K, to: FeatureSet<K>) {
        let from = SyntaxValue::Item(from);
        self.push_lambda(from, to);
    }
    fn push_lambda_features(&mut self, from: FeatureSet<K>, to: FeatureSet<K>) -> Result<()> {
        if to.is_subset(&from) {
            self.possibly_project(&from)?;
        } else {
            let from = SyntaxValue::Features(from);
            self.push_lambda(from, to);
        }
        Ok(())
    }
    fn push_projection(&mut self, ignore: FeatureSet<K>) {
        self.push(Node::Projection { ignore });
    }
    fn peek(&self) -> Option<&Node<K>> {
        self.expects.last()
    }
    fn peek_mut(&mut self) -> Option<&mut Node<K>> {
        self.expects.last_mut()
    }
    fn pop(&mut self) -> Option<Node<K>> {
        self.expects.pop()
    }
    fn pop_node(&mut self) -> Result<Node<K>> {
        self.pop().ok_or(Error::NoExpectation)
    }
    fn possibly_project(&mut self, from: &FeatureSet<K>) -> Result<()> {
        if let Some(Node::Projection { .. }) = self.peek() {
            let ignore = match self.pop() {
                Some(Node::Projection { ignore }) => ignore,
                _ => unreachable!("Already checked that expect stack top is Projection"),
            };

            if let Some(node) = self.peek_mut() {
                let onto = match node {
                    Node::Value { value } => value,
                    Node::Lambda {
                        from: SyntaxValue::Features(fs),
                        ..
                    } => fs,
                    _ => return Err(Error::NotFeatureSet),
                };
                FeatureSet::project(from, onto, &ignore)?;
            }
        }
        Ok(())
    }
}

impl<K: Clone + Ord> Default for LambdaModel<K> {
    fn default() -> Self {
        Self::new()
    }
}

/// interface with lexicon
mod lexicon {
    use super::*;
    use crate::lexicon::{LexiconEntry, LexiconNode};
    impl<K: Clone + Ord> LambdaModel<K> {
        pub fn push_lexicon_lambda(
            &mut self,
            from: LexiconNode<K>,
            to: FeatureSet<K>,
        ) -> Result<()> {
            match from {
                LexiconNode::Value { value } => match value {
                    SyntaxValue::Item(from) => {
                        self.push_lambda_item(from, to);
                    }
                    SyntaxValue::Features(from) => {
                        self.push_lambda_features(from, to)?;
                    }
                },

                LexiconNode::Lambda {
                    from: from_from,
                    to: from_to,
                    project: from_project,
                } => {
                    self.push_lambda_features(from_to, to)?;
                    if from_project {
                        let ignore = from_from
                            .get_features(Some(true))
                            .cloned()
                            .unwrap_or_default();
                        self.push_projection(ignore);
                    }
                    self.push_lexicon_node(*from_from)?;
                }
                LexiconNode::Moved { from } => {
                    self.push_lambda_features(from, to)?;
                }
            };
            Ok(())
        }

        fn push_lexicon_node(&mut self, node: LexiconNode<K>) -> Result<()> {
            match node {
                LexiconNode::Value { value } => match value {
                    SyntaxValue::Item(_) => {
                        return Err(Error::NotFeatureSet);
                    }
                    SyntaxValue::Features(fs) => {
                        self.push_value(fs);
                    }
                },
                LexiconNode::Moved { from } => {
                    self.push_value(from);
                }
                LexiconNode::Lambda {
                    from,
                    to,
                    project: _,
                } => {
                    self.push_lexicon_lambda(*from, to)?;
                }
            }
            Ok(())
        }
    }

    impl<K: Clone + Ord> CognitiveModel<K> for LambdaModel<K> {
        fn init(target: FeatureSet<K>) -> Self {
            let mut model = Self::new();
            model.push_value(target);
            model
        }
        fn understood(&self) -> bool {
            self.is_empty()
        }
        fn receive(&mut self, token: K) -> super::super::super::error::Result<()> {
            if let Node::Value { value } = self.pop_node()? {
                self.push_lambda(SyntaxValue::Item(token), value);
                Ok(())
            } else {
                Err(Error::NotFeatureSet)?
            }
        }
        fn wonder(&self) -> Option<&SyntaxValue<K>> {
            match self.peek() {
                Some(Node::Lambda { from, .. }) => Some(from),
                _ => None,
            }
        }
        /// entry_from is a subset of target_from
        /// replace target_from with target_to
        fn decide(&mut self, entry: LexiconEntry<K>) -> super::super::super::error::Result<()> {
            let target = self.pop_node()?;
            match target {
                Node::Lambda {
                    from: target_from,
                    to: target_to,
                } => {
                    let entry_to = match entry {
                        LexiconEntry::Functional {
                            from: entry_from,
                            to: mut entry_to,
                            project: entry_project,
                        } => {
                            if entry_project {
                                if let SyntaxValue::Features(target_from) = target_from {
                                    if let Some(entry_onto) = entry_to.get_features_mut(Some(true))
                                    {
                                        FeatureSet::project(&target_from, entry_onto, &entry_from)
                                            .map_err(Error::Syntax)?;
                                    }
                                }
                            };
                            entry_to
                        }
                        LexiconEntry::Lexical(entry) => entry,
                    };
                    self.push_lexicon_lambda(entry_to, target_to)?;
                    Ok(())
                }
                _ => Err(Error::ApplyEntryToNonLambda)?,
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
                write!(f, "λ({} -> {})", from, to)
            }
            Node::Projection { ignore } => {
                write!(f, ">>(ignore: {})", ignore)
            }
        }
    }
}

impl<K: Display> Display for LambdaModel<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = self
            .expects
            .iter()
            .map(|expect| format!("{}", expect))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", formatted)
    }
}
