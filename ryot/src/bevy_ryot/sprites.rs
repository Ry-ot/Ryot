//! Sprite loading and drawing.
use crate::appearances::{SpriteSheetData, SpriteSheetDataSet};
use crate::bevy_ryot::InternalContentState;
use crate::layer::Layer;
use crate::position::TilePosition;
use crate::{get_decompressed_file_name, SpriteSheetConfig, SPRITE_SHEET_FOLDER};
use crate::{prelude::*, Directional};
use bevy::prelude::*;
use bevy::utils::{HashMap, StableHashSet};
use rand::Rng;
use std::path::PathBuf;
use std::time::Duration;

/// An event that is sent when a sprite sheet texture loading is completed.
#[derive(Debug, Clone, Event)]
pub(crate) struct SpriteSheetTextureWasLoaded {
    pub sprite_id: u32,
    pub atlas: TextureAtlas,
}

/// An even that is sent when a sprite sheet is requested to be loaded.
#[derive(Debug, Clone, Event)]
pub struct LoadSpriteBatch {
    pub sprite_ids: Vec<u32>,
}

/// A component that holds the loaded sprites in a vector.
#[derive(Component, Debug, Clone, Default)]
pub struct LoadedSprites(pub Vec<LoadedSprite>);

/// A struct that holds the information needed to draw a sprite.
/// It's a wrapper around a sprite sheet and a sprite id, that also holds the
/// handle to the texture atlas.
#[derive(Debug, Clone, Component)]
pub struct LoadedSprite {
    pub sprite_id: u32,
    pub group: AppearanceGroup,
    pub config: SpriteSheetConfig,
    pub sprite_sheet: SpriteSheetData,
    pub atlas_texture_handle: Handle<TextureAtlas>,
}

/// A component that holds the information needed to animate a sprite.
#[derive(Component, Default)]
pub struct AnimationSprite {
    pub timer: Timer,
    pub sprites: Vec<LoadedSprite>,
    pub initial_index: usize,
    pub phase: usize,
    pub skip: usize,
    pub total_phases: usize,
}

/// An optional component to override animation timers.
#[derive(Component, Default)]
pub struct AnimationDuration(pub Duration);

