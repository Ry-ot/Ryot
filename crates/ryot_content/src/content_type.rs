use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// Ryot expects the sprite sheets to be cataloged in a JSON file. This file contains a list of
/// elements of type `SpriteSheetData` that represents the sprite sheet information.
/// This struct is a default representation of this catalog file and ignores the other fields
/// that your JSON might have.
///
/// You can use your own json struct to represent the catalog file, as long as it implements
/// the Into<Option<SpriteSheetData>> + Clone.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::TypePath))]
#[serde(tag = "type")]
pub enum ContentType {
    #[serde(rename = "sprite")]
    Sprite(SpriteSheetData),
    #[serde(other, rename = "unknown")]
    Unknown,
}

impl From<ContentType> for Option<SpriteSheetData> {
    fn from(content: ContentType) -> Self {
        match content {
            ContentType::Sprite(sprite_sheet) => Some(sprite_sheet),
            _ => None,
        }
    }
}
