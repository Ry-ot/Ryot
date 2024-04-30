//! `ryot_core`
//!
//! Acts as the backbone of the Ryot framework, housing essential components, systems,
//! and utilities. This core crate supports foundational game development tasks, ensuring
//! stability and efficiency across the framework.
#![feature(trait_alias)]
pub mod content;
pub mod game;

pub mod prelude {
    pub use crate::{
        content::{
            record::{Category, ContentRecord, Flags, VisualElement, VisualElements},
            sprite::{
                layout::{SpriteLayout, SpriteLayoutIter, TextureAtlasLayouts},
                sprite_sheet::SpriteSheet,
                Animation, FrameGroup, SpriteInfo,
            },
            ContentId, ContentType, RyotContentState,
        },
        game::{Elevation, Navigable, Point, Properties},
    };

    #[cfg(feature = "bevy")]
    pub use crate::content::transition_to_ready;
}

#[cfg(test)]
mod tests;
