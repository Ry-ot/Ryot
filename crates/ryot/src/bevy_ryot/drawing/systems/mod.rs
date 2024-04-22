//! This module contains all the systems for drawing the game.
//! The systems are used to draw the map and the entities that are on it.
//! The systems manipulate basic drawing entities that are added by the drawing commands.
//! Those entities are the trigger for the drawing systems within the ECS.
use crate::prelude::drawing::TileComponent;
use crate::prelude::{drawing::*, *};
use bevy::prelude::*;
use ryot_tiled::prelude::*;

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
    q_current: &Query<
        (&Visibility, &GameObjectId, Option<&FrameGroup>),
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
        (&Visibility, &GameObjectId, Option<&FrameGroup>),
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
        (&Visibility, &GameObjectId, Option<&FrameGroup>),
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
        (&Visibility, &GameObjectId, Option<&FrameGroup>),
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
