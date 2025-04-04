use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    NoExpectation,
    ApplyEntryToNonLambda,
    #[from]
    Syntax(crate::syntax::Error),
    QueryAndEntryTypeMismatch,
    LambdaToMustBeFeatures,
    TypeConversion,
}

impl std::error::Error for Error {}
