use super::sentence::Sentence;

use pest::Parser;
#[allow(unused_imports)]
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sentence/sentence.pest"]
pub struct SentenceParser;

impl SentenceParser {
    pub fn parse_sentence(input: &str) -> Result<Sentence, String> {
        match SentenceParser::parse(Rule::sentence, input) {
            Ok(pairs) => Ok(parse_sentence(pairs)?),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

fn parse_sentence(pairs: Pairs<Rule>) -> Result<Sentence, String> {
    let mut sentence = Sentence::new();
    for pair in pairs {
        sentence.add_token(pair.as_str());
    }
    Ok(sentence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pest_sentence() {
        let sentence = "Whose naughty child ate the apple in the room?";
        let pest_result = SentenceParser::parse(Rule::sentence, sentence).unwrap();
        println!("{:#?}", pest_result);
    }

    #[test]
    fn parse_sentence() {
        let input = "Whose naughty child ate the apple in the room?";
        let sentence = SentenceParser::parse_sentence(input).unwrap();
        println!("{:?}", sentence);
    }
}
