use bevy_ecs::event::Event;
use ryot_core::content::ContentId;
use ryot_core::prelude::FrameGroup;

#[cfg(feature = "debug")]
pub mod debug;
pub mod loaded;
pub mod systems;

#[derive(Event)]
pub struct LoadAppearanceEvent {
    pub object_id: ContentId,
    pub frame_group: FrameGroup,
}
