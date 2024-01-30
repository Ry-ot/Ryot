//! Sprite loading and drawing.
use crate::appearances::{SpriteSheetData, SpriteSheetDataSet};
use crate::bevy_ryot::InternalContentState;
use crate::prelude::tile_grid::TileGrid;
use crate::prelude::ContentAssets;
use crate::tile_grid::OffsetStrategy;
use crate::{get_decompressed_file_name, SpriteSheetConfig, SPRITE_SHEET_FOLDER};
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::path::PathBuf;

/// A command that is sent as a bevy event to trigger the loading of the sprite sheets
/// for a given set of sprite ids.
#[derive(Debug, Clone, Event)]
pub struct LoadSpriteSheetTextureCommand {
    pub sprite_ids: Vec<u32>,
}

/// An event that is sent when a sprite sheet texture loading is completed.
#[derive(Debug, Clone, Event)]
pub(crate) struct SpriteSheetTextureWasLoaded {
    pub sprite_id: u32,
    pub atlas: TextureAtlas,
}

/// A struct that holds the information needed to draw a sprite.
/// It's a wrapper around a sprite sheet and a sprite id, that also holds the
/// handle to the texture atlas.
#[derive(Debug)]
pub struct LoadedSprite {
    pub sprite_id: u32,
    pub sprite_sheet: SpriteSheetData,
    pub config: SpriteSheetConfig,
    pub atlas_texture_handle: Handle<TextureAtlas>,
}

impl LoadedSprite {
    pub fn new(
        sprite_id: u32,
        sprite_sheets: &SpriteSheetDataSet,
        atlas_handles: &HashMap<String, Handle<TextureAtlas>>,
    ) -> Option<Self> {
        let sprite_sheet = sprite_sheets.get_by_sprite_id(sprite_id)?;

        Some(Self {
            sprite_id,
            sprite_sheet: sprite_sheet.clone(),
            config: sprite_sheets.config,
            atlas_texture_handle: atlas_handles.get(&sprite_sheet.file)?.clone(),
        })
    }

    pub fn get_sprite_index(&self) -> usize {
        self.sprite_sheet
            .get_sprite_index(self.sprite_id)
            .expect("Sprite must exist in sheet")
    }

    pub fn get_sprite_size(&self) -> Vec2 {
        self.sprite_sheet.layout.get_size(&self.config).as_vec2()
    }
}

/// A system that gets the LoadedSprite from the resources.
/// If the sprite sheet is not loaded yet, it sends a LoadSpriteSheetTextureCommand event.
/// It returns only the loaded sprites.
pub fn load_sprites<C: ContentAssets>(
    sprite_ids: &[u32],
    content_assets: Res<C>,
    mut build_spr_sheet_texture_cmd: EventWriter<LoadSpriteSheetTextureCommand>,
) -> Vec<LoadedSprite> {
    let Some(sprite_sheets) = content_assets.sprite_sheet_data_set() else {
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

        let Some(handle) = content_assets.get_atlas_handle(sprite_sheet.file.as_str()) else {
            to_be_loaded.push(*sprite_id);
            continue;
        };

        loaded.push(LoadedSprite {
            sprite_id: *sprite_id,
            sprite_sheet: sprite_sheet.clone(),
            config: sprite_sheets.config,
            atlas_texture_handle: handle.clone(),
        });
    }

    build_spr_sheet_texture_cmd.send(LoadSpriteSheetTextureCommand {
        sprite_ids: to_be_loaded,
    });

    loaded
}

