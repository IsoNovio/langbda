use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    NotFeatureSet,
    NoExpectation,
    ApplyEntryToNonLambda,
    #[from]
    Syntax(crate::syntax::Error),
}

impl std::error::Error for Error {}
