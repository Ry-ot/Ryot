#[cfg(feature = "bevy")]
use bevy::prelude::*;
use glam::{UVec2, Vec2};
use serde_repr::{Deserialize_repr, Serialize_repr};

mod config;
pub use config::*;

mod sheet_loading;
pub use sheet_loading::*;

pub mod layer;
pub use layer::Layer;
use strum::EnumIter;

use crate::bevy_ryot::sprites::SpriteMaterial;

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
    color: Color,
    thickness: f32,
}

#[derive(Component, Debug, Clone, PartialEq, Default)]
pub struct SpriteParams {
    pub outline: Option<SpriteOutline>,
    pub tint: Option<Color>,
}

impl SpriteParams {
    pub fn with_outline(self, color: Color, thickness: f32) -> Self {
        Self {
            outline: Some(SpriteOutline { color, thickness }),
            ..self
        }
    }

    pub fn with_tint(self, color: Color) -> Self {
        Self {
            tint: Some(color),
            ..self
        }
    }

    pub fn has_any(&self) -> bool {
        self.outline.is_some() || self.tint.is_some()
    }

    pub fn to_material(&self, base: SpriteMaterial) -> SpriteMaterial {
        let mut material = base;
        if let Some(outline) = &self.outline {
            material.outline_color = outline.color;
            material.outline_thickness = outline.thickness;
        }
        if let Some(tint) = &self.tint {
            material.tint = *tint;
        }
        material
    }
}

impl From<&SpriteMaterial> for SpriteParams {
    fn from(material: &SpriteMaterial) -> Self {
        Self {
            outline: Some(SpriteOutline {
                color: material.outline_color,
                thickness: material.outline_thickness,
            }),
            tint: Some(material.tint),
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
