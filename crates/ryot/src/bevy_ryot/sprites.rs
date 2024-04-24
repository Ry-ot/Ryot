//! Sprite loading and drawing.
use crate::prelude::*;
use ryot_sprites::prelude::*;

use self::elevation::Elevation;
use self::sprite_animations::{
    AnimationDescriptor, AnimationKey, AnimationSprite, SpriteAnimationExt,
};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::{HashMap, HashSet};
use itertools::Itertools;
use ryot_tiled::prelude::*;
use std::path::PathBuf;

#[cfg(feature = "debug")]
use bevy::sprite::Anchor;
#[cfg(feature = "debug")]
use bevy_stroked_text::{StrokedText, StrokedTextBundle};

pub const SPRITE_BASE_SIZE: UVec2 = UVec2::new(32, 32);

pub struct LoadedAppearance {
    pub sprites: Vec<LoadedSprite>,
    pub layers: u32,
    pub animation: Option<(AnimationKey, AnimationDescriptor)>,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct LoadedAppearances(pub HashMap<(GameObjectId, FrameGroup), LoadedAppearance>);

/// A struct that holds the information needed to draw a sprite.
/// It's a wrapper around a sprite sheet and a sprite id, that also holds the
/// handle to the texture atlas.
#[derive(Debug, Clone)]
pub struct LoadedSprite {
    pub sprite_id: u32,
    pub group: EntityType,
    pub sprite_sheet: SpriteSheetData,
    pub texture: Handle<Image>,
    pub material: Handle<SpriteMaterial>,
    pub mesh: Handle<Mesh>,
}

impl LoadedSprite {
    pub fn new(
        group: EntityType,
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

pub fn load_sprites(
    group: EntityType,
    sprite_ids: Vec<u32>,
    sprite_sheets: &Res<SpriteSheetDataSet>,
    layouts: &Res<TextureAtlasLayouts>,
    sprite_meshes: &Res<SpriteMeshes>,
    materials: &mut ResMut<Assets<SpriteMaterial>>,
    asset_server: &Res<AssetServer>,
) -> Vec<LoadedSprite> {
    if sprite_sheets.is_empty() {
        warn!("No sprite sheets loaded");
        return vec![];
    };

    load_sprite_textures(sprite_ids, asset_server, sprite_sheets)
        .iter()
        .filter_map(|(sprite_id, texture)| {
            let Some(sprite_sheet) = sprite_sheets.get_by_sprite_id(*sprite_id) else {
                warn!("Sprite {} not found in sprite sheets", sprite_id);
                return None;
            };

            let layout = layouts
                .get(sprite_sheet.layout as usize)
                .unwrap_or_else(|| panic!("Layout not found: {:?}", sprite_sheet.layout));
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
                    alpha: 1.,
                    ..default()
                }),
            })
        })
        .collect()
}

pub(crate) fn load_sprite_textures(
    sprite_ids: Vec<u32>,
    asset_server: &Res<AssetServer>,
    sprite_sheets: &SpriteSheetDataSet,
) -> Vec<(u32, Handle<Image>)> {
    sprite_ids
        .iter()
        .filter_map(|sprite_id| {
            load_sprite_texture(*sprite_id, asset_server, sprite_sheets)
                .map(|texture| (*sprite_id, texture))
        })
        .collect()
}

pub(crate) fn load_sprite_texture(
    sprite_id: u32,
    asset_server: &Res<AssetServer>,
    sprite_sheets: &SpriteSheetDataSet,
) -> Option<Handle<Image>> {
    let Some(sprite_sheet) = sprite_sheets.get_by_sprite_id(sprite_id) else {
        warn!("Sprite {} not found in sprite sheets", sprite_id);
        return None;
    };

    let texture: Handle<Image> = asset_server.load(
        PathBuf::from(SPRITE_SHEET_FOLDER).join(get_decompressed_file_name(&sprite_sheet.file)),
    );

    Some(texture)
}

