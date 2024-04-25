use bevy_ecs::component::Component;
use glam::FloatExt;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Component, Copy, PartialEq, Serialize, Deserialize)]
pub struct Elevation {
    pub elevation: f32,
}

impl Default for Elevation {
    fn default() -> Self {
        Elevation { elevation: 0.0 }
    }
}

impl Display for Elevation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E:{}", self.elevation)
    }
}

impl Elevation {
    pub fn lerp(&self, other: &Elevation, fraction: f32) -> Elevation {
        Elevation {
            elevation: self.elevation.lerp(other.elevation, fraction),
        }
    }
}
