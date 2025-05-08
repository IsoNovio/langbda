use crate::lexicon::Lexicon;
use crate::tokenizer::Tokenizer;

pub trait Dialect: Default {
    type Token;

    fn name(&self) -> &str;
    fn lexicon(&self) -> &impl Lexicon<Self::Token>;
    fn tokenizer(&self) -> &impl Tokenizer<Self::Token>;
}
