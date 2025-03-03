use super::parser::SentenceParser;

#[derive(Debug, Clone)]
pub struct Sentence(Vec<String>);

impl Sentence {
    pub fn new() -> Sentence {
        Sentence(Vec::new())
    }

    pub fn from_str(s: &str) -> Result<Sentence, String> {
        SentenceParser::parse_sentence(s)
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn as_str(&self) -> Vec<&str> {
        self.0.iter().map(|s| s.as_str()).collect()
    }

    fn add_first_token(&mut self, token: &str)
    {
        fn is_all_caps(s: &str) -> bool {
            s.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
        }

        if !is_all_caps(token) {
            let mut chars = token.chars();
            let lowered = match chars.next() {
                None => String::new(),
                Some(c) => c.to_lowercase().to_string() + chars.as_str(),
            };

            self.0.push(lowered);
        }
    }

    pub fn add_token<'b>(&mut self, token: &str) {
        if self.size() == 0 {
            self.add_first_token(token);
        } else {
            self.0.push(token.to_string())
        }
    }
}
