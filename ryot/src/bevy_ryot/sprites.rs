use crate::appearances::{SpriteSheet, SpriteSheetSet};
use crate::prelude::tile_grid::TileGrid;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::asset_collection::AssetCollection;
use std::path::PathBuf;

use crate::bevy_ryot::{RyotSetupStates, Sprites};
use crate::{get_decompressed_file_name, SpriteSheetConfig, SPRITE_SHEET_FOLDER};
use bevy_asset_loader::prelude::*;

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextureAtlasHandlers>();

        app.add_loading_state(
            LoadingState::new(RyotSetupStates::LoadingSprites)
                .continue_to_state(RyotSetupStates::PreparingSprites)
                .load_collection::<SpriteAssets>(),
        )
        .add_systems(OnEnter(RyotSetupStates::PreparingSprites), sprites_preparer);

        app.add_event::<LoadSpriteSheetTextureCommand>()
            .add_event::<SpriteSheetTextureWasLoaded>()
            .init_resource::<TextureAtlasHandlers>()
            .add_systems(Update, sprite_sheet_loader_system)
            .add_systems(Update, atlas_handler_system);
    }
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[cfg(feature = "pre_loaded_sprites")]
    #[asset(path = "sprite-sheets", collection(typed, mapped))]
    pub sprite_sheets: HashMap<String, Handle<Image>>,
    #[asset(path = "ryot_mascot.png")]
    pub mascot: Handle<Image>,
}

#[derive(Debug, Clone, Event)]
pub struct LoadSpriteSheetTextureCommand {
    pub sprite_ids: Vec<u32>,
}

#[derive(Debug, Clone, Event)]
struct SpriteSheetTextureWasLoaded {
    pub sprite_id: u32,
    pub atlas: TextureAtlas,
}

#[derive(Resource, Default)]
pub struct TextureAtlasHandlers(HashMap<String, Handle<TextureAtlas>>);

impl TextureAtlasHandlers {
    pub fn get(&self, file: &str) -> Option<&Handle<TextureAtlas>> {
        self.0.get(&get_decompressed_file_name(file))
    }

    pub fn insert(
        &mut self,
        file: &str,
        handle: Handle<TextureAtlas>,
    ) -> Option<Handle<TextureAtlas>> {
        self.0.insert(get_decompressed_file_name(file), handle)
    }
}

#[derive(Debug)]
pub struct LoadedSprite {
    pub sprite_id: u32,
    pub sprite_sheet: SpriteSheet,
    pub config: SpriteSheetConfig,
    pub atlas_texture_handle: Handle<TextureAtlas>,
}

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
    sprites: Res<Sprites>,
    atlas_handlers: ResMut<TextureAtlasHandlers>,
    mut build_spr_sheet_texture_cmd: EventWriter<LoadSpriteSheetTextureCommand>,
) -> Vec<LoadedSprite> {
    let Some(sprite_sheets) = &sprites.sheets else {
        warn!("No sprite sheets loaded");
        return vec![];
    };

    let mut to_be_loaded: Vec<u32> = vec![];
    let mut loaded: Vec<LoadedSprite> = vec![];

    for sprite_id in sprite_ids {
        let Some(sprite_sheet) = sprite_sheets.get_by_sprite_id(*sprite_id) else {
            warn!("Sprite {} not found in sprite sheets", sprite_id);
            continue;
        };

        let Some(handle) = atlas_handlers.get(sprite_sheet.file.as_str()) else {
            to_be_loaded.push(*sprite_id);
            continue;
        };

        loaded.push(LoadedSprite {
            sprite_id: *sprite_id,
            sprite_sheet: sprite_sheet.clone(),
            config: sprite_sheets.sheet_config,
            atlas_texture_handle: handle.clone(),
        });
    }

    build_spr_sheet_texture_cmd.send(LoadSpriteSheetTextureCommand {
        sprite_ids: to_be_loaded,
    });

    loaded
}

