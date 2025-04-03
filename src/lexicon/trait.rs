use super::{LexiconEntry, LexiconNode};
use crate::syntax::SyntaxValue;
use std::collections::HashSet;

pub trait Lexicon<K> {
    fn add_entry(&mut self, from: SyntaxValue<K>, to: LexiconNode<K>) -> bool;
    fn get_entries(&self, from: &SyntaxValue<K>) -> HashSet<LexiconEntry<K>>;
}
