mod config;

pub use config::*;
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

mod sheet_loading;
pub use sheet_loading::*;

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(width: u32, height: u32) -> Self {
        Rect { width, height }
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u32)]
#[derive(Default)]
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
            SpriteLayout::OneByOne | SpriteLayout::OneByTwo => sheet_config.tile_size.width,
            SpriteLayout::TwoByOne | SpriteLayout::TwoByTwo => sheet_config.tile_size.width * 2,
        }
    }

    pub fn get_height(&self, sheet_config: &SpriteSheetConfig) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::TwoByOne => sheet_config.tile_size.height,
            SpriteLayout::OneByTwo | SpriteLayout::TwoByTwo => sheet_config.tile_size.height * 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SheetGrid {
    pub file: String,
    pub tile_size: Rect,
    pub columns: usize,
    pub rows: usize,
}
