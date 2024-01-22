use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not serialize/deserialize file: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Could not decompress file: {0}")]
    Lzma(#[from] lzma_rs::error::Error),
    #[error("Invalid image: {0}")]
    Image(#[from] image::ImageError),
    #[error("Could not find sprite.")]
    SpriteNotFound,
}
