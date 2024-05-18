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

#[cfg(feature = "bevy")]
pub use bevy_render::color::Color;

#[cfg(not(feature = "bevy"))]
pub enum Color {
    Rgba {
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    },
    Hsla {
        hue: f32,
        saturation: f32,
        lightness: f32,
        alpha: f32,
    },
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Serialize, Deserialize, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct Colorize(pub [Color; 4]);

#[derive(Debug, Clone, Default, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub enum Addons {
    #[default]
    None = 0,
    One = 1,
    Two = 2,
    All = 3,
}

impl From<Addons> for u32 {
    fn from(layers: Addons) -> u32 {
        layers as u32
    }
}

impl From<u32> for Addons {
    fn from(layers: u32) -> Self {
        match layers {
            1 => Addons::One,
            2 => Addons::Two,
            3 => Addons::All,
            _ => Addons::None,
        }
    }
}
