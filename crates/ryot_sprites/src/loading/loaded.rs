use crate::material::SpriteMaterial;
use crate::prelude::*;
use bevy_asset::Handle;
use bevy_ecs::prelude::Resource;
use bevy_render::mesh::Mesh;
use bevy_render::prelude::Image;
use bevy_utils::HashMap;
use derive_more::{Deref, DerefMut};
use ryot_core::content::{ContentId, ContentType};
use ryot_core::prelude::{FrameGroup, SpriteSheet, SpriteSheets};

pub struct LoadedAppearance {
    pub sprites: Vec<LoadedSprite>,
    pub layers: u32,
    pub animation: Option<(AnimationKey, AnimationDescriptor)>,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct LoadedAppearances(pub HashMap<(ContentId, FrameGroup), LoadedAppearance>);

/// A struct that holds the information needed to draw a sprite.
/// It's a wrapper around a sprite sheet and a sprite id, that also holds the
/// handle to the texture atlas.
#[derive(Debug, Clone)]
pub struct LoadedSprite {
    pub sprite_id: u32,
    pub group: ContentType,
    pub sprite_sheet: SpriteSheet,
    pub texture: Handle<Image>,
    pub material: Handle<SpriteMaterial>,
    pub mesh: Handle<Mesh>,
}

impl LoadedSprite {
    pub fn new(
        group: ContentType,
        sprite_id: u32,
        sprite_sheets: &SpriteSheets,
        textures: &HashMap<String, Handle<Image>>,
        material: &Handle<SpriteMaterial>,
        mesh: &Handle<Mesh>,
    ) -> Option<Self> {
        let sprite_sheet = sprite_sheets.get_by_sprite_id(sprite_id)?;
        let texture = textures.get(&sprite_sheet.file)?;
        Some(Self {
            group,
            sprite_id,
            sprite_sheet: sprite_sheet.clone(),
            texture: texture.clone(),
            material: material.clone(),
            mesh: mesh.clone(),
        })
    }

    pub fn get_sprite_index(&self) -> usize {
        self.sprite_sheet
            .get_sprite_index(self.sprite_id)
            .expect("Sprite must exist in sheet")
    }
}
