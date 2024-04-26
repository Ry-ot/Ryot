use crate::material::SpriteMaterial;
use bevy_ecs::component::Component;
use bevy_render::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct SpriteOutline {
    color: Color,
    thickness: f32,
}

#[derive(Debug, Clone, PartialEq, Default, Component)]
pub struct SpriteParams {
    pub alpha: Option<f32>,
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

    pub fn with_alpha(self, alpha: f32) -> Self {
        Self {
            alpha: Some(alpha),
            ..self
        }
    }

    pub fn has_any(&self) -> bool {
        self.outline.is_some() || self.tint.is_some() || self.alpha.is_some()
    }

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
