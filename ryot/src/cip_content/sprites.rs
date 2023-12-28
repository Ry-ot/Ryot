/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */

use image::{imageops, Rgba, RgbaImage};
use image::codecs::png::FilterType::Paeth;
use image::error::{LimitError, LimitErrorKind};
use lzma_rs::lzma_decompress_with_options;
use rayon::prelude::*;
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::cip_content::{ContentType, Result, get_full_file_buffer};

pub const SPRITE_SHEET_SIZE: u32 = 384;
pub const LZMA_CUSTOM_HEADER_SIZE: usize = 32;
pub const SHEET_CUSTOM_HEADER_SIZE: usize = 122;

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u32)]
pub enum SpriteLayout {
    OneByOne = 0,
    OneByTwo = 1,
    TwoByOne = 2,
    TwoByTwo = 3,
}

pub fn load_sprite_sheet_image(path: &str) -> Result<RgbaImage> {
    let input_data = get_full_file_buffer(path)?;
    let decompressed = decompress_lzma_sprite_sheet(input_data)?;
    create_image_from_data(decompressed, SPRITE_SHEET_SIZE, SPRITE_SHEET_SIZE)
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
pub fn decompress_lzma_sprite_sheet(buffer: Vec<u8>) -> Result<Vec<u8>> {
    let mut decompressed = Vec::new();
    lzma_decompress_with_options(&mut &buffer[LZMA_CUSTOM_HEADER_SIZE..], &mut decompressed, &lzma_rs::decompress::Options{
        unpacked_size: lzma_rs::decompress::UnpackedSize::ReadHeaderButUseProvided(None),
        ..Default::default()
    })?;

    Ok(decompressed)
}

/// Creates a sprite sheet image from a given CIP decompressed bmp and saves it to the given output path.
/// The sprite sheet is expected to have 384x384 pixels and 4 channels (RGBA).
/// The data is expected to be in the BGRA format.
/// The data is expected to be flipped vertically.
/// The data is expected to have a 122 byte header that we need to skip
pub fn create_image_from_data(data: Vec<u8>, width: u32, height: u32) -> Result<RgbaImage> {
    let data = data[SHEET_CUSTOM_HEADER_SIZE..].to_vec();

    let mut background_img = RgbaImage::from_raw(width, height, data)
        .ok_or(image::ImageError::Limits(LimitError::from_kind(LimitErrorKind::DimensionError)))?;

    flip_vertically(&mut background_img);
    reverse_channels(&mut background_img);

    Ok(background_img)
}

/// Loads, decompresses, filters and transforms all sprite sheets from a given content.
/// The result is a vector of RgbaImages, where each image is a 384x384 sprite sheet.
pub fn get_all_sprite_sheets(content: &Vec<ContentType>, path: &str) -> Vec<RgbaImage> {
    content.par_iter()
        .filter_map(|c| match c {
            ContentType::Sprite { file, layout: sprite_type, first_sprite_id, last_sprite_id, area } => {
                Some(load_sprite_sheet_image(&format!("{}{}", path, file)))
            }
            _ => None
        })
        .filter_map(Result::ok)
        .collect()
}

pub fn get_sheet_by_sprite_id(content: &Vec<ContentType>, id: u32) -> Option<ContentType> {
    content.iter()
        .find(|content| {
            match content {
                ContentType::Sprite { first_sprite_id, last_sprite_id, .. } => id >= *first_sprite_id && id <= *last_sprite_id,
                _ => false
            }
        }).cloned()
}

pub fn load_sprite_sheet_for_content(file: &str, path: &str) -> Result<RgbaImage> {
    Ok(load_sprite_sheet_image(&format!("{}{}", path, file))?)
}

fn reverse_channels(img: &mut RgbaImage) {
    for pixel in img.pixels_mut() {
        let Rgba([r, g, b, a]) = *pixel;
        *pixel = Rgba([b, g, r, a]); // Swap Red and Blue channels
    }
}

fn flip_vertically(img: &mut RgbaImage) {
    *img = imageops::flip_vertical(img);
}
