use glam::UVec2;
use serde_repr::{Deserialize_repr, Serialize_repr};

mod config;
pub use config::*;

mod sheet_loading;
pub use sheet_loading::*;

pub mod layer;

pub mod position;

pub mod error;

#[derive(Serialize_repr, Deserialize_repr, Default, PartialEq, Debug, Clone)]
#[repr(u32)]
pub enum SpriteLayout {
    #[default]
    OneByOne = 0,
    OneByTwo = 1,
    TwoByOne = 2,
    TwoByTwo = 3,
}

impl SpriteLayout {
    pub fn get_width(&self, sheet_config: &SpriteSheetConfig) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::OneByTwo => sheet_config.tile_size.x,
            SpriteLayout::TwoByOne | SpriteLayout::TwoByTwo => sheet_config.tile_size.x * 2,
        }
    }

    pub fn get_height(&self, sheet_config: &SpriteSheetConfig) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::TwoByOne => sheet_config.tile_size.y,
            SpriteLayout::OneByTwo | SpriteLayout::TwoByTwo => sheet_config.tile_size.y * 2,
        }
    }

    pub fn get_size(&self, sheet_config: &SpriteSheetConfig) -> UVec2 {
        UVec2::new(self.get_width(sheet_config), self.get_height(sheet_config))
    }
}

#[cfg(test)]
mod tests;
