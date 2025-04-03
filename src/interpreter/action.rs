use crate::lexicon::LexiconEntry;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action<K: Clone> {
    AddToken(K),
    ApplyEntry(LexiconEntry<K>),
}

impl<K: Display + Clone> Display for Action<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::AddToken(node) => write!(f, "Add token [{}]", node),
            Action::ApplyEntry(entry) => write!(f, "Apply entry [{}]", entry),
        }
    }
}
