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

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum DrawingSystems {
    Apply,
    Persist,
}

/// Auxiliary function to get the top most visible entity and its DrawingBundle from a tile position.
pub fn get_top_most_visible(
    tile_pos: TilePosition,
    map_tiles: &ResMut<MapTiles<Entity>>,
    q_current_appearance: &Query<(&Visibility, &Layer, &AppearanceDescriptor), With<TileComponent>>,
) -> Option<(Entity, DrawingBundle)> {
    get_top_most_visible_for_tile(
        map_tiles.get(&tile_pos)?.clone().into_iter().rev(),
        tile_pos,
        q_current_appearance,
    )
}

pub fn get_top_most_visible_for_bundles(
    bundles: &[DrawingBundle],
    tiles: &mut ResMut<MapTiles<Entity>>,
    q_current_appearance: &Query<(&Visibility, &Layer, &AppearanceDescriptor), With<TileComponent>>,
) -> Vec<DrawingBundle> {
    bundles
        .iter()
        .filter_map(|bundle| get_top_most_visible(bundle.tile_pos, tiles, q_current_appearance))
        .map(|(_, bundle)| bundle)
        .collect::<Vec<_>>()
}

pub fn get_top_most_visible_bottom_layer(
    tile_pos: TilePosition,
    map_tiles: &ResMut<MapTiles<Entity>>,
    q_current_appearance: &Query<(&Visibility, &Layer, &AppearanceDescriptor), With<TileComponent>>,
) -> Option<(Entity, DrawingBundle)> {
    get_top_most_visible_for_tile(
        map_tiles
            .get(&tile_pos)?
            .clone()
            .into_iter()
            .filter(|(layer, _)| matches!(layer, Layer::Bottom(_)))
            .rev(),
        tile_pos,
        q_current_appearance,
    )
}

pub fn get_top_most_visible_for_tile(
    iter: impl Iterator<Item = (Layer, Entity)>,
    tile_pos: TilePosition,
    q_current_appearance: &Query<(&Visibility, &Layer, &AppearanceDescriptor), With<TileComponent>>,
) -> Option<(Entity, DrawingBundle)> {
    for (layer, entity) in iter {
        if let Ok((visibility, _, appearance)) = q_current_appearance.get(entity) {
            if visibility == Visibility::Hidden {
                continue;
            }

            return Some((entity, DrawingBundle::new(layer, tile_pos, *appearance)));
        }
    }

    None
}

type ElevationFilter = (
    With<TileComponent>,
    Or<(Changed<Visibility>, Changed<TilePosition>)>,
);

pub fn apply_elevation<C: ContentAssets>(
    content_assets: Res<C>,
    q_tile: Query<(&TilePosition, &Layer), ElevationFilter>,
    mut q_entities: Query<
        (&mut Elevation, &Visibility, &AppearanceDescriptor),
        With<TileComponent>,
    >,
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
                let Ok((mut elevation, visibility, appearance_desc)) = q_entities.get_mut(entity)
                else {
                    return tile_elevation;
                };

                let elevation_delta = appearances
                    .get_for_group(appearance_desc.group, appearance_desc.id)
                    .cloned()
                    .and_then(|app| app.flags?.height?.elevation)
                    .unwrap_or(0) as f32;
                elevation.elevation = tile_elevation;

                if visibility != Visibility::Hidden {
                    tile_elevation + elevation_delta
                } else {
                    tile_elevation
                }
            });
    }
}
