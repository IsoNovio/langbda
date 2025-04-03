use super::Dialect;
use crate::interner::GlobalKey;
use crate::lexicon::SimpleLexicon;
use crate::lexicon::parser::{LexiconParser, PestLexiconParser};
use crate::tokenizer::SimpleTokenizer;
use std::fmt::Display;

#[derive(Debug)]
pub struct English {
    name: String,
    lexicon: SimpleLexicon<GlobalKey>,
    tokenizer: SimpleTokenizer,
}

impl Dialect<GlobalKey> for English {
    fn name(&self) -> &str {
        &self.name
    }
    fn lexicon(&self) -> &impl crate::lexicon::Lexicon<GlobalKey> {
        &self.lexicon
    }
    fn tokenizer(&self) -> &impl crate::tokenizer::Tokenizer<GlobalKey> {
        &self.tokenizer
    }
}

impl English {
    pub fn new() -> Self {
        Self {
            name: "English".to_string(),
            lexicon: SimpleLexicon::new(),
            tokenizer: SimpleTokenizer,
        }
    }
    pub fn init() -> Self {
        let mut dialect = Self::new();
        let lexicon_str = include_str!("../../assets/lexicons/en.lexicon");
        PestLexiconParser::parse_str(&mut dialect.lexicon, lexicon_str).unwrap();
        dialect
    }
}

impl Default for English {
    fn default() -> Self {
        Self::init()
    }
}

impl Display for English {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        write!(f, "{}", self.lexicon)
    }
}
