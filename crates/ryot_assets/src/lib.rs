#![feature(trait_alias)]

pub mod core;
pub mod sprites;

pub mod prelude {
    pub use crate::{
        core::{
            content::ContentType,
            visual_elements::definitions::{
                Animation, Category, EntityType, Flags, FrameGroup, Properties, SpriteInfo,
                VisualElement, VisualElements,
            },
        },
        sprites::{
            get_decompressed_file_name,
            layout::{SpriteLayout, SpriteLayoutIter},
            sheet::definitions::{SpriteSheetData, SpriteSheetDataSet},
            SPRITE_SHEET_FOLDER,
        },
    };

    #[cfg(feature = "bevy")]
    pub use crate::{
        core::{
            content::load::{transition_to_ready, Catalog, CatalogAsset, InternalContentState},
            visual_elements::load::{prepare_visual_elements, VisualElementsAsset},
        },
        sprites::{
            atlas::{AtlasLayoutsAsset, TextureAtlasLayouts},
            meshes::{RectMeshes, SpriteMeshes},
            sheet::load::prepare_sprite_sheets,
        },
    };
}
