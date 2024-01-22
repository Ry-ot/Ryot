use glam::UVec2;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
#[allow(unused)]
pub struct SpriteSheetConfig {
    pub tile_size: UVec2,
    pub sheet_size: UVec2,
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
        tile_size: UVec2::new(32, 32),
        sheet_size: UVec2::new(384, 384),
        compression_config: Some(CompressionConfig {
            compressed_header_size: 32,
            content_header_size: 122,
        }),
    }
}
