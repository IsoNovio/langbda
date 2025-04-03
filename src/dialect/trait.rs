use crate::lexicon::Lexicon;
use crate::tokenizer::Tokenizer;

pub trait Dialect<K>: Default {
    fn name(&self) -> &str;
    fn lexicon(&self) -> &impl Lexicon<K>;
    fn tokenizer(&self) -> &impl Tokenizer<K>;
}
