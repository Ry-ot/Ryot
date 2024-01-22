use crate::Rect;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
#[allow(unused)]
pub struct SpriteSheetConfig {
    pub tile_size: Rect,
    pub sheet_size: Rect,
    #[serde(default)]
    pub compression_config: Option<CompressionConfig>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct CompressionConfig {
    pub compressed_header_size: usize,
    pub content_header_size: usize,
}

pub fn cip_sheet() -> SpriteSheetConfig {
    SpriteSheetConfig {
        tile_size: Rect::new(32, 32),
        sheet_size: Rect::new(384, 384),
        compression_config: Some(CompressionConfig {
            compressed_header_size: 32,
            content_header_size: 122,
        }),
    }
}
