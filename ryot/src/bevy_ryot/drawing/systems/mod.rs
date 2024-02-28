//! This module contains all the systems for drawing the game.
//! The systems are used to draw the map and the entities that are on it.
//! The systems manipulate basic drawing entities that are added by the drawing commands.
//! Those entities are the trigger for the drawing systems within the ECS.
use crate::bevy_ryot::drawing::{DrawingBundle, TileComponent};
use crate::bevy_ryot::map::MapTiles;
use crate::bevy_ryot::AppearanceDescriptor;
use crate::position::TilePosition;
use crate::Layer;
use bevy::prelude::{Entity, Query, ResMut, SystemSet, Visibility, With};

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
    map_tiles: &ResMut<MapTiles>,
    q_current_appearance: &Query<(&Visibility, &Layer, &AppearanceDescriptor), With<TileComponent>>,
) -> Option<(Entity, DrawingBundle)> {
    let tile_content = map_tiles.get(&tile_pos)?.clone();

    for (layer, entity) in tile_content.clone().into_iter().rev() {
        if let Ok((visibility, _, appearance)) = q_current_appearance.get(entity) {
            if visibility == Visibility::Hidden {
                continue;
            }

            return Some((entity, DrawingBundle::new(layer, tile_pos, *appearance)));
        }
    }

    None
}

pub fn get_top_most_visible_for_bundles(
    bundles: &[DrawingBundle],
    tiles: &mut ResMut<MapTiles>,
    q_current_appearance: &Query<(&Visibility, &Layer, &AppearanceDescriptor), With<TileComponent>>,
) -> Vec<DrawingBundle> {
    bundles
        .iter()
        .filter_map(|bundle| get_top_most_visible(bundle.tile_pos, tiles, q_current_appearance))
        .map(|(_, bundle)| bundle)
        .collect::<Vec<_>>()
}
