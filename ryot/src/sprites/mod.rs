use glam::UVec2;
use serde_repr::{Deserialize_repr, Serialize_repr};

mod config;
pub use config::*;

mod sheet_loading;
pub use sheet_loading::*;

pub mod layer;
pub use layer::Layer;

pub mod position;

pub mod error;

#[derive(Serialize_repr, Deserialize_repr, Default, PartialEq, Debug, Copy, Clone)]
#[repr(u32)]
pub enum SpriteLayout {
    #[default]
    OneByOne = 0,
    OneByTwo = 1,
    TwoByOne = 2,
    TwoByTwo = 3,
}

impl SpriteLayout {
    pub fn get_width(&self, tile_size: &UVec2) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::OneByTwo => tile_size.x,
            SpriteLayout::TwoByOne | SpriteLayout::TwoByTwo => tile_size.x * 2,
        }
    }

    pub fn get_height(&self, tile_size: &UVec2) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::TwoByOne => tile_size.y,
            SpriteLayout::OneByTwo | SpriteLayout::TwoByTwo => tile_size.y * 2,
        }
    }

    pub fn get_size(&self, tile_size: &UVec2) -> UVec2 {
        UVec2::new(self.get_width(tile_size), self.get_height(tile_size))
    }
}

#[cfg(test)]
mod tests;
