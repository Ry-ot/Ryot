use crate::{error::*, get_full_file_buffer, CompressionConfig, ContentConfigs, SpriteSheetConfig};
use glam::UVec2;
use image::error::{LimitError, LimitErrorKind};
use image::{imageops, ImageFormat, Rgba, RgbaImage};
use log::{debug, warn};
use lzma_rs::lzma_decompress_with_options;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_sprite_sheet_image(
    path: &PathBuf,
    sheet_config: SpriteSheetConfig,
    sheet_size: &UVec2,
) -> Result<RgbaImage> {
    let input_data = get_full_file_buffer(path)?;

    let Some(compression_headers) = &sheet_config.compression_config else {
        return create_image_from_data(input_data, &sheet_config, sheet_size);
    };

    let decompressed = decompress_lzma_sprite_sheet(input_data, compression_headers)?;
    create_image_from_data(decompressed, &sheet_config, sheet_size)
}

/// Creates a sprite sheet image from a given CIP decompressed bmp and saves it to the given output path.
/// The sprite sheet is expected to have 384x384 pixels and 4 channels (RGBA).
/// The data is expected to be in the BGRA format.
/// The data is expected to be flipped vertically.
/// The data is expected to have a 122 byte header that we need to skip
pub fn create_image_from_data(
    data: Vec<u8>,
    sheet_config: &SpriteSheetConfig,
    sheet_size: &UVec2,
) -> Result<RgbaImage> {
    let mut background_img = match &sheet_config.compression_config {
        None => image::load_from_memory_with_format(&data, ImageFormat::Png)?.to_rgba8(),
        Some(compression_config) => RgbaImage::from_raw(
            sheet_size.x,
            sheet_size.y,
            data[compression_config.content_header_size..].to_vec(),
        )
        .ok_or(image::ImageError::Limits(LimitError::from_kind(
            LimitErrorKind::DimensionError,
        )))?,
    };

    if let Some(encoding_config) = &sheet_config.encoding_config {
        if encoding_config.vertically_flipped {
            flip_vertically(&mut background_img);
        }

        if encoding_config.reversed_r_b_channels {
            reverse_channels(&mut background_img);
        }
    }

    Ok(background_img)
}

/// CIP's sprite sheets have a 32 byte header that contains the following information:
///  [0..X]    - Variable number of place holder NULL (0x00) bytes. X varies according to the sheet decompressed size.
///  [X..X+5]  - 5 bytes with the constant byte sequence [0x70 0x0A 0xFA 0x80 0x24].
///  [X+6..32] - LZMA file size encoded as a 7-bit integer, can occupy 2 or 3 bytes, depending on the number represented.
///
/// Ideally the header can be used to validate the sprite sheet content. However, here we will assume that the content
/// is always valid. Thus we don't care about this header at all, so we just skip the first 32 bytes and
/// start decompressing from the 33rd byte.
///
/// Also, CIP builds the wrong LZMA header, placing the compressed size instead of the decompressed size.
/// Because of that, we have 2 options here:
///  1. Decompress always expecting the sheet size (384 * 384 * 4 + 122 = 591130 bytes)
///  2. Decompress without size defined and wait for the end-of-stream marker
/// Option 2 works well for the sprite sheets, so we went with it. The lzma library already parses the header
/// getting the lclppb, lp and pb values and ignoring the size, so we don't need to do anything special here.
pub fn decompress_lzma_sprite_sheet(
    buffer: Vec<u8>,
    compression_config: &CompressionConfig,
) -> Result<Vec<u8>> {
    let mut decompressed = Vec::new();
    lzma_decompress_with_options(
        &mut &buffer[compression_config.compressed_header_size..],
        &mut decompressed,
        &lzma_rs::decompress::Options {
            unpacked_size: lzma_rs::decompress::UnpackedSize::ReadHeaderButUseProvided(None),
            ..Default::default()
        },
    )?;

    Ok(decompressed)
}

/// Decompress and save the plain sprite sheets to the given destination path.
/// This is used to generate a decompressed cache of the sprite sheets, to optimize reading.
/// Trade off here is that we use way more disk space in pro of faster sprite loading.
pub fn decompress_sprite_sheets(
    content_configs: ContentConfigs,
    sheet_size: &UVec2,
    files: &Vec<String>,
) {
    let sprite_sheet_path = content_configs
        .directories
        .destination_path
        .join(crate::SPRITE_SHEET_FOLDER);

    fs::create_dir_all(sprite_sheet_path.clone()).expect("Failed to create sprite sheets folder");

    files.par_iter().for_each(|file| {
        decompress_sprite_sheet(
            file,
            &content_configs.directories.source_path,
            &sprite_sheet_path,
            content_configs.sprite_sheet,
            sheet_size,
        );
    });
}

pub fn decompress_sprite_sheet(
    file: &str,
    path: &Path,
    destination_path: &Path,
    sheet_config: SpriteSheetConfig,
    sheet_size: &UVec2,
) {
    let destination_file = &format!(
        "{}/{}",
        destination_path.display(),
        get_decompressed_file_name(file)
    );

    if PathBuf::from(destination_file).exists() {
        return;
    }

    debug!("Decompressing sprite sheet {} {}", path.display(), file);
    match load_sprite_sheet_image(
        &format!("{}/{}", path.display(), file).into(),
        sheet_config,
        sheet_size,
    ) {
        Ok(sheet) => sheet
            .save_with_format(destination_file, ImageFormat::Png)
            .map_err(|e| {
                warn!("Failed to save sprite sheet {}: {}", destination_file, e);
            })
            .expect("Failed to save sprite sheet"),
        Err(e) => panic!("{:?}", e),
    }
}

pub fn get_decompressed_file_name(file_name: &str) -> String {
    file_name.replace(".bmp.lzma", ".png")
}

fn flip_vertically(img: &mut RgbaImage) {
    *img = imageops::flip_vertical(img);
}

fn reverse_channels(img: &mut RgbaImage) {
    for pixel in img.pixels_mut() {
        let Rgba([r, g, b, a]) = *pixel;
        *pixel = Rgba([b, g, r, a]); // Swap Red and Blue channels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_decompressed_file_name() {
        assert_eq!(
            get_decompressed_file_name("1.bmp.lzma"),
            "1.png".to_string()
        );
    }

    #[test]
    fn test_flip_vertically() {
        let mut img = RgbaImage::from_raw(1, 2, vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
        flip_vertically(&mut img);
        assert_eq!(
            img,
            RgbaImage::from_raw(1, 2, vec![5, 6, 7, 8, 1, 2, 3, 4]).unwrap()
        );
    }

    #[test]
    fn test_reverse_channels() {
        let mut img = RgbaImage::from_raw(1, 1, vec![1, 2, 3, 4]).unwrap();
        reverse_channels(&mut img);
        assert_eq!(img, RgbaImage::from_raw(1, 1, vec![3, 2, 1, 4]).unwrap());
    }
}
