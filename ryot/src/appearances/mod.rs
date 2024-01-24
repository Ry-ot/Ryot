include!(concat!(env!("OUT_DIR"), "/appearances.rs"));

use crate::{SpriteLayout, SpriteSheetConfig};
use glam::UVec2;
use serde::{Deserialize, Serialize};

/// Those are the available contents within the content-catalog.json file
/// The content-catalog.json file is a list of all the files that the client needs to load.
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
    Sprite(SpriteSheet),
}

/// This is the content of the Sprite ContentType. It contains the information needed
/// to load the sprite sheet and individual sprites from it.
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
    pub fn has_sprite(&self, sprite_id: u32) -> bool {
        self.first_sprite_id <= sprite_id && self.last_sprite_id >= sprite_id
    }

    pub fn get_sprite_index(&self, sprite_id: u32) -> Option<u32> {
        if self.has_sprite(sprite_id) {
            Some(sprite_id - self.first_sprite_id)
        } else {
            None
        }
    }

    pub fn get_tile_size(&self, sheet_config: &SpriteSheetConfig) -> UVec2 {
        let width = self.layout.get_width(sheet_config);
        let height = self.layout.get_height(sheet_config);
        UVec2::new(width, height)
    }

    pub fn get_columns_count(&self, sheet_config: &SpriteSheetConfig) -> usize {
        let tile_size = self.get_tile_size(sheet_config);
        (sheet_config.sheet_size.x / tile_size.x) as usize
    }

    pub fn get_rows_count(&self, sheet_config: &SpriteSheetConfig) -> usize {
        let tile_size = self.get_tile_size(sheet_config);
        (sheet_config.sheet_size.y / tile_size.y) as usize
    }
}

pub struct SpriteSheetSet {
    pub sheet_config: SpriteSheetConfig,
    pub sprite_sheets: Vec<SpriteSheet>,
}

impl SpriteSheetSet {
    pub fn has_sprite(&self, sprite_id: u32) -> bool {
        self.sprite_sheets
            .iter()
            .any(|sheet| sheet.has_sprite(sprite_id))
    }

    pub fn get_by_sprite_id(&self, sprite_id: u32) -> Option<&SpriteSheet> {
        self.sprite_sheets
            .iter()
            .find(|sheet| sheet.has_sprite(sprite_id))
    }

    pub fn get_sprite_index_by_id(&self, sprite_id: u32) -> Option<u32> {
        self.get_by_sprite_id(sprite_id)?
            .get_sprite_index(sprite_id)
    }
}

#[cfg(test)]
mod tests;
