use crate::feature::FeatureSet;
use crate::direction::Direction;

#[derive(Debug)]
pub enum Interpretation<'a> {
    Val(FeatureSet<'a>),
    Lambda {
        from: FeatureSet<'a>,
        to: FeatureSet<'a>,
        projection: Direction,
    },
}

impl<'a> std::fmt::Display for Interpretation<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interpretation::Val(fset) => write!(f, "{}", fset),
            Interpretation::Lambda { from, to, projection } => {
                let projection = match projection {
                    Direction::Left => ">",
                    Direction::Right => ">>",
                };
                write!(f, "{} {} {}", from, projection, to)
            }
        }
    }
}
