include!(concat!(env!("OUT_DIR"), "/appearances.rs"));

use crate::SpriteLayout;
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
