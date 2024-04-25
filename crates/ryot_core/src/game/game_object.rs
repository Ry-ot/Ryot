use crate::prelude::EntityType;
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord,
)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy_reflect::Reflect, bevy_ecs::component::Component)
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
