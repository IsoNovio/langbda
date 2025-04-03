use super::super::{Lexicon, LexiconEntry, LexiconNode};
use crate::syntax::{FeatureSet, SyntaxValue};
use crate::trie::TrieMultiMap;
use std::cmp::Ord;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

#[derive(Debug)]
pub struct SimpleLexicon<K: Debug + Clone + Ord + Hash> {
    lexical: HashMap<K, HashSet<LexiconNode<K>>>,
    functional: TrieMultiMap<(K, Option<K>), LexiconNode<K>>,
}

impl<K: Debug + Clone + Ord + Hash> SimpleLexicon<K> {
    pub fn new() -> Self {
        SimpleLexicon {
            lexical: HashMap::new(),
            functional: TrieMultiMap::new(),
        }
    }

    fn get_lexical_entries(&self, from: &K) -> HashSet<LexiconEntry<K>> {
        self.lexical
            .get(from)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|node| LexiconEntry::Lexical(node.clone()))
            .collect()
    }

    fn get_functional_entries(&self, from: &FeatureSet<K>) -> HashSet<LexiconEntry<K>> {
        self.functional
            .get_subsets(from)
            .into_iter()
            .map(|(key, value)| {
                let project = if let LexiconNode::Lambda {
                    from: _,
                    to: _,
                    project,
                } = value
                {
                    !project
                } else {
                    true
                };
                LexiconEntry::Functional {
                    from: key.into_iter().collect(),
                    to: value,
                    project,
                }
            })
            .collect()
    }
}

impl<K> Lexicon<K> for SimpleLexicon<K>
where
    K: Debug + Clone + Ord + Hash,
{
    fn add_entry(&mut self, from: SyntaxValue<K>, to: LexiconNode<K>) -> bool {
        match from {
            SyntaxValue::Item(key) => self.lexical.entry(key).or_default().insert(to),
            SyntaxValue::Features(fs) => self.functional.insert(fs, to),
        }
    }
    fn get_entries(&self, from: &SyntaxValue<K>) -> HashSet<LexiconEntry<K>> {
        match from {
            SyntaxValue::Item(k) => self.get_lexical_entries(k),
            SyntaxValue::Features(fs) => self.get_functional_entries(fs),
        }
    }
}

impl<K> Display for SimpleLexicon<K>
where
    K: Debug + Display + Clone + Ord + Hash,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Lexical entries:")?;
        for (lex, set) in self.lexical.iter() {
            for interp in set {
                writeln!(f, "  {lex} = {interp}")?;
            }
        }
        writeln!(f, "Functional:")?;
        for (func_vec, interp) in self.functional.iter() {
            let func: FeatureSet<K> = func_vec.into_iter().collect();
            writeln!(f, "  {func} = {interp}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::FeatureSet;

    use super::*;

    #[test]
    fn test_lexicon() {
        let mut lexicon: SimpleLexicon<&str> = SimpleLexicon::new();
        let a = SyntaxValue::Item("a");
        let b = SyntaxValue::Item("b");
        let c = SyntaxValue::Item("c");
        let mut ab = FeatureSet::new();
        ab.insert("a", None);
        ab.insert("b", None);
        let ab = LexiconNode::Value {
            value: SyntaxValue::Features(ab),
        };
        let mut abc = FeatureSet::new();
        abc.insert("a", None);
        abc.insert("b", None);
        abc.insert("c", None);
        let abc = LexiconNode::Value {
            value: SyntaxValue::Features(abc),
        };
        lexicon.add_entry(a.clone(), ab.clone());
        lexicon.add_entry(b.clone(), ab.clone());
        lexicon.add_entry(a.clone(), abc.clone());
        lexicon.add_entry(b.clone(), abc.clone());
        lexicon.add_entry(c.clone(), abc.clone());
        assert_eq!(
            lexicon.get_entries(&SyntaxValue::Item("a")),
            [
                LexiconEntry::Lexical(ab.clone()),
                LexiconEntry::Lexical(abc.clone())
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(
            lexicon.get_entries(&SyntaxValue::Item("b")),
            [
                LexiconEntry::Lexical(ab.clone()),
                LexiconEntry::Lexical(abc.clone())
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(
            lexicon.get_entries(&SyntaxValue::Item("c")),
            [LexiconEntry::Lexical(abc.clone())].into_iter().collect()
        );
    }
}
