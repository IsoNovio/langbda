use derive_more::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display)]
pub enum Error {
    CategoryAlreadyHasValue,
}

impl std::error::Error for Error {}