/// A system that ensures that all entities with an GameObjectId have a SpriteMaterial mesh bundle.
pub(crate) fn initialize_sprite_material(
    mut commands: Commands,
    query: Query<(Entity, Has<Elevation>), (With<GameObjectId>, Without<Handle<SpriteMaterial>>)>,
    #[cfg(feature = "debug")] q_debug: Query<
        (Entity, &Layer),
        (With<GameObjectId>, Without<Handle<SpriteMaterial>>),
    >,
) {
    query.iter().for_each(|(entity, has_elevation)| {
        commands.entity(entity).insert((
            MaterialMesh2dBundle::<SpriteMaterial>::default(),
            SpriteLayout::default(),
        ));

        if !has_elevation {
            commands.entity(entity).insert(Elevation::default());
        }
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
    Changed<GameObjectId>,
    Changed<FrameGroup>,
    Changed<Directional>,
    (Without<AnimationSprite>, Changed<SpriteParams>),
)>;

pub(crate) fn update_sprite_system(
    mut q_updated: Query<
        (
            &GameObjectId,
            Option<&FrameGroup>,
            Option<&Directional>,
            Option<&SpriteParams>,
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
        |(object_id, frame_group, direction, sprite_params, mut layout, mut mesh, mut material)| {
            if object_id.is_none() {
                return;
            }
            let Some(loaded_appearance) =
                loaded_appereances.get(&(*object_id, frame_group.cloned().unwrap_or_default()))
            else {
                warn!("BUG: Loaded appearance for {:?} not found.", object_id);
                return;
            };

            let direction_index = match direction {
                Some(dir) => dir.index(),
                None => 0,
            } * loaded_appearance.layers as usize;

            let Some(sprite) = loaded_appearance.sprites.get(direction_index) else {
                warn!(
                    "Sprite for appearance {:?} not found for direction {:?}",
                    object_id, direction
                );
                return;
            };
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
    query: Query<
        (&GameObjectId, Option<&FrameGroup>),
        Or<(Changed<GameObjectId>, Changed<FrameGroup>)>,
    >,
    loaded_appearances: Res<LoadedAppearances>,
    mut events: EventWriter<LoadAppearanceEvent>,
) {
    query
        .iter()
        .unique()
        .filter(|(&object_id, frame_group)| {
            !loaded_appearances.contains_key(&(object_id, frame_group.cloned().unwrap_or_default()))
        })
        .for_each(|(&object_id, frame_group)| {
            events.send(LoadAppearanceEvent {
                object_id,
                frame_group: frame_group.cloned().unwrap_or_default(),
            });
        });
}

#[derive(Event)]
pub struct LoadAppearanceEvent {
    pub object_id: GameObjectId,
    pub frame_group: FrameGroup,
}

pub(crate) fn process_load_events_system(
    visual_elements: Res<VisualElements>,
    loaded_appearances: Res<LoadedAppearances>,
    mut events: EventReader<LoadAppearanceEvent>,
) -> Option<HashMap<(GameObjectId, FrameGroup), SpriteInfo>> {
    let keys = loaded_appearances.keys().cloned().collect::<HashSet<_>>();
    let ids_and_frame_groups = events
        .read()
        .filter_map(
            |LoadAppearanceEvent {
                 object_id,
                 frame_group,
             }| {
                if object_id.is_none() {
                    None
                } else {
                    Some((*object_id, *frame_group))
                }
            },
        )
        .collect::<HashSet<_>>()
        .difference(&keys)
        .cloned()
        .collect::<Vec<_>>();
    if ids_and_frame_groups.is_empty() {
        return None;
    }

    Some(
        ids_and_frame_groups
            .iter()
            .filter_map(|(object_id, frame_group)| {
                let (group, id) = object_id.as_group_and_id()?;
                let sprite_info = visual_elements
                    .get_for_group_and_id(group, id)?
                    .sprites_info
                    .get(*frame_group as usize)?;

                Some(((*object_id, *frame_group), sprite_info.clone()))
            })
            .collect(),
    )
}

pub(crate) fn load_sprite_system(
    In(inputs): In<Option<HashMap<(GameObjectId, FrameGroup), SpriteInfo>>>,
    sprite_sheets: Res<SpriteSheetDataSet>,
    layouts: Res<TextureAtlasLayouts>,
    sprite_meshes: Res<SpriteMeshes>,
    mut materials: ResMut<Assets<SpriteMaterial>>,
    asset_server: Res<AssetServer>,
) -> Option<(
    HashMap<(GameObjectId, FrameGroup), SpriteInfo>,
    HashMap<u32, LoadedSprite>,
)> {
    inputs.map(|object_id_sprite_info| {
        (
            object_id_sprite_info.clone(),
            object_id_sprite_info
                .iter()
                .filter_map(|((object_id, _frame_group), sprite_info)| {
                    object_id.group().map(|group| (group, sprite_info))
                })
                .group_by(|(group, _)| *group)
                .into_iter()
                .map(|(group, group_iter)| {
                    let sprite_ids: Vec<u32> = group_iter
                        .flat_map(|(_, sprite_info)| sprite_info.ids.clone())
                        .collect();
                    (group, sprite_ids)
                })
                .flat_map(|(group, sprite_ids)| {
                    load_sprites(
                        group,
                        sprite_ids,
                        &sprite_sheets,
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
            HashMap<(GameObjectId, FrameGroup), SpriteInfo>,
            HashMap<u32, LoadedSprite>,
        )>,
    >,
    mut loaded_appearances: ResMut<LoadedAppearances>,
) {
    let Some((object_id_sprite_infos, loaded_sprites)) = input else {
        return;
    };

    object_id_sprite_infos
        .iter()
        .for_each(|((object_id, frame_group), sprite_info)| {
            let sprites: Vec<LoadedSprite> = sprite_info
                .ids
                .iter()
                .filter_map(|sprite_id| loaded_sprites.get(sprite_id).cloned())
                .collect();

            let animation_tuple = sprite_info.animation.as_ref().map(|animation| {
                (
                    animation.get_animation_key(),
                    AnimationDescriptor {
                        sprites: sprites.clone(),
                        layers: sprite_info.layers as usize,
                        skip: (sprite_info.layers
                            * sprite_info.pattern_width
                            * sprite_info.pattern_height
                            * sprite_info.pattern_depth) as usize,
                        synchronized: animation.synchronized,
                    },
                )
            });

            let loaded_appearance = LoadedAppearance {
                sprites: sprites.clone(),
                layers: sprite_info.layers,
                animation: animation_tuple,
            };

            loaded_appearances.insert((*object_id, *frame_group), loaded_appearance);
        });
}
