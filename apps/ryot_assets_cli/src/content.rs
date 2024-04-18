use crate::SpriteSheetConfig;
use serde::Deserialize;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

pub static DYNAMIC_ASSETS_PATH: &str = "dyanmic.assets.ron";

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ContentConfigs {
    pub directories: DirectoryConfigs,
    pub sprite_sheet: SpriteSheetConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct DirectoryConfigs {
    #[serde(default = "default_source_path")]
    pub source_path: PathBuf,
    #[serde(default = "assets_root_path")]
    pub destination_path: PathBuf,
}

impl Default for DirectoryConfigs {
    fn default() -> Self {
        Self {
            source_path: assets_root_path(),
            destination_path: assets_root_path(),
        }
    }
}

pub fn default_source_path() -> PathBuf {
    assets_root_path().join("content")
}

pub fn assets_root_path() -> PathBuf {
    PathBuf::from("assets")
}

pub fn get_full_file_buffer(path: &PathBuf) -> crate::Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}
