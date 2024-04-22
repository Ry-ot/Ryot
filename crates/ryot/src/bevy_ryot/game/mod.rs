use self::elevation::{apply_elevation, Elevation};
use crate::prelude::*;
use bevy::prelude::*;
use core::fmt;
use ryot_tiled::prelude::*;
use serde::{Deserialize, Serialize};

pub mod elevation;
pub mod tile_flags;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadObjects>()
            .add_systems(Update, apply_elevation)
            .add_systems(Last, track_position_changes);
    }
}

#[derive(
    Component,
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    Default,
    Reflect,
    PartialOrd,
    Ord,
)]
pub enum GameObjectId {
    #[default]
    None,
    Object(u32),
    Outfit(u32),
    Effect(u32),
    Missile(u32),
}

impl GameObjectId {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn as_group_and_id(&self) -> Option<(EntityType, u32)> {
        match self {
            GameObjectId::None => None,
            GameObjectId::Object(id) => Some((EntityType::Object, *id)),
            GameObjectId::Outfit(id) => Some((EntityType::Outfit, *id)),
            GameObjectId::Effect(id) => Some((EntityType::Effect, *id)),
            GameObjectId::Missile(id) => Some((EntityType::Missile, *id)),
        }
    }

    pub fn group(&self) -> Option<EntityType> {
        self.as_group_and_id().map(|(group, _)| group)
    }

    pub fn id(&self) -> Option<u32> {
        self.as_group_and_id().map(|(_, id)| id)
    }
}

impl fmt::Display for GameObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameObjectId::None => write!(f, "None"),
            GameObjectId::Object(id) => write!(f, "Object({})", id),
            GameObjectId::Outfit(id) => write!(f, "Outfit({})", id),
            GameObjectId::Effect(id) => write!(f, "Effect({})", id),
            GameObjectId::Missile(id) => write!(f, "Missile({})", id),
        }
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
