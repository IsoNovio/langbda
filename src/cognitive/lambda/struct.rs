use super::super::CognitiveModel;
use super::error::{Error, Result};
use super::node::Node;
use super::valid_entry::ValidEntry;
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
    fn push_lambda(&mut self, from: SyntaxValue<K>, to: Node<K>) {
        self.push(Node::Lambda {
            from,
            to: Box::new(to),
        });
    }
    fn push_lambda_features(&mut self, from: FeatureSet<K>, to: FeatureSet<K>) -> Result<bool> {
        if to.is_subset(&from) {
            self.possibly_project(&from)?;
            Ok(false)
        } else {
            self.push_lambda(SyntaxValue::from(from), Node::from(to));
            Ok(true)
        }
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
                if let Some(onto) = node.get_features_left_mut() {
                    FeatureSet::project(from, onto, &ignore)?;
                }
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
    use crate::lexicon::LexiconEntry;
    impl<K: Clone + Ord> LambdaModel<K> {
        pub fn push_lexicon_lambda(&mut self, from: ValidEntry<K>, to: Node<K>) -> Result<bool> {
            // do not add projection if insertion fails due to being subset

            match (from, to) {
                (ValidEntry::Features(from), Node::Value { value: to }) => match to {
                    SyntaxValue::Features(to) => self.push_lambda_features(from, to),
                    _ => Err(Error::LambdaToMustBeFeatures),
                },
                (ValidEntry::Features(from), to @ Node::Lambda { .. }) => {
                    let from = SyntaxValue::from(from);
                    self.push_lambda(from, to);
                    Ok(true)
                }
                (
                    ValidEntry::Lambda {
                        from: new,
                        to: from,
                        project,
                    },
                    Node::Value { value: to },
                ) => {
                    let to = match to {
                        SyntaxValue::Features(to) => to,
                        _ => Err(Error::LambdaToMustBeFeatures)?,
                    };
                    if self.push_lambda_features(from, to)? && project {
                        let ignore = new.get_features_right();
                        self.push_projection(ignore.clone());
                    }

                    let new = Node::try_from(*new)?;
                    self.push(new);
                    Ok(true)
                }
                (
                    ValidEntry::Lambda {
                        from: a,
                        to: b,
                        project: project_ab,
                    },
                    Node::Lambda { from: c, to: d },
                ) => {
                    let b = ValidEntry::from(b);
                    if self.push_lexicon_lambda(b, *d)? && project_ab {
                        let ignore = (*a).get_features_right();
                        self.push_projection(ignore.clone());
                    }

                    let a = Node::try_from(*a)?;
                    self.push_lambda(c, a);
                    Ok(true)
                }
                (_, Node::Projection { .. }) => Err(Error::LambdaToMustBeFeatures),
            }
        }
    }

    impl<K: Clone + Ord> CognitiveModel<K> for LambdaModel<K> {
        fn init(target: FeatureSet<K>) -> Self {
            let mut model = Self::new();
            let target = Node::from(target);
            model.push(target);
            model
        }

        fn understood(&self) -> bool {
            self.is_empty()
        }

        fn demand(&self) -> bool {
            match self.peek() {
                Some(node) => match node {
                    Node::Value { .. } => true,
                    Node::Lambda { to, .. } => matches!(&**to, Node::Value { .. }),
                    _ => false,
                },
                None => false,
            }
        }

        fn receive(&mut self, token: K) -> super::super::super::error::Result<()> {
            let from = SyntaxValue::from(token);
            let to = self.pop_node()?;
            self.push_lambda(from, to);
            Ok(())
        }

        fn wonder(&self) -> Option<&SyntaxValue<K>> {
            match self.peek() {
                Some(Node::Lambda { from, .. }) => Some(from),
                _ => None,
            }
        }

        fn decide(&mut self, entry: LexiconEntry<K>) -> super::super::super::error::Result<()> {
            let target = self.pop_node()?;
            match target {
                Node::Lambda {
                    from: original_from,
                    to,
                } => match (original_from, entry) {
                    (
                        SyntaxValue::Features(from_fs),
                        LexiconEntry::Functional {
                            to: from,
                            project: entry_project,
                        },
                    ) => {
                        let mut from = ValidEntry::try_from(from)?;
                        if let Some(ignore_fs) = entry_project {
                            let onto_fs = from.get_features_right_mut();
                            FeatureSet::project(&from_fs, onto_fs, &ignore_fs)
                                .map_err(Error::Syntax)?;
                        }
                        self.push_lexicon_lambda(from, *to)?;
                    }
                    (SyntaxValue::Item(_), LexiconEntry::Lexical(from)) => {
                        let from = ValidEntry::try_from(from)?;
                        self.push_lexicon_lambda(from, *to)?;
                    }
                    _ => Err(Error::QueryAndEntryTypeMismatch)?,
                },
                _ => Err(Error::ApplyEntryToNonLambda)?,
            }
            Ok(())
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
