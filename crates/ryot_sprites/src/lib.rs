use bevy_ecs::prelude::SystemSet;

pub mod animations;
pub mod material;
pub mod meshes;
pub mod params;

// TODO: make this better module, break it down
pub mod sprite_animations;

// TODO: make this better module, break it down
pub mod sprites;

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
        get_decompressed_file_name,
        material::{embed_sprite_assets, initialize_sprite_material, SpriteMaterial},
        meshes::{RectMeshes, SpriteMeshes},
        params::{SpriteOutline, SpriteParams},
        sprite_animations::*,
        sprites::*,
        SpriteSystems, SPRITE_SHEET_FOLDER,
    };

    #[cfg(feature = "debug")]
    pub use crate::sprites::debug_sprites;
}
