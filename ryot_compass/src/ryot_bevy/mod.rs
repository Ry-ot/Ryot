use bevy::prelude::*;
use ryot::appearances::{ContentType, SpriteSheet, SpriteSheetSet};
use ryot::*;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::PathBuf;

mod appearances;
pub use appearances::*;

mod async_events;
pub use async_events::*;

mod configs;
pub use configs::*;

pub mod content;

mod palette;
pub use palette::*;
use ryot::tile_grid::TileGrid;

#[derive(Debug, Clone, Event)]
pub struct LoadCommand<T> {
    pub path: String,
    _marker: PhantomData<T>,
}

#[derive(Debug)]
pub struct LoadedSprite {
    pub sprite_id: u32,
    pub sprite_sheet: SpriteSheet,
    pub config: SpriteSheetConfig,
    pub atlas_texture_handle: Handle<TextureAtlas>,
}

#[derive(Resource, Default)]
pub struct TextureAtlasHandlers(pub bevy::utils::HashMap<String, Handle<TextureAtlas>>);

impl LoadedSprite {
    pub fn new(
        sprite_id: u32,
        sprite_sheets: &SpriteSheetSet,
        atlas_handlers: &ResMut<TextureAtlasHandlers>,
    ) -> Option<Self> {
        let sprite_sheet = sprite_sheets.get_by_sprite_id(sprite_id)?;

        Some(Self {
            sprite_id,
            sprite_sheet: sprite_sheet.clone(),
            config: sprite_sheets.sheet_config,
            atlas_texture_handle: atlas_handlers.0.get(&sprite_sheet.file)?.clone(),
        })
    }

    pub fn get_sprite_index(&self) -> usize {
        self.sprite_sheet
            .get_sprite_index(self.sprite_id)
            .expect("Sprite must exist in sheet")
    }
}

pub fn load_sprites(
    sprite_ids: &[u32],
    sprite_sheets: &SpriteSheetSet,
    asset_server: &Res<AssetServer>,
    atlas_handlers: &mut ResMut<TextureAtlasHandlers>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Vec<LoadedSprite> {
    let unsaved_sprites: Vec<(&SpriteSheet, TextureAtlas)> = sprite_ids
        .iter()
        .filter_map(|sprite_id| {
            let sprite_sheet = sprite_sheets.get_by_sprite_id(*sprite_id)?;

            Some((
                sprite_sheet,
                build_texture_atlas_from_sheet(*sprite_id, sprite_sheets, asset_server).unwrap(),
            ))
        })
        .collect();

    unsaved_sprites.iter().for_each(|(sprite_sheet, atlas)| {
        build_atlas_handler(
            sprite_sheet.file.clone(),
            atlas.clone(),
            atlas_handlers,
            texture_atlases,
        );
    });

    sprite_ids
        .iter()
        .filter_map(|sprite_id| LoadedSprite::new(*sprite_id, sprite_sheets, atlas_handlers))
        .collect::<Vec<_>>()
}

pub fn get_sprite_by_id(
    content: &[ContentType],
    sprite_id: u32,
    atlas_handlers: &ResMut<TextureAtlasHandlers>,
) -> Option<LoadedSprite> {
    let sprite_sheets = SpriteSheetSet::from_content(content, &SpriteSheetConfig::cip_sheet());

    LoadedSprite::new(sprite_id, &sprite_sheets, atlas_handlers)
}

#[derive(Debug, Clone, Event)]
pub struct AtlasesWereLoaded {
    pub atlases: HashMap<String, TextureAtlas>,
}

pub fn build_atlas(
    mut reader: EventReader<AtlasesWereLoaded>,
    mut atlas_handlers: ResMut<TextureAtlasHandlers>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for AtlasesWereLoaded { atlases } in reader.read() {
        for (file, texture_atlas) in atlases {
            if !atlas_handlers.0.contains_key(file) {
                let atlas_handle = texture_atlases.add(texture_atlas.clone());
                atlas_handlers.0.insert(file.clone(), atlas_handle);
            }
        }
    }
}

pub fn build_atlas_handler(
    file: String,
    texture_atlas: TextureAtlas,
    atlas_handlers: &mut ResMut<TextureAtlasHandlers>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    if !atlas_handlers.0.contains_key(&file) {
        let atlas_handle = texture_atlases.add(texture_atlas);
        atlas_handlers.0.insert(file.clone(), atlas_handle);
    }

    atlas_handlers
        .0
        .get(&file)
        .expect(
            "Failed to get atlas handler, this should never happen, please report this as a bug",
        )
        .clone()
}

pub fn build_texture_atlas_from_sheet(
    sprite_id: u32,
    sprite_sheets: &SpriteSheetSet,
    asset_server: &Res<AssetServer>,
) -> Result<TextureAtlas, std::io::Error> {
    let sprite_sheet = sprite_sheets.get_by_sprite_id(sprite_id).expect(
        "Sprite must exist in sheet, this should never happen, please report this as a bug",
    );

    let image_handle: Handle<Image> = asset_server.load(
        PathBuf::from(SPRITE_SHEET_FOLDER).join(get_decompressed_file_name(&sprite_sheet.file)),
    );

    Ok(TextureAtlas::from_grid(
        image_handle,
        sprite_sheet
            .get_tile_size(&sprite_sheets.sheet_config)
            .as_vec2(),
        sprite_sheet.get_columns_count(&sprite_sheets.sheet_config),
        sprite_sheet.get_rows_count(&sprite_sheets.sheet_config),
        None,
        None,
    ))
}

pub fn draw_sprite(pos: Vec3, sprite: &LoadedSprite, commands: &mut Commands) {
    let Some(tile_pos) = TileGrid::default().get_display_position_from_tile_pos_vec3(pos) else {
        return;
    };

    commands.spawn(build_sprite_bundle(
        sprite.atlas_texture_handle.clone(),
        tile_pos,
        sprite.get_sprite_index(),
    ));
}

pub fn build_sprite_bundle(
    handle: Handle<TextureAtlas>,
    translation: Vec3,
    index: usize,
) -> SpriteSheetBundle {
    SpriteSheetBundle {
        transform: Transform {
            translation,
            ..default()
        },
        sprite: TextureAtlasSprite::new(index),
        texture_atlas: handle,
        ..default()
    }
}
