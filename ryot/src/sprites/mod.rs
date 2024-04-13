#[cfg(feature = "bevy")]
use crate::bevy_ryot::sprites::SpriteMaterial;
#[cfg(feature = "bevy")]
use bevy::prelude::*;
use glam::{UVec2, Vec2};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::EnumIter;

mod config;
pub use config::*;

mod sheet_loading;
pub use sheet_loading::*;

pub mod layer;
pub use layer::Layer;

pub mod position;

pub mod error;

#[derive(
    Serialize_repr, Deserialize_repr, Default, Eq, PartialEq, Debug, Copy, Clone, EnumIter, Hash,
)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[repr(u32)]
pub enum SpriteLayout {
    #[default]
    OneByOne = 0,
    OneByTwo = 1,
    TwoByOne = 2,
    TwoByTwo = 3,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpriteOutline {
    #[cfg(feature = "bevy")]
    color: Color,
    thickness: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub struct SpriteParams {
    pub alpha: Option<f32>,
    pub outline: Option<SpriteOutline>,
    #[cfg(feature = "bevy")]
    pub tint: Option<Color>,
}

impl SpriteParams {
    #[cfg(feature = "bevy")]
    pub fn with_outline(self, color: Color, thickness: f32) -> Self {
        Self {
            outline: Some(SpriteOutline { color, thickness }),
            ..self
        }
    }

    #[cfg(not(feature = "bevy"))]
    pub fn with_outline(self, thickness: f32) -> Self {
        Self {
            outline: Some(SpriteOutline { thickness }),
            ..self
        }
    }

    #[cfg(feature = "bevy")]
    pub fn with_tint(self, color: Color) -> Self {
        Self {
            tint: Some(color),
            ..self
        }
    }

    pub fn with_alpha(self, alpha: f32) -> Self {
        Self {
            alpha: Some(alpha),
            ..self
        }
    }

    #[cfg(feature = "bevy")]
    pub fn has_any(&self) -> bool {
        self.outline.is_some() || self.tint.is_some() || self.alpha.is_some()
    }

    #[cfg(not(feature = "bevy"))]
    pub fn has_any(&self) -> bool {
        self.outline.is_some() || self.alpha.is_some()
    }

    #[cfg(feature = "bevy")]
    pub fn to_material(&self, base: SpriteMaterial) -> SpriteMaterial {
        let mut material = base;

        if let Some(outline) = &self.outline {
            material.outline_color = outline.color;
            material.outline_thickness = outline.thickness;
        }

        #[cfg(not(feature = "bevy"))]
        if let Some(tint) = &self.tint {
            material.tint = *tint;
        }

        if let Some(alpha) = &self.alpha {
            material.alpha = *alpha;
        }

        material
    }
}

#[cfg(feature = "bevy")]
impl From<&SpriteMaterial> for SpriteParams {
    fn from(material: &SpriteMaterial) -> Self {
        Self {
            outline: Some(SpriteOutline {
                color: material.outline_color,
                thickness: material.outline_thickness,
            }),
            tint: Some(material.tint),
            alpha: Some(material.alpha),
        }
    }
}

impl SpriteLayout {
    pub fn get_width(&self, tile_size: &UVec2) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::OneByTwo => tile_size.x,
            SpriteLayout::TwoByOne | SpriteLayout::TwoByTwo => tile_size.x * 2,
        }
    }

    pub fn get_height(&self, tile_size: &UVec2) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::TwoByOne => tile_size.y,
            SpriteLayout::OneByTwo | SpriteLayout::TwoByTwo => tile_size.y * 2,
        }
    }

    pub fn get_size(&self, tile_size: &UVec2) -> UVec2 {
        UVec2::new(self.get_width(tile_size), self.get_height(tile_size))
    }

    pub fn get_counts(&self, sheet_size: Vec2, tile_size: Vec2) -> Vec2 {
        let width = sheet_size.x / tile_size.x;
        let height = sheet_size.y / tile_size.y;
        match self {
            SpriteLayout::OneByOne => Vec2::new(width, height),
            SpriteLayout::OneByTwo => Vec2::new(width, height / 2.),
            SpriteLayout::TwoByOne => Vec2::new(width / 2., height),
            SpriteLayout::TwoByTwo => Vec2::new(width / 2., height / 2.),
        }
    }
}

#[cfg(test)]
mod tests;
