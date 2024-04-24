use crate::prelude::*;
use bevy::prelude::*;
use ryot_tiled::prelude::*;

pub mod tile_flags;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadObjects>().add_plugins(ElevationPlugin);
    }
}

#[derive(Bundle, Copy, Clone, Debug)]
pub struct GameObjectBundle {
    pub object_id: GameObjectId,
    pub position: TilePosition,
    pub elevation: Elevation,
    pub layer: Layer,
}

impl GameObjectBundle {
    pub fn new(object_id: GameObjectId, position: TilePosition, layer: Layer) -> Self {
        Self {
            object_id,
            position,
            layer,
            elevation: default(),
        }
    }
}

#[derive(Event, Clone, Debug, Deref, DerefMut)]
pub struct LoadObjects(pub Vec<GameObjectBundle>);
