#[cfg(feature = "bevy")]
pub mod atlas;
pub mod definitions;
pub mod layout;
#[cfg(feature = "bevy")]
pub mod meshes;
pub mod sheet;

pub static SPRITE_SHEET_FOLDER: &str = "sprite-sheets";

pub fn get_decompressed_file_name(file_name: &str) -> String {
    file_name.replace(".bmp.lzma", ".png")
}

pub mod prelude {
    pub use crate::{
        definitions::{Animation, SpriteInfo},
        get_decompressed_file_name,
        layout::{SpriteLayout, SpriteLayoutIter},
        sheet::definitions::{SpriteSheetData, SpriteSheetDataSet},
        SPRITE_SHEET_FOLDER,
    };

    #[cfg(feature = "bevy")]
    pub use crate::{
        atlas::{AtlasLayoutsAsset, TextureAtlasLayouts},
        meshes::{RectMeshes, SpriteMeshes},
    };
}

#[cfg(test)]
mod tests;
