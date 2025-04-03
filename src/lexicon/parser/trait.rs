use crate::lexicon::Lexicon;

pub trait LexiconParser<K> {
    fn parse_str(lexicon: &mut impl Lexicon<K>, input: &str) -> Result<(), String>;
}
