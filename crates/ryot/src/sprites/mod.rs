use crate::bevy_ryot::elevation::Elevation;
#[cfg(feature = "bevy")]
use crate::bevy_ryot::sprites::SpriteMaterial;
#[cfg(feature = "bevy")]
use bevy::prelude::*;
use glam::Vec2;
use ryot_grid::prelude::*;

use ryot_assets::prelude::SpriteLayout;

pub mod position;

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

pub fn elevate_position(
    position: &TilePosition,
    layout: SpriteLayout,
    layer: Layer,
    elevation: Elevation,
) -> Vec3 {
    let anchor = Vec2::new(
        elevation.elevation.clamp(0.0, 1.0),
        (-elevation.elevation).clamp(-1.0, 0.0),
    );
    position.to_vec3(&layer)
        - (SpriteLayout::OneByOne.get_size(&tile_size()).as_vec2() * anchor).extend(0.)
        - (layout.get_size(&tile_size()).as_vec2() * Vec2::new(0.5, -0.5)).extend(0.)
}
