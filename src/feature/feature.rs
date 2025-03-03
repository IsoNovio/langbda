#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Feature<'a> {
    Cat(&'a str),
    Val(&'a str),
    Feat {
        category: &'a str,
        value: &'a str,
    }
}

impl<'a> Feature<'a> {
    pub fn new_both(category: &'a str, value: &'a str) -> Feature<'a> {
        Feature::Feat { category: category, value: value }
    }

    pub fn new_category(category: &'a str) -> Feature<'a> {
        Feature::Cat(category)
    }

    pub fn new_value(value: &'a str) -> Feature<'a> {
        Feature::Val(value)
    }
}

impl<'a> std::fmt::Display for Feature<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Feature::Cat(c) => write!(f, "{}", c),
            Feature::Val(v) => write!(f, "{}", v),
            Feature::Feat { category, value } => write!(f, "{}:{}", category, value),
        }
    }
}