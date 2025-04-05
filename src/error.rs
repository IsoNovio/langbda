use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum Error {
    Interpreter(crate::interpreter::Error),
    Cognitive(crate::cognitive::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}
