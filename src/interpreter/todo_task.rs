use crate::feature::FeatureSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TodoTask<'a> {
    Val(usize),
    Lambda {
        from: usize,
        to: usize,
    },
    Project {
        from: usize,
        to: usize,
        ignore: &'a FeatureSet<'a>,
    },
}

impl<'a> std::fmt::Display for TodoTask<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoTask::Val(idx) => write!(f, "{}", idx),
            TodoTask::Lambda { from, to } => write!(f, "({}, {})", from, to),
            TodoTask::Project { from, to, ignore: _ } => write!(f, "<{}, {}>", from, to),
        }
    }
}
