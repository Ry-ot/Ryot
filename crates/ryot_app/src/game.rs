use bevy_app::{App, Last, Plugin, Update};
use bevy_ecs::prelude::*;

use ryot_sprites::SpriteSystems;
use ryot_tiled::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadObjects>().add_plugins(ElevationPlugin);
    }
}

pub struct ElevationPlugin;

impl Plugin for ElevationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                initialize_elevation.in_set(SpriteSystems::Initialize),
                apply_elevation,
            ),
        )
        .add_systems(Last, track_position_changes);
    }
}