fn sprite_sheet_loader_system(
    sprites: Res<Sprites>,
    asset_server: Res<AssetServer>,
    atlas_handlers: ResMut<TextureAtlasHandlers>,
    mut sprite_sheet_texture_was_loaded: EventWriter<SpriteSheetTextureWasLoaded>,
    mut build_spr_sheet_texture_cmd: EventReader<LoadSpriteSheetTextureCommand>,
) {
    let Some(sprite_sheets) = &sprites.sheets else {
        return;
    };

    for LoadSpriteSheetTextureCommand { sprite_ids } in build_spr_sheet_texture_cmd.read() {
        for sprite_id in sprite_ids {
            let Some(sprite_sheet) = sprite_sheets.get_by_sprite_id(*sprite_id) else {
                warn!("Sprite {} not found in sprite sheets", sprite_id);
                continue;
            };

            if atlas_handlers.get(sprite_sheet.file.as_str()).is_some() {
                continue;
            }

            let image_handle: Handle<Image> = asset_server.load(
                PathBuf::from(SPRITE_SHEET_FOLDER)
                    .join(get_decompressed_file_name(&sprite_sheet.file)),
            );

            let config = &sprite_sheets.sheet_config;

            sprite_sheet_texture_was_loaded.send(SpriteSheetTextureWasLoaded {
                sprite_id: *sprite_id,
                atlas: TextureAtlas::from_grid(
                    image_handle,
                    sprite_sheet.get_tile_size(config).as_vec2(),
                    sprite_sheet.get_columns_count(config),
                    sprite_sheet.get_rows_count(config),
                    None,
                    None,
                ),
            });
        }
    }
}

fn atlas_handler_system(
    sprites: Res<crate::bevy_ryot::Sprites>,
    mut atlas_handlers: ResMut<TextureAtlasHandlers>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut sprite_sheet_texture_was_loaded: EventReader<SpriteSheetTextureWasLoaded>,
) {
    for SpriteSheetTextureWasLoaded { sprite_id, atlas } in sprite_sheet_texture_was_loaded.read() {
        let sprite_sheet = sprites
            .sheets
            .as_ref()
            .expect("Sprite sheets must be loaded")
            .get_by_sprite_id(*sprite_id)
            .expect("Sprite must exist in sheet");

        if atlas_handlers.get(sprite_sheet.file.as_str()).is_some() {
            continue;
        }

        let atlas_handle = texture_atlases.add(atlas.clone());
        atlas_handlers.insert(&sprite_sheet.file, atlas_handle);
    }
}

pub fn draw_sprite(pos: Vec3, sprite: &LoadedSprite, commands: &mut Commands, tile_grid: TileGrid) {
    let Some(tile_pos) = tile_grid.get_display_position_from_tile_pos_vec3(pos) else {
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

#[allow(dead_code)]
fn sprites_preparer(
    _sprites: Res<Sprites>,
    _sprite_assets: Res<SpriteAssets>,
    mut state: ResMut<NextState<RyotSetupStates>>,
    _atlas_handlers: ResMut<TextureAtlasHandlers>,
    _texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Preparing sprites");

    #[cfg(feature = "pre_loaded_sprites")]
    {
        let Some(sheets) = &sprites.sheets else {
            panic!("Sprite sheets configs were not setup.");
        };

        for (file, handle) in &sprite_assets.sprite_sheets {
            let file = match file.strip_prefix("sprite-sheets/") {
                Some(file) => file,
                None => file,
            };

            if atlas_handlers.get(file).is_some() {
                warn!("Skipping file {}: it's already loaded", file);
                continue;
            }

            let Some(sprite_sheet) = &sheets.get_for_file(file) else {
                warn!("Skipping file {}: it's not in sprite sheets", file);
                continue;
            };

            let atlas = TextureAtlas::from_grid(
                handle.clone(),
                sprite_sheet.get_tile_size(&sheets.sheet_config).as_vec2(),
                sprite_sheet.get_columns_count(&sheets.sheet_config),
                sprite_sheet.get_rows_count(&sheets.sheet_config),
                None,
                None,
            );

            let atlas_handle = texture_atlases.add(atlas.clone());
            atlas_handlers.insert(&sprite_sheet.file, atlas_handle);
        }
    }

    state.set(RyotSetupStates::Ready);

    info!("Finished preparing sprites");
}
