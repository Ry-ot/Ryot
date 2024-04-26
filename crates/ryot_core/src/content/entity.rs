use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, Default, PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[repr(usize)]
pub enum ContentType {
    #[default]
    Object,
    Outfit,
    Effect,
    Missile,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy_reflect::Reflect, bevy_ecs::component::Component)
)]
pub enum ContentId {
    Object(u32),
    Outfit(u32),
    Effect(u32),
    Missile(u32),
}

impl Default for ContentId {
    fn default() -> Self {
        ContentId::Object(0)
    }
}

impl ContentId {
    pub fn is_none(&self) -> bool {
        self.get_id() == 0
    }

    pub fn get_id(&self) -> u32 {
        match self {
            ContentId::Object(id) => *id,
            ContentId::Outfit(id) => *id,
            ContentId::Effect(id) => *id,
            ContentId::Missile(id) => *id,
        }
    }

    pub fn as_group_and_id(&self) -> Option<(ContentType, u32)> {
        match self {
            ContentId::Object(id) => Some((ContentType::Object, *id)),
            ContentId::Outfit(id) => Some((ContentType::Outfit, *id)),
            ContentId::Effect(id) => Some((ContentType::Effect, *id)),
            ContentId::Missile(id) => Some((ContentType::Missile, *id)),
        }
    }

    pub fn group(&self) -> Option<ContentType> {
        self.as_group_and_id().map(|(group, _)| group)
    }

    pub fn id(&self) -> Option<u32> {
        self.as_group_and_id().map(|(_, id)| id)
    }
}

impl fmt::Display for ContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentId::Object(id) => write!(f, "Object({})", id),
            ContentId::Outfit(id) => write!(f, "Outfit({})", id),
            ContentId::Effect(id) => write!(f, "Effect({})", id),
            ContentId::Missile(id) => write!(f, "Missile({})", id),
        }
    }
}
