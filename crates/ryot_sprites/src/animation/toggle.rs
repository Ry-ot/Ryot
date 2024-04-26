use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::Resource;

/// A resource to enable/disable sprite animation globally.
#[derive(Resource, PartialEq, Debug, Clone)]
pub struct SpriteAnimationEnabled(pub bool);

impl Default for SpriteAnimationEnabled {
    fn default() -> Self {
        Self(true)
    }
}

pub fn toggle_sprite_animation(mut enabled: ResMut<SpriteAnimationEnabled>) {
    enabled.0 = !enabled.0;
}
