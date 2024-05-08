use bevy_app::prelude::*;
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

/// `NavigableApp` inserts a `Navigable` to the game world, providing the necessary system and
/// resource setup for managing it. It ensures that the flag cache is up-to-date and reflects the
/// latest flag state of the whole tile, per position. This avoids the need to iterate over each
/// entity within a tile to check its properties.
pub trait NavigableApp {
    fn add_navigable<N: Navigable + Copy + Default + Component>(&mut self) -> &mut Self;
}

impl NavigableApp for App {
    fn add_navigable<N: Navigable + Copy + Default + Component>(&mut self) -> &mut Self {
        self.init_resource_once::<Cache<TilePosition, N>>()
            .add_systems(
                PostUpdate,
                collect_updatable_positions
                    .pipe(build_new_flags_for_map::<N>)
                    .pipe(update_tile_flag_cache::<N>)
                    .in_set(CacheSystems::UpdateCache),
            )
    }
}
