/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use std::fmt::{Display, Formatter};
use std::result;

#[derive(Debug)]
pub enum Error {
    DatabaseError(String),
}

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
