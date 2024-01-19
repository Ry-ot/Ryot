use crate::appearances::is_path_within_root;
use config::Config;
use serde::Deserialize;
use std::path::{Path, PathBuf};

mod compression;
pub use compression::{compress, decompress, Compression, Zstd};

#[cfg(feature = "std")]
pub mod lmdb;

pub mod appearances;

mod sprites;
pub use sprites::*;

pub static CONFIG_PATH: &str = "config/Assets.toml";
pub static SPRITE_SHEET_FOLDER: &str = "sprite-sheets";

#[derive(Debug, Deserialize)]
pub struct AssetsConfig {
    pub directories: DirectoryConfigs,
    pub sprite_sheet: SpriteSheetConfig,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct DirectoryConfigs {
    pub source_path: PathBuf,
    #[serde(default = "default_destination_path")]
    pub destination_path: PathBuf,
}

pub fn default_destination_path() -> PathBuf {
    PathBuf::from("assets")
}

pub fn read_assets_configs(config_path: &str) -> AssetsConfig {
    let settings = Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()
        .expect("Failed to build config")
        .try_deserialize::<AssetsConfig>()
        .expect("Failed to deserialize config");

    let dir_settings = &settings.directories;

    match is_path_within_root(&dir_settings.destination_path, Path::new("assets")) {
        Ok(true) => settings,
        Ok(false) | Err(_) => panic!(
            "Target path {} is not within assets folder",
            dir_settings
                .destination_path
                .to_str()
                .expect("Failed to convert target path to str")
        ),
    }
}
