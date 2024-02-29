use crate::prelude::drawing::DrawingInfo;
use crate::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadObjects>();
    }
}

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GameObjectId {
    Object(u32),
    Outfit(u32),
}

impl From<GameObjectId> for AppearanceDescriptor {
    fn from(id: GameObjectId) -> Self {
        match id {
            GameObjectId::Object(id) => AppearanceDescriptor::object(id),
            GameObjectId::Outfit(_) => panic!("Outfit is not supported yet"),
        }
    }
}

#[derive(Bundle, Copy, Clone, Debug)]
pub struct GameObjectBundle {
    pub object_id: GameObjectId,
    pub position: TilePosition,
    pub layer: Layer,
}

impl GameObjectBundle {
    pub fn new(object_id: GameObjectId, position: TilePosition, layer: Layer) -> Self {
        Self {
            object_id,
            position,
            layer,
        }
    }
}

impl From<GameObjectBundle> for DrawingInfo {
    fn from(bundle: GameObjectBundle) -> Self {
        (
            bundle.position,
            bundle.layer,
            Visibility::Visible,
            Some(bundle.object_id.into()),
        )
    }
}

#[derive(Event, Clone, Debug, Deref, DerefMut)]
pub struct LoadObjects(pub Vec<GameObjectBundle>);
