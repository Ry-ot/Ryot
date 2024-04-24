use bevy::prelude::*;
use ryot_tiled::prelude::*;

pub mod tile_flags;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadObjects>().add_plugins(ElevationPlugin);
    }
}
