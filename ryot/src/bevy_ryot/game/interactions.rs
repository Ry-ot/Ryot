//! This module provides a handy way of dealing with tile level interactions like movement and
//! sight restrictions, and more. Any component flag like BlockWalk, BlockSight or others
//! custom defined behaviors can be used to filter positions or entities.
//!
//! The pieces provided in this module are designed to be used like building blocks in a bevy
//! pipeline, and can be combined in any way to achieve the desired behavior.
//!
//! You can start interaction filtering by using the `with_flag` or `without_flag` functions
//! after a method that provides the positions that you want to interact with. For example:
//!
//! ```
//! use bevy::prelude::*;
//! use ryot::prelude::*;
//! use ryot::prelude::interactions::*;
//! #[derive(Debug, Clone, Component, Copy)]
//! pub struct BlockSight;
//!
//! fn get_all_visible_positions(q_camera_sector: Query<&Sector, With<Camera>>) -> Vec<TilePosition> {
//!     let mut positions = Vec::new();
//!
//!     for sector in q_camera_sector.iter() {
//!         for x in sector.min.x..=sector.max.x {
//!             for y in sector.min.y..=sector.max.y {
//!                 positions.push(TilePosition::new(x, y, 0));
//!             }
//!         }
//!     }
//!
//!     info!("Positions count: {}", positions.len());
//!     positions
//! }
//!
//! App::new().add_systems(
//!     Update,
//!     get_all_visible_positions
//!         .pipe(without_flag)
//!         .pipe(partition_positions_by_flag::<(&BlockWalk, &BlockSight), TilePosition>)
//!         .pipe(get_elements_meeting_condition)
//!         .pipe(print_walkable_count)
//! );
//!
//! fn print_walkable_count(In(walkable): In<Vec<TilePosition>>) {
//!     info!("Walkable count: {}", walkable.len());
//! }
//! ```
//!
//! In the example above, we get all visible positions from the camera sector, then we filter out
//! the positions that do not contain entities with the `BlockWalk` component and a new custom
//! `BlockSight` component. Finally, we print the count of walkable positions.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::appearances::is_true;
use crate::bevy_ryot::map::MapTiles;
use crate::bevy_ryot::{AppearanceAssets, GameObjectId};
use crate::prelude::TilePosition;

/// A flag component that represent entities that are not walkable, meaning that they block the
/// walking in the tile/positions that they are contained.
#[derive(Debug, Clone, Component, Copy, PartialEq, Serialize, Deserialize)]
pub struct BlockWalk;

/// Represents the filtering intention in the interaction pipeline. You can either filter for
/// all positions that contain entities with a specific flag (WithFlag), or filter for all
/// positions that do not contain entities with a specific flag (WithoutFlag).
#[derive(Debug, Copy, Clone)]
pub enum FilterMode {
    WithFlag,
    WithoutFlag,
}

/// Helper system that initializes the interaction filtering pipeline from a list of positions
/// with an intention of filtering for positions that contain entities with a specific flag.
pub fn with_flag<T: Copy + Into<TilePosition>>(In(positions): In<Vec<T>>) -> (FilterMode, Vec<T>) {
    (FilterMode::WithFlag, positions)
}

/// Helper system that initializes the interaction filtering pipeline from a list of positions
/// with an intention of filtering for positions that don't contain entities with a specific flag.
pub fn without_flag<T: Copy + Into<TilePosition>>(
    In(positions): In<Vec<T>>,
) -> (FilterMode, Vec<T>) {
    (FilterMode::WithoutFlag, positions)
}

/// Main system that partitions the positions based on the flag component provided in the query.
/// It receives an intention and an array of positions, and returns two arrays: one with the
/// positions that contain entities with the flag, and another with the positions that don't.
/// It only considers entities that are visible.
pub fn partition_positions_by_flag<F: QueryData, T: Copy + Into<TilePosition>>(
    In((mode, positions)): In<(FilterMode, Vec<T>)>,
    map_tiles: Res<MapTiles<Entity>>,
    q_flag: Query<F>,
    q_visibility: Query<&Visibility>,
) -> (Vec<T>, Vec<T>) {
    positions.iter().partition(|&has_pos| {
        let pos: TilePosition = (*has_pos).into();

        let Some(tile) = map_tiles.get(&pos) else {
            return false;
        };

        let contains_flag = tile.clone().into_iter().any(|(_, entity)| {
            q_flag.contains(entity) && !matches!(q_visibility.get(entity), Ok(Visibility::Hidden))
        });

        match mode {
            FilterMode::WithFlag if contains_flag => true,
            FilterMode::WithoutFlag if !contains_flag => true,
            _ => false,
        }
    })
}

/// Retrieves the subset of positions that meet the specified condition based on the flag component.
pub fn get_elements_meeting_condition<T: Copy + Into<TilePosition>>(
    In((meeting_condition_positions, _)): In<(Vec<T>, Vec<T>)>,
) -> Vec<T> {
    meeting_condition_positions
}

/// Retrieves the subset of positions that do not meet the specified condition based on the flag component.
pub fn get_elements_not_meeting_condition<T: Copy + Into<TilePosition>>(
    In((_, not_meeting_condition_positions)): In<(Vec<T>, Vec<T>)>,
) -> Vec<T> {
    not_meeting_condition_positions
}

/// Example system that adds the `BlockWalk` component to all positions that contain entities
/// with is_not_walkable flag. This needs to be made more generic and configurable.
pub fn check_interaction_flags<C: AppearanceAssets>(
    mut commands: Commands,
    appearance_assets: Res<C>,
    q_updated_game_object_ids: Query<(Entity, &GameObjectId), Changed<GameObjectId>>,
) {
    let appearances = appearance_assets.prepared_appearances();
    for (entity, object_id) in q_updated_game_object_ids.iter() {
        let is_not_walkable = || -> Option<bool> {
            let (group, id) = object_id.as_group_and_id()?;
            let appearance = appearances.get_for_group(group, id).cloned()?;
            appearance.flags?.is_not_walkable
        };

        if is_true(is_not_walkable()) {
            commands.entity(entity).insert(BlockWalk);
        } else {
            commands.entity(entity).remove::<BlockWalk>();
        }
    }
}
