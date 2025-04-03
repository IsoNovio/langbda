use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    LambdaModel(#[from] super::lambda::Error),
    TreeModel(#[from] super::tree::Error),
}

impl std::error::Error for Error {}
