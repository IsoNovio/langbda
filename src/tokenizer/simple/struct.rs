use super::super::Tokenizer;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct SimpleTokenizer;

impl<K> Tokenizer<K> for SimpleTokenizer
where
    K: FromStr + Copy,
{
    fn tokenize<'a>(&self, input: &'a str) -> Vec<(K, &'a str)> {
        let input = input.trim_start();

        if input.is_empty() {
            return Vec::new();
        }

        let (token_str, remainder) = if input
            .chars()
            .next()
            .expect("impossible: empty input")
            .is_ascii_punctuation()
        {
            let char_len = input
                .chars()
                .next()
                .expect("impossible: empty input")
                .len_utf8();
            (&input[..char_len], &input[char_len..])
        } else {
            match input.find(|c: char| c.is_whitespace() || c.is_ascii_punctuation()) {
                Some(pos) => (&input[..pos], &input[pos..]),
                None => (input, ""),
            }
        };
        if token_str.is_empty() {
            return Vec::new();
        }
        let k = match K::from_str(token_str) {
            Ok(k) => k,
            Err(_) => return Vec::new(),
        };
        vec![(k, remainder)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interner::GlobalKey;

    #[test]
    fn test_simple_tokenizer() {
        let input = "hello world";
        let tokenizer = SimpleTokenizer;
        let tokens: Vec<(GlobalKey, &str)> = tokenizer.tokenize(input);
        assert_eq!(tokens[0].0, GlobalKey::from_str("hello").unwrap());
    }
}
