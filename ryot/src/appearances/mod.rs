//! This module contains the structs and enums that are used to load the graphical content
//! of a OT based game, with assets from the client's content folder.
//! The content folder is the folder that contains the content-catalog.json file and the
//! files that are listed in it.
//! The content-catalog.json file is a list of resource files that the client needs to load.
//! This module also wraps the protobuf representation of the appearances.dat file.
//! See appearances.proto and the auto generated appearances.rs file for more information.
include!(concat!(env!("OUT_DIR"), "/appearances.rs"));

use crate::{SpriteLayout, SpriteSheetConfig};
use glam::UVec2;
use serde::{Deserialize, Serialize};

/// Those are the available contents within the content-catalog.json file
/// There are 5 known types of content: appearances, staticdata, staticmapdata, map and sprite.
/// Even though we deserialize all the 5 types, we only use the sprites and appearances in this
/// library.
///
/// Example:
/// ```json
/// [
///    {
///      "type": "appearances",
///      "file": "appearances.dat",
///      "version": 1
///    },
///    {
///      "type": "staticdata",
///      "file": "staticdata.dat"
///    },
///    {
///      "type": "staticmapdata",
///      "file": "staticmapdata.dat"
///    },
///    {
///      "type": "map",
///      "file": "map.otbm"
///    },
///    {
///       "type": "sprite",
///       "file": "spritesheet.png",
///       "spritetype": 0,
///       "firstspriteid": 100,
///       "lastspriteid": 200,
///       "area": 64
///     }
/// ]
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ContentType {
    #[serde(rename = "appearances")]
    Appearances { file: String, version: u32 },
    #[serde(rename = "staticdata")]
    StaticData { file: String },
    #[serde(rename = "staticmapdata")]
    StaticMapData { file: String },
    #[serde(rename = "map")]
    Map { file: String },
    #[serde(rename = "sprite")]
    Sprite(SpriteSheetData),
}

/// This is the content of the Sprite ContentType. It contains the information needed
/// to load the sprite sheet and individual sprites from it.
/// A sprite sheet is defined by:
/// - a lzma compressed bmp file
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
    pub fn get_tile_size(&self, sheet_config: &SpriteSheetConfig) -> UVec2 {
        let width = self.layout.get_width(sheet_config);
        let height = self.layout.get_height(sheet_config);
        UVec2::new(width, height)
    }

    /// Returns the position of a sprite in the sprite sheet.
    /// The position is calculated based on the tile size and the sprite sheet config.
    pub fn get_columns_count(&self, sheet_config: &SpriteSheetConfig) -> usize {
        let tile_size = self.get_tile_size(sheet_config);
        (sheet_config.sheet_size.x / tile_size.x) as usize
    }

    /// Returns the position of a sprite in the sprite sheet.
    /// The position is calculated based on the tile size and the sprite sheet config.
    pub fn get_rows_count(&self, sheet_config: &SpriteSheetConfig) -> usize {
        let tile_size = self.get_tile_size(sheet_config);
        (sheet_config.sheet_size.y / tile_size.y) as usize
    }
}

/// This is a collection of sprite sheets.
/// It contains the sprite sheets and the sprite sheet config.
/// The sprite sheet config is used to calculate the position and size of a sprite in the sprite
/// sheet.
#[derive(Debug, Default, Clone)]
pub struct SpriteSheetDataSet {
    pub data: Vec<SpriteSheetData>,
    pub config: SpriteSheetConfig,
}

impl SpriteSheetDataSet {
    /// Creates a new SpriteSheetSet from a list of ContentType and a SpriteSheetConfig.
    /// The ContentType list is filtered to only contain the Sprite ContentType.
    /// The SpriteSheetConfig is a reference to a game specific config that contains the
    /// information needed to calculate the position and size of a sprite in the sprite sheet.
    pub fn from_content(content: &[ContentType], sheet_config: &SpriteSheetConfig) -> Self {
        let sprite_sheets = content
            .iter()
            .filter_map(|content_type| match content_type {
                ContentType::Sprite(sprite_sheet) => Some(sprite_sheet),
                _ => None,
            })
            .cloned()
            .collect::<Vec<_>>();

        Self {
            data: sprite_sheets,
            config: *sheet_config,
        }
    }

    /// Checks if any of the sprite sheets contains the given sprite id.
    /// Returns true if the sprite id is in any of the sprite sheets.
    /// Returns false if the sprite id is not in any of the sprite sheets.
    pub fn has_sprite(&self, sprite_id: u32) -> bool {
        self.data.iter().any(|sheet| sheet.has_sprite(sprite_id))
    }

    /// Returns the sprite sheet that contains the given sprite id.
    /// Returns None if the sprite id is not in any of the sprite sheets.
    pub fn get_by_sprite_id(&self, sprite_id: u32) -> Option<&SpriteSheetData> {
        self.data.iter().find(|sheet| sheet.has_sprite(sprite_id))
    }

    /// Returns the index of a given sprite id in one of the sprite sheets.
    /// The index is the position of the sprite in the sprite sheet, starting from 0.
    /// Returns None if the sprite id is not in any of the sprite sheets.
    pub fn get_sprite_index_by_id(&self, sprite_id: u32) -> Option<usize> {
        self.get_by_sprite_id(sprite_id)?
            .get_sprite_index(sprite_id)
    }
}

#[cfg(test)]
mod tests;
