//! Sprite loading and drawing.
use crate::appearances::{SpriteInfo, SpriteSheetData, SpriteSheetDataSet};
use crate::bevy_ryot::InternalContentState;
use crate::{get_decompressed_file_name, SPRITE_SHEET_FOLDER};
use crate::{prelude::*, Directional};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::{HashMap, HashSet};
use itertools::Itertools;

use std::path::PathBuf;

pub const SPRITE_BASE_SIZE: UVec2 = UVec2::new(32, 32);

use self::drawing::Elevation;
use self::sprite_animations::{
    AnimationDescriptor, AnimationKey, AnimationSprite, SpriteAnimationExt,
    SynchronizedAnimationTimers,
};

pub struct LoadedAppearance {
    pub sprites: Vec<LoadedSprite>,
    pub layers: u32,
    pub animation: Option<(AnimationKey, AnimationDescriptor)>,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct LoadedAppearances(pub HashMap<AppearanceDescriptor, LoadedAppearance>);

/// A struct that holds the information needed to draw a sprite.
/// It's a wrapper around a sprite sheet and a sprite id, that also holds the
/// handle to the texture atlas.
#[derive(Debug, Clone)]
pub struct LoadedSprite {
    pub sprite_id: u32,
    pub group: AppearanceGroup,
    pub sprite_sheet: SpriteSheetData,
    pub texture: Handle<Image>,
    pub material: Handle<SpriteMaterial>,
    pub mesh: Handle<Mesh>,
}

impl LoadedSprite {
    pub fn new(
        group: AppearanceGroup,
        sprite_id: u32,
        sprite_sheets: &SpriteSheetDataSet,
        textures: &HashMap<String, Handle<Image>>,
        material: &Handle<SpriteMaterial>,
        mesh: &Handle<Mesh>,
    ) -> Option<Self> {
        let sprite_sheet = sprite_sheets.get_by_sprite_id(sprite_id)?;
        let texture = textures.get(&sprite_sheet.file)?;
        Some(Self {
            group,
            sprite_id,
            sprite_sheet: sprite_sheet.clone(),
            texture: texture.clone(),
            material: material.clone(),
            mesh: mesh.clone(),
        })
    }

