use bevy_app::{App, Last, Plugin, PostUpdate, Update};
use bevy_ecs::prelude::*;
use ryot_internal::prelude::*;
use std::marker::PhantomData;

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

/// `NavigablePlugin` provides the necessary system and resource setup for managing `Navigable`
/// within the game world. It ensures that the flag cache is up-to-date and reflects the latest
/// flag state of the whole tile, per position. This avoids the need to iterate over each entity
/// within a tile to check its properties.
pub struct NavigablePlugin<N: Navigable + Copy + Default + Component>(PhantomData<N>);

impl<N: Navigable + Copy + Default + Component> Default for NavigablePlugin<N> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<N: Navigable + Copy + Default + Component> Plugin for NavigablePlugin<N> {
    fn build(&self, app: &mut App) {
        app.init_resource_once::<Cache<TilePosition, N>>()
            .add_systems(PostUpdate, update_tile_flag_cache::<N>);
    }
}
