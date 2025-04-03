pub trait Tokenizer<K> {
    fn tokenize<'a>(&self, input: &'a str) -> Vec<(K, &'a str)>;
}
