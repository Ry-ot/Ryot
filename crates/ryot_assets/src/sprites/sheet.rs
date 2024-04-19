use crate::prelude::SpriteLayout;
use glam::UVec2;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// This is the content of the Sprite ContentType. It contains the information needed
/// to load the sprite sheet and individual sprites from it.
/// A sprite sheet is defined by:
/// - a sprite file (that can be compressed or not)
/// - a sprite layout (1:1, 1:2, 2:1 or 2:2)
/// - the ids of first and last sprites in the sheet, to determine which sprites are in the sheet
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpriteSheetData {
    pub file: String,
    #[serde(rename = "spritetype")]
    pub layout: SpriteLayout,
    #[serde(rename = "firstspriteid")]
    pub first_sprite_id: u32,
    #[serde(rename = "lastspriteid")]
    pub last_sprite_id: u32,
    pub area: u32,
}

impl SpriteSheetData {
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

/// This is a collection of sprite sheets.
/// It contains the sprite sheets and the sprite sheet config.
/// The sprite sheet config is used to calculate the position and size of a sprite in the sprite
/// sheet.
#[derive(Debug, Default, Clone)]
pub struct SpriteSheetDataSet(Vec<SpriteSheetData>);

impl Deref for SpriteSheetDataSet {
    type Target = Vec<SpriteSheetData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<&[T]> for SpriteSheetDataSet
where
    T: Into<Option<SpriteSheetData>> + Clone,
{
    fn from(content: &[T]) -> Self {
        let sprite_sheets = content
            .iter()
            .filter_map(|content_type| content_type.clone().into())
            .collect::<Vec<_>>();

        Self(sprite_sheets)
    }
}

impl SpriteSheetDataSet {
    /// Returns the sprite sheet that contains the given sprite id.
    /// Returns None if the sprite id is not in any of the sprite sheets.
    pub fn get_by_sprite_id(&self, sprite_id: u32) -> Option<&SpriteSheetData> {
        self.iter().find(|sheet| sheet.has_sprite(sprite_id))
    }
}
