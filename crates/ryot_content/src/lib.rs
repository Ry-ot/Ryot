#![feature(trait_alias)]
pub mod content_type;
pub mod frame_group;
pub mod properties;
#[cfg(feature = "bevy")]
pub mod state;
pub mod visual_element;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::{
        content_type::ContentType,
        frame_group::FrameGroup,
        properties::{Category, EntityType, Flags, Properties},
        visual_element::{VisualElement, VisualElements},
    };

    #[cfg(feature = "bevy")]
    pub use crate::state::{transition_to_ready, RyotContentState};
}
