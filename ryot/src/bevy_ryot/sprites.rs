//! Sprite loading and drawing.
use crate::appearances::{SpriteInfo, SpriteSheetData, SpriteSheetDataSet};
use crate::{get_decompressed_file_name, SPRITE_SHEET_FOLDER};
use crate::{prelude::*, Directional};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
#[cfg(feature = "debug")]
use bevy::sprite::Anchor;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::{HashMap, HashSet};
#[cfg(feature = "debug")]
use bevy_stroked_text::{StrokedText, StrokedTextBundle};
use itertools::Itertools;

use std::path::PathBuf;

pub const SPRITE_BASE_SIZE: UVec2 = UVec2::new(32, 32);

use self::drawing::Elevation;
use self::sprite_animations::{
    AnimationDescriptor, AnimationKey, AnimationSprite, SpriteAnimationExt,
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
                    ..default()
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

/// A system that ensures that all entities with an AppearanceDescriptor have a SpriteMaterial mesh bundle.
pub(crate) fn ensure_appearance_initialized(
    mut commands: Commands,
    query: Query<Entity, (With<AppearanceDescriptor>, Without<Handle<SpriteMaterial>>)>,
    #[cfg(feature = "debug")] q_debug: Query<
        (Entity, &Layer),
        (With<AppearanceDescriptor>, Without<Handle<SpriteMaterial>>),
    >,
) {
    query.iter().for_each(|entity| {
        commands.entity(entity).insert((
            MaterialMesh2dBundle::<SpriteMaterial>::default(),
            SpriteLayout::default(),
            Elevation::default(),
        ));
    });

    #[cfg(feature = "debug")]
    q_debug.iter().for_each(|(entity, layer)| {
        commands.entity(entity).with_children(|builder| {
            builder.spawn((
                StrokedTextBundle::new(StrokedText {
                    font_size: 16.,
                    text_anchor: Anchor::BottomRight,
                    ..default()
                })
                .with_transform(
                    Transform::from_translation(Vec3::new(8., debug_y_offset(layer), 1.))
                        .with_scale(Vec3::splat(0.18)),
                ),
                PositionDebugText,
                Layer::Hud(0),
            ));
            builder.spawn((
                StrokedTextBundle::new(StrokedText {
                    text: format!("{}", layer),
                    font_size: 16.,
                    text_anchor: Anchor::BottomLeft,
                    color: Color::from(layer),
                    ..default()
                })
                .with_transform(
                    Transform::from_translation(Vec3::new(8.5, debug_y_offset(layer), 1.))
                        .with_scale(Vec3::splat(0.18)),
                ),
                Layer::Hud(0),
            ));
        });
    });
}

pub(crate) type ChangingAppearanceFilter = Or<(
    Changed<AppearanceDescriptor>,
    Changed<Directional>,
    (Without<AnimationSprite>, Changed<SpriteParams>),
)>;

pub(crate) fn update_sprite_system(
    mut q_updated: Query<
        (
            &AppearanceDescriptor,
            Option<&Directional>,
            Option<&SpriteParams>,
            &mut Elevation,
            &mut SpriteLayout,
            &mut Mesh2dHandle,
            &mut Handle<SpriteMaterial>,
        ),
        ChangingAppearanceFilter,
    >,
    mut materials: ResMut<Assets<SpriteMaterial>>,
    loaded_appereances: Res<LoadedAppearances>,
) {
    q_updated.iter_mut().for_each(
        |(
            descriptor,
            direction,
            sprite_params,
            mut elevation,
            mut layout,
            mut mesh,
            mut material,
        )| {
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
            elevation.base_height = sprite.sprite_sheet.layout.get_height(&SPRITE_BASE_SIZE);
            *layout = sprite.sprite_sheet.layout;
            *mesh = Mesh2dHandle(sprite.mesh.clone());
            *material = sprite_material_from_params(sprite_params, &mut materials, sprite);
        },
    );
}

pub(crate) fn sprite_material_from_params(
    sprite_params: Option<&SpriteParams>,
    materials: &mut ResMut<'_, Assets<SpriteMaterial>>,
    sprite: &LoadedSprite,
) -> Handle<SpriteMaterial> {
    sprite_params
        .map(|params| {
            if params.has_any() {
                materials
                    .get(sprite.material.id())
                    .map(|base| params.to_material(base.clone()))
                    .map(|material| materials.add(material))
                    .unwrap_or_else(|| sprite.material.clone())
            } else {
                sprite.material.clone()
            }
        })
        .unwrap_or_else(|| sprite.material.clone())
}

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

#[derive(Event)]
pub struct LoadAppearanceEvent(pub AppearanceDescriptor);

pub(crate) fn process_load_events_system<C: ContentAssets>(
    content_assets: Res<C>,
    loaded_appearances: Res<LoadedAppearances>,
    mut events: EventReader<LoadAppearanceEvent>,
) -> Option<HashMap<AppearanceDescriptor, SpriteInfo>> {
    let descriptors = events
        .read()
        .filter_map(|LoadAppearanceEvent(descriptor)| {
            if descriptor.id != 0 {
                Some(descriptor)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .difference(&loaded_appearances.keys().collect::<HashSet<_>>())
        .cloned()
        .collect::<Vec<_>>();
    if descriptors.is_empty() {
        return None;
    }
    debug!(
        "Loading ids: {:?}",
        descriptors
            .iter()
            .map(|descriptor| descriptor.id)
            .collect::<Vec<u32>>()
    );

    Some(
        descriptors
            .iter()
            .filter_map(|&descriptor| {
                content_assets
                    .prepared_appearances()
                    .get_for_group(descriptor.group, descriptor.id)?
                    .frame_groups
                    .get(descriptor.frame_group_index() as usize)?
                    .sprite_info
                    .as_ref()
                    .map(|sprite_info| (*descriptor, sprite_info.clone()))
            })
            .collect(),
    )
}

pub(crate) fn load_sprite_system<C: ContentAssets>(
    In(descriptor_sprite_infos): In<Option<HashMap<AppearanceDescriptor, SpriteInfo>>>,
    content_assets: Res<C>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    sprite_meshes: Res<SpriteMeshes>,
    mut materials: ResMut<Assets<SpriteMaterial>>,
    asset_server: Res<AssetServer>,
) -> Option<(
    HashMap<AppearanceDescriptor, SpriteInfo>,
    HashMap<u32, LoadedSprite>,
)> {
    descriptor_sprite_infos.map(|descriptor_sprite_infos| {
        (
            descriptor_sprite_infos.clone(),
            descriptor_sprite_infos
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
                .collect(),
        )
    })
}

pub(crate) fn store_loaded_appearances_system(
    In(input): In<
        Option<(
            HashMap<AppearanceDescriptor, SpriteInfo>,
            HashMap<u32, LoadedSprite>,
        )>,
    >,
    mut loaded_appearances: ResMut<LoadedAppearances>,
) {
    let Some((descriptor_sprite_infos, loaded_sprites)) = input else {
        return;
    };

    descriptor_sprite_infos
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

#[derive(AsBindGroup, TypePath, Asset, Debug, Clone, Default, PartialEq)]
pub struct SpriteMaterial {
    #[uniform(0)]
    pub index: u32,
    #[uniform(0)]
    pub counts: Vec2,
    #[uniform(0)]
    pub outline_thickness: f32,
    #[uniform(0)]
    pub outline_color: Color,
    #[uniform(0)]
    pub tint: Color,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for SpriteMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://ryot/bevy_ryot/shaders/sprite.wgsl".into()
    }
}
