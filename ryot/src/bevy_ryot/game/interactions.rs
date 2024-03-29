//! This module provides a handy way of dealing with tile level interactions like movement and
//! sight restrictions, and more. Any component flag like BlockWalk, BlockSight or others
//! custom defined behaviors can be used to filter out positions or entities.
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
//!         .pipe(filter_positions_by_flag::<(&BlockWalk, &BlockSight)>)
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
pub fn with_flag(In(positions): In<Vec<TilePosition>>) -> (FilterMode, Vec<TilePosition>) {
    (FilterMode::WithFlag, positions)
}

/// Helper system that initializes the interaction filtering pipeline from a list of positions
/// with an intention of filtering for positions that don't contain entities with a specific flag.
pub fn without_flag(In(positions): In<Vec<TilePosition>>) -> (FilterMode, Vec<TilePosition>) {
    (FilterMode::WithoutFlag, positions)
}

/// Main system that filters the positions based on the flag component provided in the query.
/// It receives an intention and an array of positions, and returns a filtered array of positions.
///
/// This system will go through all the positions and check if any entity in the position has the
/// given flag component. If an entity with the flag is found, the position is kept in the list,
/// otherwise it is removed. It only considers entities that are visible.
pub fn filter_positions_by_flag<F: QueryData>(
    In((mode, positions)): In<(FilterMode, Vec<TilePosition>)>,
    map_tiles: Res<MapTiles<Entity>>,
    q_flag: Query<F>,
    q_visibility: Query<&Visibility>,
) -> Vec<TilePosition> {
    positions
        .iter()
        .filter_map(|pos| {
            let contains_flag = map_tiles.get(pos)?.clone().into_iter().any(|(_, entity)| {
                q_flag.contains(entity)
                    && !matches!(q_visibility.get(entity), Ok(Visibility::Hidden))
            });

            match mode {
                FilterMode::WithFlag if contains_flag => Some(*pos),
                FilterMode::WithoutFlag if !contains_flag => Some(*pos),
                _ => None,
            }
        })
        .collect()
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
