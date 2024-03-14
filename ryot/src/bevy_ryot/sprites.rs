//! Sprite loading and drawing.
use crate::appearances::{SpriteSheetData, SpriteSheetDataSet};
use crate::bevy_ryot::InternalContentState;
use crate::{get_decompressed_file_name, SPRITE_SHEET_FOLDER};
use crate::{prelude::*, Directional};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::hashbrown::HashSet;
use bevy::utils::{FixedState, HashMap};
use itertools::Either;
use rayon::prelude::*;

use std::path::PathBuf;

pub const SPRITE_BASE_SIZE: UVec2 = UVec2::new(32, 32);

use self::drawing::Elevation;
use self::sprite_animations::{
    AnimationDescriptor, AnimationSprite, SpriteAnimationExt, SynchronizedAnimationTimers,
};

/// An event that is sent when a sprite sheet texture loading is completed.
#[derive(Debug, Clone, Event)]
pub(crate) struct SpriteSheetTextureWasLoaded {
    pub sprite_id: u32,
    pub texture: Handle<Image>,
}

/// An even that is sent when a sprite sheet is requested to be loaded.
#[derive(Debug, Clone, Event)]
pub struct LoadSpriteBatch {
    pub sprite_ids: Vec<u32>,
}

/// A struct that holds the information needed to draw a sprite.
/// It's a wrapper around a sprite sheet and a sprite id, that also holds the
/// handle to the texture atlas.
#[derive(Debug, Clone, Component)]
pub struct LoadedSprite {
    pub sprite_id: u32,
    pub group: AppearanceGroup,
    pub sprite_sheet: SpriteSheetData,
    pub texture: Handle<Image>,
}

impl LoadedSprite {
    pub fn new(
        group: AppearanceGroup,
        sprite_id: u32,
        sprite_sheets: &SpriteSheetDataSet,
        textures: &HashMap<String, Handle<Image>>,
    ) -> Option<Self> {
        let sprite_sheet = sprite_sheets.get_by_sprite_id(sprite_id)?;
        let texture = textures.get(&sprite_sheet.file)?;
        Some(Self {
            group,
            sprite_id,
            sprite_sheet: sprite_sheet.clone(),
            texture: texture.clone(),
        })
    }

