//! This module contains all the systems for drawing the game.
//! The systems are used to draw the map and the entities that are on it.
//! The systems manipulate basic drawing entities that are added by the drawing commands.
//! Those entities are the trigger for the drawing systems within the ECS.
use crate::prelude::drawing::TileComponent;
use crate::prelude::{drawing::*, map::*, *};
use bevy::prelude::*;

mod deletion;
pub use deletion::*;

mod update;
use itertools::Itertools;
pub use update::*;

use self::sprites::SPRITE_BASE_SIZE;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum DrawingSystems {
    Apply,
    Persist,
}

/// Auxiliary function to get the top most visible entity and its DrawingBundle from a tile position.
pub fn get_top_most_visible(
    tile_pos: TilePosition,
    map_tiles: &ResMut<MapTiles<Entity>>,
    q_current: &Query<
        (&Visibility, &GameObjectId, Option<&FrameGroupComponent>),
        (With<TileComponent>, With<Layer>),
    >,
) -> Option<(Entity, DrawingBundle)> {
    get_top_most_visible_for_tile(
        map_tiles.get(&tile_pos)?.clone().into_iter().rev(),
        tile_pos,
        q_current,
    )
}

pub fn get_top_most_visible_for_bundles(
    bundles: &[DrawingBundle],
    tiles: &mut ResMut<MapTiles<Entity>>,
    q_current: &Query<
        (&Visibility, &GameObjectId, Option<&FrameGroupComponent>),
        (With<TileComponent>, With<Layer>),
    >,
) -> Vec<DrawingBundle> {
    bundles
        .iter()
        .filter_map(|bundle| get_top_most_visible(bundle.tile_pos, tiles, q_current))
        .map(|(_, bundle)| bundle)
        .collect::<Vec<_>>()
}

pub fn get_top_most_visible_bottom_layer(
    tile_pos: TilePosition,
    map_tiles: &ResMut<MapTiles<Entity>>,
    q_current: &Query<
        (&Visibility, &GameObjectId, Option<&FrameGroupComponent>),
        (With<TileComponent>, With<Layer>),
    >,
) -> Option<(Entity, DrawingBundle)> {
    get_top_most_visible_for_tile(
        map_tiles
            .get(&tile_pos)?
            .clone()
            .into_iter()
            .filter(|(layer, _)| matches!(layer, Layer::Bottom(_)))
            .rev(),
        tile_pos,
        q_current,
    )
}

pub fn get_top_most_visible_for_tile(
    iter: impl Iterator<Item = (Layer, Entity)>,
    tile_pos: TilePosition,
    q_current_appearance: &Query<
        (&Visibility, &GameObjectId, Option<&FrameGroupComponent>),
        (With<TileComponent>, With<Layer>),
    >,
) -> Option<(Entity, DrawingBundle)> {
    for (layer, entity) in iter {
        if let Ok((visibility, object_id, frame_group)) = q_current_appearance.get(entity) {
            if visibility == Visibility::Hidden {
                continue;
            }

            return Some((
                entity,
                DrawingBundle::new(
                    layer,
                    tile_pos,
                    *object_id,
                    frame_group.cloned().unwrap_or_default(),
                ),
            ));
        }
    }

    None
}

type ElevationFilter = (
    With<GameObjectId>,
    Or<(Changed<Visibility>, Changed<TilePosition>)>,
);

pub fn apply_elevation<C: ContentAssets>(
    content_assets: Res<C>,
    q_tile: Query<(&TilePosition, &Layer), ElevationFilter>,
    mut q_entities: Query<(&mut Elevation, &Visibility, &GameObjectId)>,
    map_tiles: Res<MapTiles<Entity>>,
) {
    let appearances = content_assets.prepared_appearances();
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
                let Ok((mut elevation, visibility, object_id)) = q_entities.get_mut(entity) else {
                    return tile_elevation;
                };
                let Some((group, id)) = object_id.as_group_and_id() else {
                    return tile_elevation;
                };

                let appearance = appearances.get_for_group(group, id).cloned();
                let elevation_delta = appearance
                    .and_then(|app| app.flags?.height?.elevation)
                    .unwrap_or(0) as f32
                    / SPRITE_BASE_SIZE.y as f32;
                elevation.elevation = match group {
                    AppearanceGroup::Object => tile_elevation,
                    AppearanceGroup::Outfit => tile_elevation,
                    AppearanceGroup::Effect => 0.,
                    AppearanceGroup::Missile => 0.,
                };

                if visibility != Visibility::Hidden {
                    tile_elevation + elevation_delta
                } else {
                    tile_elevation
                }
            });
    }
}
