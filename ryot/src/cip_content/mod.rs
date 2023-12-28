include!(concat!(env!("OUT_DIR"), "/appearances.rs"));

mod sprites;
pub use sprites::*;

use std::io::Read;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Serde(serde_json::Error),
    Lzma(lzma_rs::error::Error),
    Image(image::ImageError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<lzma_rs::error::Error> for Error {
    fn from(e: lzma_rs::error::Error) -> Self {
        Error::Lzma(e)
    }
}

impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Self {
        Error::Image(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ContentType {
    #[serde(rename = "appearances")]
    Appearances {
        file: String,
        version: u32,
    },
    #[serde(rename = "staticdata")]
    StaticData {
        file: String,
    },
    #[serde(rename = "staticmapdata")]
    StaticMapData {
        file: String,
    },
    #[serde(rename = "map")]
    Map {
        file: String,
    },
    #[serde(rename = "sprite")]
    Sprite {
        file: String,
        #[serde(rename = "spritetype")]
        layout: SpriteLayout,
        #[serde(rename = "firstspriteid")]
        first_sprite_id: u32,
        #[serde(rename = "lastspriteid")]
        last_sprite_id: u32,
        area: u32,
    },
}

pub fn load_content(path: &str) -> Result<Vec<ContentType>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let content: Vec<ContentType> = serde_json::from_reader(reader)?;

    Ok(content)
}

pub fn get_full_file_buffer(path: &str) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}