/// A system that listens to the LoadSpriteSheetTextureCommand event, loads the sprite sheet
/// from the '.png' files and sends the SpriteSheetTextureWasLoaded event once it's done.
pub(crate) fn load_sprite_sheets_from_command<C: ContentAssets>(
    content_assets: Res<C>,
    asset_server: Res<AssetServer>,
    mut sprite_sheet_texture_was_loaded: EventWriter<SpriteSheetTextureWasLoaded>,
    mut build_spr_sheet_texture_cmd: EventReader<LoadSpriteSheetTextureCommand>,
) {
    let Some(sprite_sheets) = content_assets.sprite_sheet_data_set() else {
        return;
    };

    for LoadSpriteSheetTextureCommand { sprite_ids } in build_spr_sheet_texture_cmd.read() {
        for sprite_id in sprite_ids {
            let Some(sprite_sheet) = sprite_sheets.get_by_sprite_id(*sprite_id) else {
                warn!("Sprite {} not found in sprite sheets", sprite_id);
                continue;
            };

            if content_assets
                .get_atlas_handle(sprite_sheet.file.as_str())
                .is_some()
            {
                continue;
            }

            let image_handle: Handle<Image> = asset_server.load(
                PathBuf::from(SPRITE_SHEET_FOLDER)
                    .join(get_decompressed_file_name(&sprite_sheet.file)),
            );

            let config = &sprite_sheets.config;

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

/// A system that handles the loading of sprite sheets.
/// It listens to the SpriteSheetTextureWasLoaded event, adds the loaded texture atlas to the   
/// atlas handles resource and stores the handle to the atlas.
pub(crate) fn store_atlases_assets_after_loading<C: ContentAssets>(
    mut content_assets: ResMut<C>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut sprite_sheet_texture_was_loaded: EventReader<SpriteSheetTextureWasLoaded>,
) {
    for SpriteSheetTextureWasLoaded { sprite_id, atlas } in sprite_sheet_texture_was_loaded.read() {
        let sprite_sheet = content_assets
            .sprite_sheet_data_set()
            .as_ref()
            .expect("Sprite sheets must be loaded")
            .get_by_sprite_id(*sprite_id)
            .expect("Sprite must exist in sheet")
            .clone();

        if content_assets
            .get_atlas_handle(&sprite_sheet.file)
            .is_some()
        {
            continue;
        }

        let atlas_handle = texture_atlases.add(atlas.clone());
        content_assets.insert_atlas_handle(&sprite_sheet.file, atlas_handle);
    }
}

/// Primitive draw function, to be replaced with a more sophisticated drawing system.
pub fn draw_sprite(pos: Vec3, sprite: &LoadedSprite, commands: &mut Commands, tile_grid: TileGrid) {
    let tile_pos = tile_grid.get_display_position_from_tile_pos(
        pos,
        OffsetStrategy::ProportionalToSpriteSize(sprite.get_sprite_size()),
    );

    let Some(tile_pos) = tile_pos else {
        return;
    };

    commands.spawn(build_sprite_bundle(
        sprite.atlas_texture_handle.clone(),
        tile_pos,
        sprite.get_sprite_index(),
    ));
}

/// A helper function to build a sprite bundle from a sprite sheet handle, a translation and a sprite index.
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

/// A system that prepares the sprite assets for use in the game.
/// It loads the sprite sheets as atlases and stores their handles.
/// It also determines the loading as completed and sets the internal state to Ready.
pub(crate) fn sprites_preparer<C: ContentAssets>(
    mut content_assets: ResMut<C>,
    mut state: ResMut<NextState<InternalContentState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Preparing sprites");

    if !content_assets.sprite_sheets().is_empty() {
        let Some(sheets) = content_assets.sprite_sheet_data_set().clone() else {
            panic!("Sprite sheets configs were not setup.");
        };

        for (file, handle) in content_assets.sprite_sheets().clone() {
            let file = match file.strip_prefix("sprite-sheets/") {
                Some(file) => file,
                None => &file,
            };

            if content_assets.get_atlas_handle(file).is_some() {
                warn!("Skipping file {}: it's already loaded", file);
                continue;
            }

            let Some(sprite_sheet) = sheets.get_for_file(file) else {
                warn!("Skipping file {}: it's not in sprite sheets", file);
                continue;
            };

            let atlas = TextureAtlas::from_grid(
                handle.clone(),
                sprite_sheet.get_tile_size(&sheets.config).as_vec2(),
                sprite_sheet.get_columns_count(&sheets.config),
                sprite_sheet.get_rows_count(&sheets.config),
                None,
                None,
            );

            let atlas_handle = texture_atlases.add(atlas.clone());
            content_assets.insert_atlas_handle(&sprite_sheet.file, atlas_handle);
        }
    }

    state.set(InternalContentState::Ready);

    info!("Finished preparing sprites");
}
