//! `ryot_core`
//!
//! Acts as the backbone of the Ryot framework, housing essential components, systems,
//! and utilities. This core crate supports foundational game development tasks, ensuring
//! stability and efficiency across the framework.
#![feature(trait_alias)]
pub mod content_type;
pub mod frame_group;
pub mod game;
pub mod properties;
pub mod sprite;
#[cfg(feature = "bevy")]
pub mod state;
pub mod visual_element;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::{
        content_type::ContentType,
        frame_group::FrameGroup,
        game::{elevation::Elevation, game_object::GameObjectId},
        properties::{Category, EntityType, Flags, Properties},
        sprite::{
            layout::{SpriteLayout, SpriteLayoutIter, TextureAtlasLayouts},
            sprite_sheet_data::{SpriteSheetData, SpriteSheetDataSet},
            Animation, SpriteInfo,
        },
        visual_element::{VisualElement, VisualElements},
    };

    #[cfg(feature = "bevy")]
    pub use crate::state::{transition_to_ready, RyotContentState};
}
