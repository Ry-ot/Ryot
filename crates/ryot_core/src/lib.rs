//! `ryot_core`
//!
//! Acts as the backbone of the Ryot framework, housing essential components, systems,
//! and utilities. This core crate supports foundational game development tasks, ensuring
//! stability and efficiency across the framework.
#![feature(trait_alias)]
pub mod content;
pub mod game;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::{
        content::{
            sprite::{
                layout::{SpriteLayout, SpriteLayoutIter, TextureAtlasLayouts},
                sprite_sheet::{SpriteSheet, SpriteSheets},
                Animation, FrameGroup, SpriteInfo,
            },
            ContentId, ContentRecord, ContentType, RyotContentState, VisualElement, VisualElements,
        },
        game::{Category, Elevation, Flags, Properties},
    };

    #[cfg(feature = "bevy")]
    pub use crate::content::transition_to_ready;
}
