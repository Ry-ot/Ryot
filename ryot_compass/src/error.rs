use std::result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
    #[error("Database error: {0}")]
    DatabaseError(#[from] heed::Error),
}

pub type Result<T> = result::Result<T, Error>;
