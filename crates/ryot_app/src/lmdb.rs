use bevy_app::{App, Plugin, Startup};
use bevy_ecs::prelude::*;
use ryot_tiled::prelude::*;

pub struct LmdbPlugin;

impl Plugin for LmdbPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LmdbEnv>()
            .init_resource::<LmdbCompactor>()
            .add_systems(Startup, init_tiles_db.map(drop));
    }
}
