
use crate::{feature::FeatureSet, syntax_node::SyntaxNode};
use super::interpretation::Interpretation;
use super::parser::LexiconParser;

use multimap::MultiMap;

#[derive(Debug)]
pub struct Lexicon<'a>(MultiMap<SyntaxNode<'a>, Interpretation<'a>>);

impl<'a> Lexicon<'a> {
    pub fn new() -> Lexicon<'a> {
        Lexicon(MultiMap::new())
    }

    pub fn insert(&mut self, node: SyntaxNode<'a>, interp: Interpretation<'a>) {
        self.0.insert(node, interp);
    }

    pub fn insert_lexical(&mut self, lexi: &'a str, features: FeatureSet<'a>) {
        self.insert(SyntaxNode::Lexi(lexi), Interpretation::Val(features));
    }

    pub fn insert_functional(&mut self, fset: FeatureSet<'a>, interp: Interpretation<'a>) {
        self.insert(SyntaxNode::Feat(fset), interp);
    }

    pub fn get_vec(&self, node: &SyntaxNode<'a>) -> Option<&Vec<Interpretation<'a>>> {
        self.0.get_vec(node)
    }

    pub fn iter_all(&self) -> multimap::IterAll<SyntaxNode<'a>, Vec<Interpretation<'a>>> {
        self.0.iter_all()
    }

    pub fn from_str(input: &'a str) -> Result<Lexicon<'a>, String> {
        LexiconParser::parse_lexicon(&input)
    }
}


impl<'a> std::fmt::Display for Lexicon<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lexical = Vec::new();
        let mut functional = Vec::new();
        for (node, interp) in self.0.flat_iter() {
            match node {
                SyntaxNode::Lexi(lexi) => lexical.push(format!("{} = {}", lexi, interp)),
                SyntaxNode::Feat(fset) => functional.push(format!("{} = {}", fset, interp)),
            }
        }
        lexical.sort();
        functional.sort();
        write!(f, "[Functional]\n{}\n\n[Lexical]\n{}\n", functional.join("\n"), lexical.join("\n"))
    }
}
