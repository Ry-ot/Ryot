use crate::animation::descriptor::AnimationDescriptor;
use crate::animation::key::SpriteAnimationExt;
use crate::material::meshes::SpriteMeshes;
use crate::material::SpriteMaterial;
use crate::prelude::*;
use bevy_asset::{AssetServer, Assets, Handle};
use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::prelude::{Changed, In, Or, Query};
use bevy_render::prelude::Image;
use bevy_utils::tracing::warn;
use bevy_utils::{default, HashMap, HashSet};
use itertools::Itertools;
use ryot_core::prelude::*;
use ryot_tiled::tile_size;
use std::path::PathBuf;

fn load_sprites(
    group: ContentType,
    sprite_ids: Vec<u32>,
    sprite_sheets: &Res<SpriteSheets>,
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

pub fn load_from_entities_system(
    query: Query<(&ContentId, Option<&FrameGroup>), Or<(Changed<ContentId>, Changed<FrameGroup>)>>,
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

pub fn process_load_events_system(
    visual_elements: Res<VisualElements>,
    loaded_appearances: Res<LoadedAppearances>,
    mut events: EventReader<LoadAppearanceEvent>,
) -> Option<HashMap<(ContentId, FrameGroup), SpriteInfo>> {
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

pub fn load_sprite_system(
    In(inputs): In<Option<HashMap<(ContentId, FrameGroup), SpriteInfo>>>,
    sprite_sheets: Res<SpriteSheets>,
    layouts: Res<TextureAtlasLayouts>,
    sprite_meshes: Res<SpriteMeshes>,
    mut materials: ResMut<Assets<SpriteMaterial>>,
    asset_server: Res<AssetServer>,
) -> Option<(
    HashMap<(ContentId, FrameGroup), SpriteInfo>,
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

pub fn store_loaded_appearances_system(
    In(input): In<
        Option<(
            HashMap<(ContentId, FrameGroup), SpriteInfo>,
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

fn load_sprite_textures(
    sprite_ids: Vec<u32>,
    asset_server: &Res<AssetServer>,
    sprite_sheets: &SpriteSheets,
) -> Vec<(u32, Handle<Image>)> {
    sprite_ids
        .iter()
        .filter_map(|sprite_id| {
            load_sprite_texture(*sprite_id, asset_server, sprite_sheets)
                .map(|texture| (*sprite_id, texture))
        })
        .collect()
}

fn load_sprite_texture(
    sprite_id: u32,
    asset_server: &Res<AssetServer>,
    sprite_sheets: &SpriteSheets,
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
