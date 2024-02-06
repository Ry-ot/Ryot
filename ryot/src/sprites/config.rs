use glam::UVec2;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
#[allow(unused)]
pub struct SpriteSheetConfig {
    pub tile_size: UVec2,
    pub sheet_size: UVec2,
    #[serde(default)]
    pub compression_config: Option<CompressionConfig>,
    #[serde(default)]
    pub encoding_config: Option<EncodingConfig>,
}

impl Default for SpriteSheetConfig {
    fn default() -> Self {
        SpriteSheetConfig {
            tile_size: UVec2::new(32, 32),
            sheet_size: UVec2::new(384, 384),
            compression_config: None,
            encoding_config: None,
        }
    }
}

impl SpriteSheetConfig {
    pub fn cip_sheet() -> SpriteSheetConfig {
        SpriteSheetConfig {
            tile_size: UVec2::new(32, 32),
            sheet_size: UVec2::new(384, 384),
            compression_config: Some(CompressionConfig {
                compressed_header_size: 32,
                content_header_size: 122,
            }),
            encoding_config: Some(EncodingConfig::default()),
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct CompressionConfig {
    pub compressed_header_size: usize,
    pub content_header_size: usize,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct EncodingConfig {
    pub vertically_flipped: bool,
    pub reversed_r_b_channels: bool,
}

impl Default for EncodingConfig {
    fn default() -> Self {
        EncodingConfig {
            vertically_flipped: true,
            reversed_r_b_channels: true,
        }
    }
}