impl LoadedSprite {
    pub fn new(
        group: AppearanceGroup,
        sprite_id: u32,
        sprite_sheets: &SpriteSheetDataSet,
        atlas_handles: &HashMap<String, Handle<TextureAtlas>>,
    ) -> Option<Self> {
        let sprite_sheet = sprite_sheets.get_by_sprite_id(sprite_id)?;

        Some(Self {
            group,
            sprite_id,
            config: sprite_sheets.config,
            sprite_sheet: sprite_sheet.clone(),
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

/// A system helper that gets the LoadedSprite from the resources.
/// If the sprite sheet is not loaded yet, it emits a LoadSpriteBatch event.
/// It returns only the loaded sprites.
pub fn load_sprites<C: ContentAssets>(
    group: AppearanceGroup,
    sprite_ids: &[u32],
    content_assets: &Res<C>,
    load_sprite_batch_events: &mut EventWriter<LoadSpriteBatch>,
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

        match content_assets.get_atlas_handle(sprite_sheet.file.as_str()) {
            Some(handle) => {
                loaded.push(LoadedSprite {
                    group,
                    sprite_id: *sprite_id,
                    config: sprite_sheets.config,
                    sprite_sheet: sprite_sheet.clone(),
                    atlas_texture_handle: handle.clone(),
                });
            }
            None => {
                to_be_loaded.push(*sprite_id);
            }
        }
    }

    load_sprite_batch_events.send(LoadSpriteBatch {
        sprite_ids: to_be_loaded,
    });

    loaded
}

/// A system that listens to the LoadSpriteBatch event and loads the sprite sheets
/// from the '.png' files and sends the SpriteSheetTextureWasLoaded event once it's done.
pub(crate) fn load_sprite_sheets_from_command<C: ContentAssets>(
    content_assets: Res<C>,
    asset_server: Res<AssetServer>,
    mut load_sprite_batch_events: EventReader<LoadSpriteBatch>,
    mut sprite_sheet_texture_was_loaded: EventWriter<SpriteSheetTextureWasLoaded>,
) {
    let Some(sprite_sheets) = content_assets.sprite_sheet_data_set() else {
        return;
    };
    let to_load: StableHashSet<u32> = load_sprite_batch_events
        .read()
        .flat_map(|batch| batch.sprite_ids.iter().copied())
        .collect();

    load_sprite_textures(
        to_load.into_iter().collect(),
        &content_assets,
        &asset_server,
        sprite_sheets,
        &mut sprite_sheet_texture_was_loaded,
    );
}

/// A system that handles the loading of sprite sheets.
/// It listens to the SpriteSheetTextureWasLoaded event, adds the loaded texture atlas to the
/// atlas handles resource and stores the handle to the atlas.
pub(crate) fn store_atlases_assets_after_loading<C: PreloadedContentAssets>(
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

pub(crate) fn load_sprite_textures<C: ContentAssets>(
    sprite_ids: Vec<u32>,
    content_assets: &Res<C>,
    asset_server: &Res<AssetServer>,
    sprite_sheets: &SpriteSheetDataSet,
    sprite_sheet_texture_was_loaded: &mut EventWriter<SpriteSheetTextureWasLoaded>,
) -> Vec<(u32, TextureAtlas)> {
    let events = sprite_ids
        .iter()
        .filter_map(|sprite_id| {
            load_sprite_texture(*sprite_id, content_assets, asset_server, sprite_sheets)
        })
        .collect::<Vec<_>>();

    sprite_sheet_texture_was_loaded.send_batch(events.clone());

    events
        .iter()
        .map(|SpriteSheetTextureWasLoaded { sprite_id, atlas }| (*sprite_id, atlas.clone()))
        .collect::<Vec<(u32, TextureAtlas)>>()
}

pub(crate) fn load_sprite_texture<C: ContentAssets>(
    sprite_id: u32,
    content_assets: &Res<C>,
    asset_server: &Res<AssetServer>,
    sprite_sheets: &SpriteSheetDataSet,
) -> Option<SpriteSheetTextureWasLoaded> {
    let Some(sprite_sheet) = sprite_sheets.get_by_sprite_id(sprite_id) else {
        warn!("Sprite {} not found in sprite sheets", sprite_id);
        return None;
    };

    if content_assets
        .get_atlas_handle(sprite_sheet.file.as_str())
        .is_some()
    {
        return None;
    }

    let image_handle: Handle<Image> = asset_server.load(
        PathBuf::from(SPRITE_SHEET_FOLDER).join(get_decompressed_file_name(&sprite_sheet.file)),
    );

    let config = &sprite_sheets.config;

    Some(SpriteSheetTextureWasLoaded {
        sprite_id,
        atlas: TextureAtlas::from_grid(
            image_handle,
            sprite_sheet.get_tile_size(config).as_vec2(),
            sprite_sheet.get_columns_count(config),
            sprite_sheet.get_rows_count(config),
            None,
            None,
        ),
    })
}

/// A system that prepares the sprite assets for use in the game.
/// It loads the sprite sheets as atlases and stores their handles.
/// It also determines the loading as completed and sets the internal state to Ready.
pub(crate) fn prepare_sprites<C: PreloadedContentAssets>(
    mut content_assets: ResMut<C>,
    mut state: ResMut<NextState<InternalContentState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    debug!("Preparing sprites");

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

        content_assets.sprite_sheets().clear();
    }

    state.set(InternalContentState::Ready);

    debug!("Finished preparing sprites");
}

fn load_desired_appereance_sprite<C: ContentAssets>(
    group: AppearanceGroup,
    id: u32,
    frame_group_index: i32,
    direction: Option<&Directional>,
    content_assets: &Res<C>,
    load_sprite_batch_events: &mut EventWriter<LoadSpriteBatch>,
) -> Option<(LoadedSprite, Option<AnimationSprite>, Vec<LoadedSprite>)> {
    // If the id is 0, it means that the appearance is not set yet, so we return None
    if id == 0 {
        return None;
    }
    let Some(prepared_appearance) = content_assets
        .prepared_appearances()
        .get_for_group(group, id)
    else {
        if id > 0 {
            warn!("Appearance id {:?} for group {:?} not found", id, group);
        }
        return None;
    };

    let Some(frame_group) = prepared_appearance
        .frame_groups
        .get(frame_group_index as usize)
    else {
        warn!(
            "Frame group {:?} for appearance {:?} not found",
            frame_group_index, group
        );
        return None;
    };
    let Some(sprite_info) = frame_group.sprite_info.as_ref() else {
        warn!(
            "Sprite info for appearance {:?} and frame group {:?} not found",
            group, frame_group_index
        );
        return None;
    };

    let sprites = load_sprites(
        group,
        &sprite_info.sprite_id,
        content_assets,
        load_sprite_batch_events,
    );

    // This means that it was not loaded yet, and loading was requested
    if sprites.len() != sprite_info.sprite_id.len() {
        debug!(
            "Sprite for appearance {:?} and frame group {:?} not loaded yet expected {} got {}",
            group,
            frame_group_index,
            sprite_info.sprite_id.len(),
            sprites.len()
        );
        return None;
    }

    let direction_index = match direction {
        Some(dir) => dir.index(),
        None => 0,
    } * sprite_info.layers() as usize;
    let Some(sprite) = sprites.get(direction_index) else {
        warn!(
            "Sprite for appearance {:?} and frame group {:?} not found",
            group, frame_group_index
        );
        return None;
    };

    let animation_sprite = sprite_info
        .animation
        .as_ref()
        .map(|animation| AnimationSprite {
            timer: Timer::from_seconds(
                animation
                    .sprite_phase
                    .first()
                    .map(|phase| {
                        let range = phase.duration_min()..phase.duration_max();
                        if range.start == range.end {
                            return range.start;
                        }
                        rand::thread_rng().gen_range(range)
                    })
                    .unwrap_or(300) as f32
                    / 1000.,
                TimerMode::Repeating,
            ),
            sprites: sprites.clone(),
            phase: match animation.random_start_phase() {
                true => rand::thread_rng().gen_range(0..animation.sprite_phase.len()) as usize,
                false => animation.default_start_phase() as usize,
            },
            initial_index: direction_index,
            total_phases: animation.sprite_phase.len(),
            skip: (sprite_info.layers()
                * sprite_info.pattern_width()
                * sprite_info.pattern_height()
                * sprite_info.pattern_depth()) as usize,
        });
    Some((sprite.clone(), animation_sprite, sprites))
}

type UninitializedSpriteFilter = (With<AppearanceDescriptor>, Without<TextureAtlasSprite>);

type InitializedSpriteFilter = (
    With<AppearanceDescriptor>,
    With<TextureAtlasSprite>,
    Or<(
        With<LoadingAppearance>,
        Changed<AppearanceDescriptor>,
        Changed<Directional>,
    )>,
);

/// Update the sprite system, which updates the sprite appearance based on the
/// `AppearanceDescriptor` component. It also updates the `LoadedSprites` component
/// with the new sprites.
/// It's meant to run every frame to update the appearance of the entities when the
/// `AppearanceDescriptor` changes.
pub(crate) fn update_sprite_system<C: ContentAssets>(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &AppearanceDescriptor,
            Option<&Directional>,
            &mut LoadedSprites,
            &mut Handle<TextureAtlas>,
            &mut TextureAtlasSprite,
            Option<&mut AnimationSprite>,
        ),
        InitializedSpriteFilter,
    >,
    content_assets: Res<C>,
    mut load_sprite_batch_events: EventWriter<LoadSpriteBatch>,
) {
    for (
        entity,
        descriptor,
        direction,
        mut loaded_sprites,
        mut atlas,
        mut atlas_sprite,
        animation_sprite,
    ) in &mut query
    {
        let (sprite, anim, sprites) = match load_desired_appereance_sprite(
            descriptor.group,
            descriptor.id,
            descriptor.frame_group_index(),
            direction,
            &content_assets,
            &mut load_sprite_batch_events,
        ) {
            Some(value) => value,
            None => {
                commands.entity(entity).insert(LoadingAppearance);
                continue;
            }
        };
        *loaded_sprites = LoadedSprites(sprites.clone());
        *atlas = sprite.atlas_texture_handle.clone();
        atlas_sprite.index = sprite.get_sprite_index();
        if let Some(anim) = anim {
            if let Some(mut animation_sprite) = animation_sprite {
                *animation_sprite = anim;
            } else {
                commands.entity(entity).insert(anim);
            }
        } else {
            commands.entity(entity).remove::<AnimationSprite>();
        }

        commands.entity(entity).remove::<LoadingAppearance>();
    }
}

/// Load sprites for entities that have a `AppearanceDescriptor` and a
/// `TilePosition` component. This system will wait until all sprites are loaded
/// before initializing the entity.
/// It's meant to run only once to complete the initialization of the entities.
pub(crate) fn load_sprite_system<C: ContentAssets>(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &TilePosition,
            &AppearanceDescriptor,
            &Layer,
            Option<&Directional>,
        ),
        UninitializedSpriteFilter,
    >,
    content_assets: Res<C>,
    mut load_sprite_batch_events: EventWriter<LoadSpriteBatch>,
) {
    for (entity, position, descriptor, layer, direction) in &mut query {
        let (sprite, anim, sprites) = match load_desired_appereance_sprite(
            descriptor.group,
            descriptor.id,
            descriptor.frame_group_index(),
            direction,
            &content_assets,
            &mut load_sprite_batch_events,
        ) {
            Some(value) => value,
            None => {
                commands.entity(entity).insert(LoadingAppearance);
                continue;
            }
        };

        commands
            .entity(entity)
            .insert(LoadedSprites(sprites.clone()))
            .insert(SpriteSheetBundle {
                transform: Transform::from_translation(position.to_vec3(layer)),
                sprite: TextureAtlasSprite {
                    index: sprite.get_sprite_index(),
                    anchor: RYOT_ANCHOR,
                    ..Default::default()
                },
                texture_atlas: sprite.atlas_texture_handle.clone(),
                ..default()
            })
            .remove::<LoadingAppearance>();

        if let Some(anim) = anim {
            commands.entity(entity).insert(anim);
        } else {
            commands.entity(entity).remove::<AnimationSprite>();
        }
    }
}

/// A system that animates the sprites based on the `AnimationSprite` component.
/// It's meant to run every frame to update the animation of the entities.
/// It will only run if the entity has a `TextureAtlasSprite` and an `AnimationSprite` component.
pub fn animate_sprite_system(
    time: Res<Time>,
    mut sprites_to_animate: Query<(
        &mut AnimationSprite,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        Option<&AnimationDuration>,
    )>,
) {
    for (mut anim, mut atlas, mut atlas_sprite, duration) in &mut sprites_to_animate {
        if let Some(duration) = duration {
            let frame_duration = duration.0 / anim.total_phases as u32;
            if anim.timer.duration() != frame_duration {
                anim.timer.set_duration(frame_duration)
            }
        }
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            anim.phase += 1;
            if anim.phase >= anim.total_phases {
                anim.phase = 0;
            }
            let Some(sprite) = anim
                .sprites
                .get(anim.initial_index + anim.phase * anim.skip)
            else {
                return;
            };
            *atlas = sprite.atlas_texture_handle.clone();
            atlas_sprite.index = sprite.get_sprite_index();
        }
    }
}
