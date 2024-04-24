use bevy_ecs::prelude::SystemSet;

pub mod animations;
pub mod atlas;
pub mod material;
pub mod meshes;
pub mod params;
pub mod plugins;

pub static SPRITE_SHEET_FOLDER: &str = "sprite-sheets";

pub fn get_decompressed_file_name(file_name: &str) -> String {
    file_name.replace(".bmp.lzma", ".png")
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SpriteSystems {
    Load,
    Initialize,
    Update,
}

pub mod prelude {
    pub use crate::{
        animations::AnimationSystems,
        atlas::TextureAtlasLayouts,
        get_decompressed_file_name,
        material::SpriteMaterial,
        meshes::{RectMeshes, SpriteMeshes},
        params::{SpriteOutline, SpriteParams},
        plugins::RyotSpritePlugin,
        SpriteSystems, SPRITE_SHEET_FOLDER,
    };
}
