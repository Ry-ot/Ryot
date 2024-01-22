use std::fmt::{Display, Formatter};
use std::result;

#[derive(Debug)]
pub enum Error {
    DatabaseError(String),
}

#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
impl From<heed::Error> for Error {
    fn from(e: heed::Error) -> Self {
        Error::DatabaseError(e.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;
