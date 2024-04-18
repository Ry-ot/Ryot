pub mod sprites;

pub static SPRITE_SHEET_FOLDER: &str = "sprite-sheets";

pub fn get_decompressed_file_name(file_name: &str) -> String {
    file_name.replace(".bmp.lzma", ".png")
}

pub mod prelude {
    pub use crate::sprites::layout::{SpriteLayout, SpriteLayoutIter};
    pub use crate::{get_decompressed_file_name, SPRITE_SHEET_FOLDER};
}
