//! This module contains all the systems for drawing the game.
//! The systems are used to draw the map and the entities that are on it.
//! The systems manipulate basic drawing entities that are added by the drawing commands.
//! Those entities are the trigger for the drawing systems within the ECS.
use crate::prelude::drawing::TileComponent;
use crate::prelude::{drawing::*, map::*, *};
use bevy::prelude::*;
// use bevy::sprite::Anchor;
use bevy::utils::HashMap;

mod deletion;
pub use deletion::*;

mod update;
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

type ElevationQuery<'w, 's, 'a> = Query<
    'w,
    's,
    (
        &'a mut Elevation,
        &'a Layer,
        &'a TilePosition,
        &'a Visibility,
        &'a AppearanceDescriptor,
    ),
    (
        With<TileComponent>,
        Or<(
            Changed<Visibility>,
            Added<Visibility>,
            Changed<TilePosition>,
        )>,
    ),
>;

pub fn apply_elevation<C: ContentAssets>(
    content_assets: Res<C>,
    mut q_tile: ElevationQuery,
    mut elevation_per_pos: Local<HashMap<TilePosition, u32>>,
) {
    let appearances = content_assets.prepared_appearances();

    for (mut elevation, layer, tile_pos, visibility, appearance_desc) in q_tile.iter_mut() {
        let Layer::Bottom(_) = layer else {
            continue;
        };

        let elevation_delta = (|| -> Option<u32> {
            appearances
                .get_for_group(appearance_desc.group, appearance_desc.id)?
                .clone()
                .flags?
                .height?
                .elevation
        })()
        .unwrap_or(0);

        let tile_elevation = elevation_per_pos.entry(*tile_pos).or_default();
        elevation.elevation = *tile_elevation as f32;

        if visibility == Visibility::Hidden {
            *tile_elevation -= elevation_delta;
        } else {
            *tile_elevation += elevation_delta;
        }
    }
}
