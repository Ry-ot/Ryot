use config::Config;
use serde::Deserialize;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, result};

#[cfg(feature = "compression")]
mod compression;
#[cfg(feature = "compression")]
pub use compression::{compress, decompress, Compression, Zstd};

#[cfg(feature = "lmdb")]
pub mod lmdb;

pub mod appearances;

mod sprites;
pub use sprites::*;

pub static CONTENT_CONFIG_PATH: &str = "config/Content.toml";
pub static SPRITE_SHEET_FOLDER: &str = "sprite-sheets";

#[derive(Debug, Clone, Deserialize)]
pub struct ContentConfigs {
    pub directories: DirectoryConfigs,
    pub sprite_sheet: SpriteSheetConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct DirectoryConfigs {
    pub source_path: PathBuf,
    #[serde(default = "assets_root_path")]
    pub destination_path: PathBuf,
}

pub fn assets_root_path() -> PathBuf {
    PathBuf::from("assets")
}

pub fn read_content_configs(config_path: &str) -> ContentConfigs {
    let settings = Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()
        .expect("Failed to build config")
        .try_deserialize::<ContentConfigs>()
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

pub fn is_path_within_root(
    destination_path: &Path,
    root_path: &Path,
) -> result::Result<bool, std::io::Error> {
    Ok(fs::canonicalize(destination_path)?.starts_with(fs::canonicalize(root_path)?))
}

pub fn get_full_file_buffer(path: &PathBuf) -> error::Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

type Result<T> = result::Result<T, config::ConfigError>;

pub fn config_from<'de, T: Deserialize<'de>>(config_path: &str) -> Result<T> {
    Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()?
        .try_deserialize::<T>()
}
