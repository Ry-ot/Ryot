pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Serde(serde_json::Error),
    Lzma(lzma_rs::error::Error),
    Image(image::ImageError),
    SpriteNotFound,
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
