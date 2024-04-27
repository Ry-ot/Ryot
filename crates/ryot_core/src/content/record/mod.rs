use crate::prelude::*;
use serde::{Deserialize, Serialize};

mod category;
pub use category::Category;

mod flags;
pub use flags::Flags;

mod visual_element;
pub use visual_element::{VisualElement, VisualElements};

/// Ryot expects the sprite sheets to be cataloged in a JSON file. This file contains a list of
/// elements of type `SpriteSheet` that represents the sprite sheet information.
/// This struct is a default representation of this catalog file and ignores the other fields
/// that your JSON might have.
///
/// You can use your own json struct to represent the catalog file, as long as it implements
/// the Into<Option<SpriteSheet>> + Clone.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::TypePath))]
#[serde(tag = "type")]
pub enum ContentRecord {
    #[serde(rename = "sprite")]
    SpriteSheet(SpriteSheet),
    #[serde(other, rename = "unknown")]
    Unknown,
}

impl From<ContentRecord> for Option<SpriteSheet> {
    fn from(content: ContentRecord) -> Self {
        match content {
            ContentRecord::SpriteSheet(sprite_sheet) => Some(sprite_sheet),
            _ => None,
        }
    }
}
