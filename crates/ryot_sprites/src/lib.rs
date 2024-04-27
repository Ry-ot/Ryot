//! `ryot_sprites`
//!
//! Focuses on sprite management, dealing with animations and visual representations of
//! game objects, supporting rich graphical content within games.
use bevy_ecs::prelude::SystemSet;

pub mod animation;
pub mod loading;
pub mod material;
pub mod sheets;
pub mod update;

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
        animation::{
            descriptor::{AnimationDescriptor, AnimationSprite},
            key::{AnimationKey, SpriteAnimationExt},
            state::{AnimationStartPhase, AnimationState},
            systems::{
                initialize_animation_sprite_system, tick_animation_system, AnimationDuration,
                AnimationSystems, SynchronizedAnimationTimers,
            },
            toggle::{toggle_sprite_animation, SpriteAnimationEnabled},
        },
        get_decompressed_file_name,
        loading::{
            loaded::{LoadedAppearance, LoadedAppearances, LoadedSprite},
            systems::{
                load_from_entities_system, load_sprite_system, process_load_events_system,
                store_loaded_appearances_system,
            },
            LoadAppearanceEvent,
        },
        material::{
            embed_sprite_assets, initialize_sprite_material,
            meshes::{RectMeshes, SpriteMeshes},
            params::{SpriteOutline, SpriteParams},
            SpriteMaterial,
        },
        sheets::SpriteSheets,
        update::update_sprite_system,
        SpriteSystems, SPRITE_SHEET_FOLDER,
    };

    #[cfg(feature = "debug")]
    pub use crate::loading::debug::debug_sprites;
}

#[cfg(test)]
mod tests;
