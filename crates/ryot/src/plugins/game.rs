use bevy_app::{App, Last, Plugin, PostUpdate, Update};
use bevy_ecs::prelude::*;
use ryot_internal::prelude::*;

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

/// `TileFlagPlugin` provides the necessary system and resource setup for managing `TileFlags`
/// within the game world. It ensures that the flag cache is up-to-date and reflects the latest
/// flag state of the whole tile, per position. This avoids the need to iterate over each entity
/// within a tile to check its properties.
pub struct TileFlagPlugin;

impl Plugin for TileFlagPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cache<TilePosition, TileFlags>>()
            .add_systems(PostUpdate, update_tile_flag_cache);
    }
}
