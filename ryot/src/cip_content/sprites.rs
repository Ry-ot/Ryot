/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */

use std::path::PathBuf;
use image::{imageops, Rgba, RgbaImage};
use image::error::{LimitError, LimitErrorKind};
use image::imageops::crop;
use log::{debug, warn};
use lzma_rs::lzma_decompress_with_options;
use rayon::prelude::*;
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::cip_content::{ContentType, Result, get_full_file_buffer};

pub const SPRITE_SHEET_SIZE: SpriteSize = SpriteSize{ width: 384, height: 384 };
pub const DEFAULT_SPRITE_SIZE: SpriteSize = SpriteSize{ width: 32, height: 32 };
pub const LZMA_CUSTOM_HEADER_SIZE: usize = 32;
pub const SHEET_CUSTOM_HEADER_SIZE: usize = 122;

#[derive(Debug, Clone)]
pub struct SpriteSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u32)]
pub enum SpriteLayout {
    OneByOne = 0,
    OneByTwo = 1,
    TwoByOne = 2,
    TwoByTwo = 3,
}

impl SpriteLayout {
    pub fn get_width(&self) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::OneByTwo => DEFAULT_SPRITE_SIZE.width,
            SpriteLayout::TwoByOne | SpriteLayout::TwoByTwo => DEFAULT_SPRITE_SIZE.width * 2,
        }
    }

    pub fn get_height(&self) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::TwoByOne => DEFAULT_SPRITE_SIZE.height,
            SpriteLayout::OneByTwo | SpriteLayout::TwoByTwo => DEFAULT_SPRITE_SIZE.height * 2,
        }
    }
}

impl Default for SpriteLayout {
    fn default() -> Self {
        SpriteLayout::OneByOne
    }
}

pub fn load_sprite_sheet_image(path: &str) -> Result<RgbaImage> {
    let input_data = get_full_file_buffer(path)?;

    if is_compressed_file(path) {
        debug!("Decompressing sprite sheet {}", path);
        let decompressed = decompress_lzma_sprite_sheet(input_data)?;
        return create_image_from_data(decompressed, SPRITE_SHEET_SIZE);
    }

    create_image_from_data(input_data, SPRITE_SHEET_SIZE)
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
pub fn create_image_from_data(data: Vec<u8>, size: SpriteSize) -> Result<RgbaImage> {
    let data = data[SHEET_CUSTOM_HEADER_SIZE..].to_vec();

    let mut background_img = RgbaImage::from_raw(size.width, size.height, data)
        .ok_or(image::ImageError::Limits(LimitError::from_kind(LimitErrorKind::DimensionError)))?;

    flip_vertically(&mut background_img);
    reverse_channels(&mut background_img);

    Ok(background_img)
}

/// Decompress and save the plain sprite sheets to the given destination path.
/// This is used to generate a decompressed cache of the sprite sheets, to optimize reading.
/// Trade off here is that we use way more disk space in pro of faster sprite loading.
pub fn decompress_all_sprite_sheets(content: &Vec<ContentType>, path: &str, destination_path: &str) {
    content.par_iter()
        .for_each(|c| match c {
            ContentType::Sprite { file, .. } => {
                let destination_file = &format!("{}/{}", destination_path, get_decompressed_file_name(file));

                if PathBuf::from(destination_file).exists() {
                    return;
                }

                match load_sprite_sheet_image(&format!("{}/{}", path, file)) {
                    Ok(sheet) => sheet.save(destination_file).map_err(|e| {
                        warn!("Failed to save sprite sheet {}: {}", destination_file, e);
                    }).unwrap(),
                    Err(_) => (),
                }
            }
            _ => ()
        });
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

pub fn get_sprite_image_by_id(content: &Vec<ContentType>, id: u32, path: &str) -> Option<RgbaImage> {
    match get_sheet_by_sprite_id(content, id) {
        Some(ContentType::Sprite {
             file,
             layout,
             first_sprite_id,
             ..
         }) => get_sprite_image_from_file(
            format!("{}/{}", path, file),
            layout,
            first_sprite_id,
            id
        ),
        _ => None
    }
}

pub fn get_sprite_image_from_file(file: String, layout: SpriteLayout, first_sprite_id: u32, id: u32) -> Option<RgbaImage> {
    let sprite_offset = id - first_sprite_id;

    let width = layout.get_width();
    let height = layout.get_height();

    let columns = SPRITE_SHEET_SIZE.width / width;

    let row = ((sprite_offset as f32) / (columns as f32)).floor() as u32;
    let column = sprite_offset % columns;

    let decompressed_file_name = get_decompressed_file_name(&file);

    let file_name = if PathBuf::from(&decompressed_file_name).exists() {
        decompressed_file_name
    } else {
        file
    };

    let mut sheet = load_sprite_sheet_image(&file_name).expect("Failed to load sprite sheet");

    Some(crop(&mut sheet, column * width, row * height, width, height).to_image())
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

fn get_decompressed_file_name(file_name: &str) -> String {
    file_name.replace(".lzma", "")
}

fn is_compressed_file(file_name: &str) -> bool {
    file_name.ends_with(".lzma")
}