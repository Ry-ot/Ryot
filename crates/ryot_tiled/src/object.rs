use crate::prelude::{Layer, TilePosition};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::event::Event;
use bevy_utils::default;
use derive_more::*;
use ryot_content::prelude::{Elevation, GameObjectId};

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
