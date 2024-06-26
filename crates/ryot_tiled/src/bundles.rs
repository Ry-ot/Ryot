use crate::prelude::{Layer, TilePosition};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::event::Event;
use bevy_utils::default;
use derive_more::*;
use ryot_core::prelude::{ContentId, Elevation};

#[derive(Bundle, Copy, Clone, Debug)]
pub struct TiledContentBundle {
    pub object_id: ContentId,
    pub position: TilePosition,
    pub elevation: Elevation,
    pub layer: Layer,
}

impl TiledContentBundle {
    pub fn new(object_id: ContentId, position: TilePosition, layer: Layer) -> Self {
        Self {
            object_id,
            position,
            layer,
            elevation: default(),
        }
    }
}

#[derive(Event, Clone, Debug, Deref, DerefMut)]
pub struct LoadObjects(pub Vec<TiledContentBundle>);
