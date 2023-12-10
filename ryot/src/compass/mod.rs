mod generator;
mod plan;
mod serde;

pub use plan::*;
pub use generator::{build_map};
pub use serde::{types::*};

pub enum KeyOption {
    None,
    Position(Position),
    PositionWithOwner(Position, u8),
}

impl KeyOption {
    pub fn get_key_bytes(&self) -> Vec<u8> {
        match self {
            KeyOption::None => {vec![]},
            KeyOption::Position(position) => {
                let mut key = Vec::with_capacity(5);
                key.extend_from_slice(&position.x.to_be_bytes());
                key.extend_from_slice(&position.y.to_be_bytes());
                key.push(position.z);
                key
            },
            KeyOption::PositionWithOwner(position, owner) => {
                let mut key = KeyOption::Position(position.clone()).get_key_bytes();
                key.push(*owner);
                key
            }
        }
    }

    pub fn get_key_str(&self) -> String {
        match self {
            KeyOption::None => {"".to_string()},
            KeyOption::Position(position) => {
                format!("{}:{}:{}", position.x, position.y, position.z)
            },
            KeyOption::PositionWithOwner(position, owner) => {
                format!("{}:{}", KeyOption::Position(position.clone()).get_key_str(), owner)
            }
        }
    }
}
