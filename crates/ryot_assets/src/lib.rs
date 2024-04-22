pub mod core;
pub mod sprites;

pub mod prelude {
    pub use crate::{
        core::{
            visual_elements::{
                Animation, Category, EntityType, Flags, FrameGroup, Properties, SpriteInfo,
                VisualElement, VisualElements,
            },
            ContentType,
        },
        sprites::{
            get_decompressed_file_name,
            layout::{SpriteLayout, SpriteLayoutIter},
            sheet::{SpriteSheetData, SpriteSheetDataSet},
            SPRITE_SHEET_FOLDER,
        },
    };
}
