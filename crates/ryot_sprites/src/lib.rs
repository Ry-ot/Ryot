#[cfg(feature = "bevy")]
pub mod atlas;
#[cfg(feature = "bevy")]
pub mod material;
#[cfg(feature = "bevy")]
pub mod meshes;
#[cfg(feature = "bevy")]
pub mod plugins;

pub static SPRITE_SHEET_FOLDER: &str = "sprite-sheets";

pub fn get_decompressed_file_name(file_name: &str) -> String {
    file_name.replace(".bmp.lzma", ".png")
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::SystemSet))]
pub enum SpriteSystems {
    Load,
    Initialize,
    Update,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::SystemSet))]
pub enum AnimationSystems {
    Initialize,
    Update,
}

pub mod prelude {
    pub use crate::{get_decompressed_file_name, SPRITE_SHEET_FOLDER};

    #[cfg(feature = "bevy")]
    pub use crate::{
        atlas::{AtlasLayoutsAsset, TextureAtlasLayouts},
        material::SpriteMaterial,
        meshes::{RectMeshes, SpriteMeshes},
        plugins::RyotSpritePlugin,
        AnimationSystems, SpriteSystems,
    };
}

#[cfg(test)]
mod tests;
