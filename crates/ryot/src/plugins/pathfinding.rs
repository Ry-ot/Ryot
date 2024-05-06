use bevy_app::{App, Plugin};
use ryot_internal::prelude::*;

pub struct PathFindingPlugin;

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_pathable::<TilePosition, Flags>();
    }
}
