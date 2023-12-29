/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use std::collections::HashMap;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use image::RgbaImage;
use rayon::prelude::*;
use ryot::cip_content::{ContentType, get_sprite_image_by_id, SpriteSize};

#[derive(Resource, Debug)]
pub struct CipContent {
    pub raw_content: Vec<ContentType>,
    pub sprites: HashMap<u32, (Handle<Image>, SpriteSize)>,
}

impl Default for CipContent {
    fn default() -> Self {
        Self { raw_content: vec![], sprites: HashMap::new() }
    }
}

pub fn load_sprites(
    sprite_ids: &Vec<u32>,
    path: &str,
    mut content: &mut ResMut<CipContent>,
    mut textures: &mut ResMut<Assets<Image>>,
) -> Vec<(Handle<Image>, SpriteSize)> {
    let loaded_sprites: Vec<(u32, Option<(Image, SpriteSize)>)> = sprite_ids.par_iter().map(|sprite_id| {
        match content.sprites.get(sprite_id).cloned() {
            Some(_) => (*sprite_id, None),
            None => (*sprite_id, load_sprite(*sprite_id, path, &content.raw_content)),
        }
    }).collect();

    loaded_sprites.into_iter()
        .filter_map(|(sprite_id, sprite)| {
            if let Some((image, size)) = sprite {
                let handle = textures.add(image);
                content.sprites.insert(sprite_id, (handle, size));
            }

            content.sprites.get(&sprite_id).cloned()
        }).collect()
}

pub fn load_sprite(
    sprite_id: u32,
    path: &str,
    raw_content: &Vec<ContentType>,
) -> Option<(Image, SpriteSize)> {
    if let Some(rgba_image) = get_sprite_image_by_id(raw_content, sprite_id, path) {
        let size = SpriteSize{width: rgba_image.width(), height: rgba_image.height()};
        let image = rgba_image_to_bevy_image(rgba_image);

        return Some((image, size));
    }

    None
}

pub fn rgba_image_to_bevy_image(image: RgbaImage) -> Image {
    Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            },
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[],
        },
        data: image.into_raw(),
        ..Default::default()
    }
}

pub fn build_sprite_bundle(handle: Handle<Image>, pos: Vec2) -> SpriteBundle {
    SpriteBundle {
        texture: handle,
        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
        ..Default::default()
    }
}