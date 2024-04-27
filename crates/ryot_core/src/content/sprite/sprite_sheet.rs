use crate::prelude::SpriteLayout;
use glam::UVec2;
use serde::{Deserialize, Serialize};

/// This is the content of the Sprite Content. It contains the information needed
/// to load the sprite sheet and individual sprites from it.
/// A sprite sheet is defined by:
/// - a sprite file (that can be compressed or not)
/// - a sprite layout (1:1, 1:2, 2:1 or 2:2)
/// - the ids of first and last sprites in the sheet, to determine which sprites are in the sheet
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpriteSheet {
    pub file: String,
    #[serde(rename = "spritetype")]
    pub layout: SpriteLayout,
    #[serde(rename = "firstspriteid")]
    pub first_sprite_id: u32,
    #[serde(rename = "lastspriteid")]
    pub last_sprite_id: u32,
    pub area: u32,
}

impl SpriteSheet {
    /// Checks if the sprite sheet contains the given sprite id
    pub fn has_sprite(&self, sprite_id: u32) -> bool {
        self.first_sprite_id <= sprite_id && self.last_sprite_id >= sprite_id
    }

    /// Returns the index of a given sprite id in the sprite sheet.
    /// The index is the position of the sprite in the sprite sheet, starting from 0.
    /// The index is used to calculate the position of the sprite in the sprite sheet.
    /// If the sprite id is not in the sprite sheet, None is returned.
    pub fn get_sprite_index(&self, sprite_id: u32) -> Option<usize> {
        if self.has_sprite(sprite_id) {
            Some((sprite_id - self.first_sprite_id) as usize)
        } else {
            None
        }
    }

    /// Returns the size of a sprite in the sprite sheet.
    /// The size is calculated based on the sprite layout and the sprite sheet config.
    pub fn get_tile_size(&self, tile_size: &UVec2) -> UVec2 {
        let width = self.layout.get_width(tile_size);
        let height = self.layout.get_height(tile_size);
        UVec2::new(width, height)
    }
}
