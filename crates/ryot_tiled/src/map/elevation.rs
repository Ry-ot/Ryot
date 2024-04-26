use bevy_ecs::change_detection::Res;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::*;
use bevy_render::prelude::Visibility;
use glam::{UVec2, Vec2, Vec3};
use itertools::Itertools;
use ryot_core::prelude::{ContentId, ContentType, Elevation, SpriteLayout, VisualElements};

use crate::prelude::{Layer, MapTiles, TilePosition};
use crate::tile_size;

const SPRITE_BASE_SIZE: UVec2 = UVec2::new(32, 32);

pub fn initialize_elevation(
    mut commands: Commands,
    query: Query<Entity, (With<ContentId>, Without<Elevation>)>,
) {
    query.iter().for_each(|entity| {
        commands.entity(entity).insert(Elevation::default());
    });
}

pub fn elevate_position(
    position: &TilePosition,
    layout: SpriteLayout,
    layer: Layer,
    elevation: Elevation,
) -> Vec3 {
    let elevation = elevation.clamp(0.0, 1.0);
    let anchor = Vec2::new(elevation, -elevation);
    position.to_vec3(&layer)
        - (SpriteLayout::OneByOne.get_size(&tile_size()).as_vec2() * anchor).extend(0.)
        - (layout.get_size(&tile_size()).as_vec2() * Vec2::new(0.5, -0.5)).extend(0.)
}

pub fn apply_elevation(
    visual_elements: Res<VisualElements>,
    q_tile: Query<(&TilePosition, &Layer), ElevationFilter>,
    mut q_entities: Query<(&mut Elevation, &ContentId, Option<&Visibility>)>,
    map_tiles: Res<MapTiles<Entity>>,
) {
    for tile in q_tile
        .iter()
        .filter(|(_, layer)| matches!(layer, Layer::Bottom(_)))
        .map(|(pos, _)| *pos)
        .unique()
        .filter_map(|pos| map_tiles.get(&pos))
    {
        tile.into_iter()
            .filter(|(layer, _)| matches!(layer, Layer::Bottom(_)))
            .map(|(_, entity)| entity)
            .fold(0., |tile_elevation, entity| {
                let Ok((mut elevation, object_id, visibility)) = q_entities.get_mut(entity) else {
                    return tile_elevation;
                };
                let Some((group, id)) = object_id.as_group_and_id() else {
                    return tile_elevation;
                };

                let elevation_delta =
                    if visibility.cloned().unwrap_or_default() != Visibility::Hidden {
                        visual_elements
                            .get_for_group_and_id(group, id)
                            .cloned()
                            .map(|app| *app.properties.elevation)
                            .unwrap_or(0.)
                            / SPRITE_BASE_SIZE.y as f32
                    } else {
                        0.
                    };

                **elevation = match group {
                    ContentType::Object => tile_elevation,
                    ContentType::Outfit => tile_elevation,
                    ContentType::Effect => 0.,
                    ContentType::Missile => 0.,
                };

                tile_elevation + elevation_delta
            });
    }
}

type ElevationFilter = (
    With<ContentId>,
    Or<(
        Changed<ContentId>,
        Changed<Visibility>,
        Changed<TilePosition>,
    )>,
);
