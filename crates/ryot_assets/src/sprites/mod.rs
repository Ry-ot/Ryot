#[cfg(feature = "bevy")]
pub mod atlas;
pub mod layout;
#[cfg(feature = "bevy")]
pub mod meshes;
pub mod sheet;

pub static SPRITE_SHEET_FOLDER: &str = "sprite-sheets";

pub fn get_decompressed_file_name(file_name: &str) -> String {
    file_name.replace(".bmp.lzma", ".png")
}

#[cfg(test)]
mod tests;