    pub fn get_sprite_index(&self) -> usize {
        self.sprite_sheet
            .get_sprite_index(self.sprite_id)
            .expect("Sprite must exist in sheet")
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

    let (to_be_loaded, loaded) =
        sprite_ids.par_iter().partition_map(|sprite_id| {
            match sprite_sheets.get_by_sprite_id(*sprite_id) {
                Some(sprite_sheet) => {
                    match content_assets.get_texture(sprite_sheet.file.as_str()) {
                        Some(texture) => Either::Right(LoadedSprite {
                            group,
                            sprite_id: *sprite_id,
                            sprite_sheet: sprite_sheet.clone(),
                            texture: texture.clone(),
                        }),
                        None => Either::Left(*sprite_id),
                    }
                }
                None => {
                    warn!("Sprite {} not found in sprite sheets", sprite_id);
                    Either::Left(*sprite_id)
                }
            }
        });

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
    let to_load: HashSet<u32, FixedState> = load_sprite_batch_events
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
    mut sprite_sheet_texture_was_loaded: EventReader<SpriteSheetTextureWasLoaded>,
) {
    for SpriteSheetTextureWasLoaded { sprite_id, texture } in sprite_sheet_texture_was_loaded.read()
    {
        let sprite_sheet = content_assets
            .sprite_sheet_data_set()
            .as_ref()
            .expect("Sprite sheets must be loaded")
            .get_by_sprite_id(*sprite_id)
            .expect("Sprite must exist in sheet")
            .clone();

        if content_assets.get_texture(&sprite_sheet.file).is_some() {
            continue;
        }

        content_assets.insert_texture(&sprite_sheet.file, texture.clone());
    }
}

pub(crate) fn load_sprite_textures<C: ContentAssets>(
    sprite_ids: Vec<u32>,
    content_assets: &Res<C>,
    asset_server: &Res<AssetServer>,
    sprite_sheets: &SpriteSheetDataSet,
    sprite_sheet_texture_was_loaded: &mut EventWriter<SpriteSheetTextureWasLoaded>,
) {
    let events = sprite_ids
        .iter()
        .filter_map(|sprite_id| {
            load_sprite_texture(*sprite_id, content_assets, asset_server, sprite_sheets)
        })
        .collect::<Vec<_>>();

    sprite_sheet_texture_was_loaded.send_batch(events.clone());
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
        .get_texture(sprite_sheet.file.as_str())
        .is_some()
    {
        return None;
    }

    let texture: Handle<Image> = asset_server.load(
        PathBuf::from(SPRITE_SHEET_FOLDER).join(get_decompressed_file_name(&sprite_sheet.file)),
    );

    Some(SpriteSheetTextureWasLoaded { sprite_id, texture })
}

/// A system that prepares the sprite assets for use in the game.
/// It loads the sprite sheets as atlases and stores their handles.
/// It also determines the loading as completed and sets the internal state to Ready.
pub(crate) fn prepare_sprites<C: PreloadedContentAssets>(
    mut content_assets: ResMut<C>,
    mut state: ResMut<NextState<InternalContentState>>,
) {
    if !content_assets.sprite_sheets().is_empty() {
        for (file, texture) in content_assets.sprite_sheets().clone() {
            let file = match file.strip_prefix(&(SPRITE_SHEET_FOLDER.to_string() + "/")) {
                Some(file) => file,
                None => &file,
            };

            if content_assets.get_texture(file).is_some() {
                warn!("Skipping file {}: it's already loaded", file);
                continue;
            }

            content_assets.insert_texture(file, texture.clone());
        }
    }
    state.set(InternalContentState::Ready);
}

#[allow(clippy::too_many_arguments)]
fn load_desired_appereance_sprite<C: ContentAssets>(
    group: AppearanceGroup,
    id: u32,
    frame_group_index: i32,
    animation_sprite: Option<&AnimationSprite>,
    direction: Option<&Directional>,
    content_assets: &Res<C>,
    load_sprite_batch_events: &mut EventWriter<LoadSpriteBatch>,
    synced_timers: &mut ResMut<SynchronizedAnimationTimers>,
) -> Option<(LoadedSprite, Option<AnimationSprite>)> {
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

    let animation_sprite = animation_sprite.cloned().or_else(|| {
        sprite_info.animation.as_ref().map(|animation| {
            let key = animation.get_animation_key();
            let descriptor = AnimationDescriptor {
                sprites: sprites.clone(),
                initial_index: direction_index,
                skip: (sprite_info.layers()
                    * sprite_info.pattern_width()
                    * sprite_info.pattern_height()
                    * sprite_info.pattern_depth()) as usize,
            };
            if animation.synchronized() {
                synced_timers
                    .try_insert(key.clone(), key.default_state())
                    .ok();
                AnimationSprite::Synchronized { key, descriptor }
            } else {
                AnimationSprite::Independent {
                    key: key.clone(),
                    descriptor,
                    state: key.default_state(),
                }
            }
        })
    });
    Some((sprite.clone(), animation_sprite))
}

/// Update the sprite system, which updates the sprite appearance based on the
/// `AppearanceDescriptor` component. It also updates the `LoadedSprites` component
/// with the new sprites.
/// It's meant to run every frame to update the appearance of the entities when the
/// `AppearanceDescriptor` changes.
pub(crate) fn sprite_material_system<C: ContentAssets>(
    mut commands: Commands,
    content_assets: Res<C>,
    layouts: ResMut<Assets<TextureAtlasLayout>>,
    sprite_meshes: Res<SpriteMeshes>,
    mut materials: ResMut<Assets<SpriteMaterial>>,
    mut material_cache: Local<HashMap<u32, Handle<SpriteMaterial>>>,
    mut query: Query<(Entity, &LoadedSprite, Has<Transform>), Changed<LoadedSprite>>,
) {
    for (entity, sprite, has_transform) in &mut query {
        let Some(layout) = content_assets.get_atlas_layout(sprite.sprite_sheet.layout) else {
            warn!(
                "Atlas layout for sprite layout {:?} not found",
                sprite.sprite_sheet.layout
            );
            continue;
        };
        let Some(layout) = layouts.get(layout) else {
            warn!(
                "Atlas layout for sprite layout {:?} not found",
                sprite.sprite_sheet.layout
            );
            continue;
        };
        let Some(mesh_handle) = sprite_meshes.get(&sprite.sprite_sheet.layout) else {
            warn!(
                "Mesh for sprite layout {:?} not found",
                sprite.sprite_sheet.layout
            );
            continue;
        };

        let material = material_cache
            .entry(sprite.sprite_id)
            .or_insert_with(|| {
                materials.add(SpriteMaterial {
                    texture: sprite.texture.clone(),
                    index: sprite.get_sprite_index() as u32,
                    counts: sprite
                        .sprite_sheet
                        .layout
                        .get_counts(layout.size, tile_size().as_vec2()),
                })
            })
            .clone();

        if has_transform {
            commands
                .entity(entity)
                .insert((Mesh2dHandle(mesh_handle.clone()), material));
        } else {
            let elevation =
                Elevation::new(0., sprite.sprite_sheet.layout.get_height(&SPRITE_BASE_SIZE));
            commands.entity(entity).insert((
                elevation,
                MaterialMesh2dBundle {
                    // Translation is computed in the position system
                    mesh: Mesh2dHandle(mesh_handle.clone()),
                    material,
                    ..default()
                },
            ));
        }
    }
}

pub(crate) fn update_sprite_system<C: ContentAssets>() {}

/// Load sprites for entities that have a `AppearanceDescriptor` and a
/// `TilePosition` component. This system will wait until all sprites are loaded
/// before initializing the entity.
/// It's meant to run only once to complete the initialization of the entities.
#[allow(clippy::type_complexity)]
pub(crate) fn load_sprite_system<C: ContentAssets>(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            Ref<AppearanceDescriptor>,
            Option<&AnimationSprite>,
            Option<Ref<Directional>>,
        ),
        Or<(
            Changed<AppearanceDescriptor>,
            Changed<Directional>,
            Without<LoadedSprite>,
        )>,
    >,
    content_assets: Res<C>,
    mut load_sprite_batch_events: EventWriter<LoadSpriteBatch>,
    mut synced_timers: ResMut<SynchronizedAnimationTimers>,
) {
    query
        .iter_mut()
        .for_each(|(entity, descriptor, mut animation_sprite, direction)| {
            if descriptor.is_changed()
                || direction
                    .as_ref()
                    .map(|direction| direction.is_changed())
                    .unwrap_or(false)
            {
                commands.entity(entity).remove::<AnimationSprite>();
                animation_sprite = None;
            }
            let (sprite, anim) = match load_desired_appereance_sprite(
                descriptor.group,
                descriptor.id,
                descriptor.frame_group_index(),
                animation_sprite,
                direction.as_deref(),
                &content_assets,
                &mut load_sprite_batch_events,
                &mut synced_timers,
            ) {
                Some(value) => value,
                None => {
                    return;
                }
            };

            match anim {
                Some(anim) => {
                    let sprite = {
                        let (state, descriptor) = match &anim {
                            AnimationSprite::Independent {
                                state, descriptor, ..
                            } => (state, descriptor),
                            AnimationSprite::Synchronized { key, descriptor } => {
                                let state = synced_timers
                                    .get(key)
                                    .expect("Synchronized timer not found");
                                (state, descriptor)
                            }
                        };
                        descriptor
                            .sprites
                            .get(descriptor.initial_index + state.current_phase * descriptor.skip)
                            .expect("Sprite not found")
                            .clone()
                    };
                    commands.entity(entity).insert((sprite, anim));
                }
                None => {
                    commands
                        .entity(entity)
                        .insert(sprite)
                        .remove::<AnimationSprite>();
                }
            }
        });
}

#[derive(AsBindGroup, TypePath, Asset, Debug, Clone)]
pub struct SpriteMaterial {
    #[uniform(0)]
    pub index: u32,
    #[uniform(0)]
    pub counts: Vec2,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for SpriteMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://ryot/bevy_ryot/shaders/sprite.wgsl".into()
    }
}
