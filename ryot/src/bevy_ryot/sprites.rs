//! Sprite loading and drawing.
use crate::appearances::{SpriteSheet, SpriteSheetSet};
use crate::prelude::tile_grid::TileGrid;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::asset_collection::AssetCollection;
use std::marker::PhantomData;
use std::path::PathBuf;

use crate::bevy_ryot::{InternalContentState, Sprites};
use crate::{get_decompressed_file_name, SpriteSheetConfig, SPRITE_SHEET_FOLDER};
use bevy_asset_loader::prelude::*;

/// A trait that represents the sprite assets of a game.
/// It expects the type to implement AssetCollection and Resource.
/// It's a Bevy resource that holds the handles to the assets loaded by bevy_asset_loader.
///
/// Assets contains a map of sprite sheets (loaded from a *.png file) with the sprite sheet name
/// as key and the handle to the sprite sheet image as value.
///
/// Also contains a map of texture atlases, built from the loaded images and needed by Bevy to
/// efficiently deal with sprite-sheet based textures.
pub trait SpriteAssets: Resource + AssetCollection + Send + Sync + 'static {
    fn sprite_sheets(&self) -> &HashMap<String, Handle<Image>>;
    fn atlas_handles(&self) -> &HashMap<String, Handle<TextureAtlas>>;
    fn insert_atlas_handle(&mut self, file: &str, handle: Handle<TextureAtlas>);
    fn get_atlas_handle(&self, file: &str) -> Option<&Handle<TextureAtlas>>;
}

/// A plugin that registers implementations of SpriteAssets and loads them.
/// It inits the necessary resources and adds the necessary systems and plugins to load
/// the sprite assets.
///
/// It also manages the loading state of the sprite assets and the lifecycle of the sprites.
///
/// It also adds the necessary systems to draw sprites.
pub struct SpritesPlugin<T: SpriteAssets> {
    _marker: PhantomData<T>,
}

impl<T: SpriteAssets> SpritesPlugin<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: SpriteAssets> Default for SpritesPlugin<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: SpriteAssets + Default> Plugin for SpritesPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<T>();
        app.add_loading_state(
            LoadingState::new(InternalContentState::LoadingSprites)
                .continue_to_state(InternalContentState::PreparingSprites)
                .load_collection::<T>(),
        )
        .add_systems(
            OnEnter(InternalContentState::PreparingSprites),
            sprites_preparer::<T>,
        );

        app.add_event::<LoadSpriteSheetTextureCommand>()
            .add_event::<SpriteSheetTextureWasLoaded>()
            .add_systems(Update, load_sprite_sheets_from_command::<T>)
            .add_systems(Update, store_atlases_assets_after_loading::<T>);
    }
}

/// A command that is sent as a bevy event to trigger the loading of the sprite sheets
/// for a given set of sprite ids.
#[derive(Debug, Clone, Event)]
pub struct LoadSpriteSheetTextureCommand {
    pub sprite_ids: Vec<u32>,
}

/// An event that is sent when a sprite sheet texture loading is completed.
#[derive(Debug, Clone, Event)]
struct SpriteSheetTextureWasLoaded {
    pub sprite_id: u32,
    pub atlas: TextureAtlas,
}

/// A struct that holds the information needed to draw a sprite.
/// It's a wrapper around a sprite sheet and a sprite id, that also holds the
/// handle to the texture atlas.
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
        atlas_handles: &HashMap<String, Handle<TextureAtlas>>,
    ) -> Option<Self> {
        let sprite_sheet = sprite_sheets.get_by_sprite_id(sprite_id)?;

        Some(Self {
            sprite_id,
            sprite_sheet: sprite_sheet.clone(),
            config: sprite_sheets.sheet_config,
            atlas_texture_handle: atlas_handles.get(&sprite_sheet.file)?.clone(),
        })
    }

    pub fn get_sprite_index(&self) -> usize {
        self.sprite_sheet
            .get_sprite_index(self.sprite_id)
            .expect("Sprite must exist in sheet")
    }
}

/// A system that gets the LoadedSprite from the resources.
/// If the sprite sheet is not loaded yet, it sends a LoadSpriteSheetTextureCommand event.
/// It returns only the loaded sprites.
pub fn load_sprites<T: SpriteAssets>(
    sprite_ids: &[u32],
    sprites: Res<Sprites>,
    sprite_assets: ResMut<T>,
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

        let Some(handle) = sprite_assets.get_atlas_handle(sprite_sheet.file.as_str()) else {
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

/// A system that listens to the LoadSpriteSheetTextureCommand event, loads the sprite sheet
/// from the '.png' files and sends the SpriteSheetTextureWasLoaded event once it's done.
fn load_sprite_sheets_from_command<T: SpriteAssets>(
    sprites: Res<Sprites>,
    sprite_assets: ResMut<T>,
    asset_server: Res<AssetServer>,
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

            if sprite_assets
                .get_atlas_handle(sprite_sheet.file.as_str())
                .is_some()
            {
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

/// A system that handles the loading of sprite sheets.
/// It listens to the SpriteSheetTextureWasLoaded event, adds the loaded texture atlas to the   
/// atlas handles resource and stores the handle to the atlas.
fn store_atlases_assets_after_loading<T: SpriteAssets>(
    sprites: Res<Sprites>,
    mut sprite_assets: ResMut<T>,
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

        if sprite_assets
            .get_atlas_handle(sprite_sheet.file.as_str())
            .is_some()
        {
            continue;
        }

        let atlas_handle = texture_atlases.add(atlas.clone());
        sprite_assets.insert_atlas_handle(&sprite_sheet.file, atlas_handle);
    }
}

/// Primitive draw function, to be replaced with a more sophisticated drawing system.
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
fn sprites_preparer<T: SpriteAssets>(
    sprites: Res<Sprites>,
    mut sprite_assets: ResMut<T>,
    mut state: ResMut<NextState<InternalContentState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Preparing sprites");

    if !sprite_assets.sprite_sheets().is_empty() {
        let Some(sheets) = &sprites.sheets else {
            panic!("Sprite sheets configs were not setup.");
        };

        for (file, handle) in sprite_assets.sprite_sheets().clone() {
            let file = match file.strip_prefix("sprite-sheets/") {
                Some(file) => file,
                None => &file,
            };

            if sprite_assets.get_atlas_handle(file).is_some() {
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
            sprite_assets.insert_atlas_handle(&sprite_sheet.file, atlas_handle);
        }
    }

    state.set(InternalContentState::Ready);

    info!("Finished preparing sprites");
}
