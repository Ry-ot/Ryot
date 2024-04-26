use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Properties {
    pub elevation: Elevation,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Serialize, Deserialize, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct Elevation(f32);

impl Display for Elevation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E:{}", self.0)
    }
}

impl From<u32> for Elevation {
    fn from(value: u32) -> Self {
        Elevation(value as f32)
    }
}