    pub fn get_sprite_index(&self) -> usize {
        self.sprite_sheet
            .get_sprite_index(self.sprite_id)
            .expect("Sprite must exist in sheet")
    }
}

pub fn load_sprites<C: ContentAssets>(
    group: AppearanceGroup,
    sprite_ids: Vec<u32>,
    content_assets: &Res<C>,
    layouts: &Res<Assets<TextureAtlasLayout>>,
    sprite_meshes: &Res<SpriteMeshes>,
    materials: &mut ResMut<Assets<SpriteMaterial>>,
    asset_server: &Res<AssetServer>,
) -> Vec<LoadedSprite> {
    let Some(sprite_sheets) = content_assets.sprite_sheet_data_set() else {
        warn!("No sprite sheets loaded");
        return vec![];
    };

    load_sprite_textures(sprite_ids, content_assets, asset_server, sprite_sheets)
        .iter()
        .filter_map(|(sprite_id, texture)| {
            let Some(sprite_sheet) = sprite_sheets.get_by_sprite_id(*sprite_id) else {
                warn!("Sprite {} not found in sprite sheets", sprite_id);
                return None;
            };

            let layout = content_assets
                .get_atlas_layout(sprite_sheet.layout)
                .unwrap_or_default();
            let layout = layouts
                .get(&layout)
                .unwrap_or_else(|| panic!("Layout not found: {:?}", layout));
            let Some(mesh_handle) = sprite_meshes.get(&sprite_sheet.layout) else {
                panic!("Mesh for sprite layout {:?} not found", sprite_sheet.layout);
            };

            Some(LoadedSprite {
                group,
                sprite_id: *sprite_id,
                sprite_sheet: sprite_sheet.clone(),
                texture: texture.clone(),
                mesh: mesh_handle.clone(),
                material: materials.add(SpriteMaterial {
                    texture: texture.clone(),
                    counts: sprite_sheet
                        .layout
                        .get_counts(layout.size, tile_size().as_vec2()),
                    index: sprite_sheet
                        .get_sprite_index(*sprite_id)
                        .expect("Sprite must exist in sheet") as u32,
                }),
            })
        })
        .collect()
}

pub(crate) fn load_sprite_textures<C: ContentAssets>(
    sprite_ids: Vec<u32>,
    content_assets: &Res<C>,
    asset_server: &Res<AssetServer>,
    sprite_sheets: &SpriteSheetDataSet,
) -> Vec<(u32, Handle<Image>)> {
    sprite_ids
        .iter()
        .filter_map(|sprite_id| {
            load_sprite_texture(*sprite_id, content_assets, asset_server, sprite_sheets)
                .map(|texture| (*sprite_id, texture))
        })
        .collect()
}

pub(crate) fn load_sprite_texture<C: ContentAssets>(
    sprite_id: u32,
    content_assets: &Res<C>,
    asset_server: &Res<AssetServer>,
    sprite_sheets: &SpriteSheetDataSet,
) -> Option<Handle<Image>> {
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

    Some(texture)
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

/// A system that ensures that all entities with an AppearanceDescriptor have a SpriteMaterial mesh bundle.
pub(crate) fn ensure_appearance_initialized(
    mut commands: Commands,
    query: Query<Entity, (With<AppearanceDescriptor>, Without<Handle<SpriteMaterial>>)>,
) {
    query.iter().for_each(|entity| {
        commands.entity(entity).insert((
            MaterialMesh2dBundle::<SpriteMaterial>::default(),
            SpriteLayout::default(),
            Elevation::default(),
        ));
    });
}

type ChangingAppearanceFilter = Or<(Changed<AppearanceDescriptor>, Changed<Directional>)>;

pub(crate) fn update_sprite_system(
    mut commands: Commands,
    mut q_updated: Query<
        (
            &AppearanceDescriptor,
            Option<&Directional>,
            &mut Elevation,
            &mut SpriteLayout,
            &mut Mesh2dHandle,
            &mut Handle<SpriteMaterial>,
        ),
        ChangingAppearanceFilter,
    >,
    q_maybe_animated: Query<(Entity, &AppearanceDescriptor), ChangingAppearanceFilter>,
    loaded_appereances: Res<LoadedAppearances>,
    mut synced_timers: ResMut<SynchronizedAnimationTimers>,
) {
    q_updated.par_iter_mut().for_each(
        |(descriptor, direction, mut elevation, mut layout, mut mesh, mut material)| {
            if descriptor.id == 0 {
                return;
            }
            let Some(loaded_appearance) = loaded_appereances.get(descriptor) else {
                warn!("BUG: Loaded appearance for {:?} not found.", descriptor);
                return;
            };

            let direction_index = match direction {
                Some(dir) => dir.index(),
                None => 0,
            } * loaded_appearance.layers as usize;

            let Some(sprite) = loaded_appearance.sprites.get(direction_index) else {
                warn!(
                    "Sprite for appearance {:?} not found for direction {:?}",
                    descriptor, direction
                );
                return;
            };
            elevation.elevation = 0.;
            elevation.base_height = sprite.sprite_sheet.layout.get_height(&SPRITE_BASE_SIZE);
            *layout = sprite.sprite_sheet.layout;
            *mesh = Mesh2dHandle(sprite.mesh.clone());
            *material = sprite.material.clone();
        },
    );

    q_maybe_animated.iter().for_each(|(entity, descriptor)| {
        let Some(loaded_appearance) = loaded_appereances.get(descriptor) else {
            warn!("BUG: Loaded appearance for {:?} not found.", descriptor);
            return;
        };
        let Some((ref key, ref descriptor)) = loaded_appearance.animation else {
            commands.entity(entity).remove::<AnimationSprite>();
            return;
        };
        commands
            .entity(entity)
            .insert(AnimationSprite::from_key_and_descriptor(key, descriptor));

        if descriptor.synchronized {
            synced_timers
                .try_insert(key.clone(), key.default_state())
                .ok();
        }
    });
}

#[derive(Event)]
pub struct LoadAppearanceEvent(pub AppearanceDescriptor);

pub(crate) fn load_from_entities_system(
    query: Query<&AppearanceDescriptor, Changed<AppearanceDescriptor>>,
    loaded_appearances: Res<LoadedAppearances>,
    mut events: EventWriter<LoadAppearanceEvent>,
) {
    query
        .iter()
        .unique()
        .filter(|descriptor| !loaded_appearances.contains_key(*descriptor))
        .cloned()
        .for_each(|descriptor| {
            events.send(LoadAppearanceEvent(descriptor));
        });
}

/// Load sprites for entities that have a `AppearanceDescriptor` and a
/// `TilePosition` component. This system will wait until all sprites are loaded
/// before initializing the entity.
/// It's meant to run only once to complete the initialization of the entities.
pub(crate) fn load_sprite_system<C: ContentAssets>(
    content_assets: Res<C>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    sprite_meshes: Res<SpriteMeshes>,
    mut materials: ResMut<Assets<SpriteMaterial>>,
    mut loaded_appearances: ResMut<LoadedAppearances>,
    asset_server: Res<AssetServer>,
    mut events: EventReader<LoadAppearanceEvent>,
) {
    let descriptors = events
        .read()
        .map(|LoadAppearanceEvent(descriptor)| descriptor)
        .filter(|descriptor| descriptor.id != 0)
        .collect::<HashSet<_>>()
        .difference(&loaded_appearances.keys().collect::<HashSet<_>>())
        .cloned()
        .collect::<Vec<_>>();
    if descriptors.is_empty() {
        return;
    }
    debug!(
        "Loading ids: {:?}",
        descriptors
            .iter()
            .map(|descriptor| descriptor.id)
            .collect::<Vec<u32>>()
    );

    let descriptor_infos: HashMap<AppearanceDescriptor, SpriteInfo> = descriptors
        .iter()
        .filter_map(|&descriptor| {
            let Some(prepared_appearance) = content_assets
                .prepared_appearances()
                .get_for_group(descriptor.group, descriptor.id)
            else {
                warn!("Appearance {:?} not found", descriptor);
                return None;
            };

            let Some(frame_group) = prepared_appearance
                .frame_groups
                .get(descriptor.frame_group_index() as usize)
            else {
                warn!("Frame group for appearance {:?} not found", descriptor);
                return None;
            };
            frame_group
                .sprite_info
                .as_ref()
                .map(|sprite_info| (*descriptor, sprite_info.clone()))
        })
        .collect();

    let loaded_sprites: HashMap<u32, LoadedSprite> = descriptor_infos
        .iter()
        .map(|(descriptor, sprite_info)| (descriptor.group, sprite_info))
        .group_by(|(group, _)| *group)
        .into_iter()
        .map(|(group, group_iter)| {
            let sprite_ids: Vec<u32> = group_iter
                .flat_map(|(_, sprite_info)| sprite_info.sprite_ids.clone())
                .collect();
            (group, sprite_ids)
        })
        .flat_map(|(group, sprite_ids)| {
            load_sprites(
                group,
                sprite_ids,
                &content_assets,
                &layouts,
                &sprite_meshes,
                &mut materials,
                &asset_server,
            )
        })
        .map(|sprite| (sprite.sprite_id, sprite))
        .collect();

    descriptor_infos
        .iter()
        .for_each(|(descriptor, sprite_info)| {
            let sprites: Vec<LoadedSprite> = sprite_info
                .sprite_ids
                .iter()
                .filter_map(|sprite_id| loaded_sprites.get(sprite_id).cloned())
                .collect();

            let animation_tuple = sprite_info.animation.as_ref().map(|animation| {
                (
                    animation.get_animation_key(),
                    AnimationDescriptor {
                        sprites: sprites.clone(),
                        initial_index: 0,
                        skip: (sprite_info.layers()
                            * sprite_info.pattern_width()
                            * sprite_info.pattern_height()
                            * sprite_info.pattern_depth()) as usize,
                        synchronized: animation.synchronized(),
                    },
                )
            });

            let loaded_appearance = LoadedAppearance {
                sprites: sprites.clone(),
                layers: sprite_info.layers(),
                animation: animation_tuple,
            };

            loaded_appearances.insert(*descriptor, loaded_appearance);
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
