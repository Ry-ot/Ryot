//! This module focuses on updating and processing the perspectives and visibility
//! for entities based on their positions and visibility conditions.
//! It leverages RadialViewPoints to calculate potential intersections and updates
//! entities' visible positions accordingly.
use std::sync::mpsc;

use bevy::prelude::*;
use itertools::Itertools;

use crate::position::TilePosition;
use crate::prelude::{perspective::*, tile_flags::TileFlags, *};

/// Defines system sets for managing perspective calculation systems.
/// This enum categorizes systems related to perspective calculations, facilitating the organization
/// and prioritization of systems that calculate and update entity perspectives based on game state.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PerspectiveSystems {
    CalculatePerspectives,
}

/// Updates the cache of intersections for radial view points that have changed.
/// This is crucial for maintaining an updated view of what each entity can potentially see,
/// based on their RadialViewPoint and the current state of the game world.
///
/// Run as part of [`CacheSystems::UpdateCache`].
pub fn update_intersection_cache<V: ConditionalViewPoint>(
    mut intersection_cache: ResMut<Cache<RadialViewPoint, Vec<Vec<TilePosition>>>>,
    q_radial_view_points: Query<&V, Changed<V>>,
) {
    q_radial_view_points.iter().for_each(|view_point| {
        let radial_view_point: RadialViewPoint = view_point.get_view_point();

        intersection_cache
            .entry(radial_view_point)
            .or_insert_with(|| Perspective::from(radial_view_point).get_filtered_intersections());
    });
}

/// Processes each entity's perspective to update its visible positions.
/// This system filters the previously calculated intersections based on actual visibility
/// conditions (e.g., obstructions, visibility flags) and updates each entity's InterestPositions.
///
/// Run as part of [`PerspectiveSystems::CalculatePerspectives`].
pub fn process_perspectives<V: ConditionalViewPoint>(
    tile_flags_cache: Res<Cache<TilePosition, TileFlags>>,
    intersection_cache: Res<Cache<RadialViewPoint, Vec<Vec<TilePosition>>>>,
    mut q_radial_view_points: Query<(Entity, &V, &mut InterestPositions<V>)>,
) {
    let (tx, rx) = mpsc::channel::<(Entity, TilePosition)>();

    q_radial_view_points
        .par_iter_mut()
        .for_each(|(entity, view_point, mut interest_positions)| {
            interest_positions.positions.clear();

            let radial_view_point: RadialViewPoint = view_point.get_view_point();

            let Some(intersections_per_view_point) = intersection_cache.get(&radial_view_point)
            else {
                return;
            };

            for intersections in intersections_per_view_point {
                for pos in intersections {
                    let flags = tile_flags_cache.get(pos).copied().unwrap_or_default();

                    if view_point.meets_condition(&flags, pos) {
                        tx.send((entity, *pos)).ok();
                    } else {
                        break;
                    }
                }
            }
        });

    for (entity, pos) in rx.try_iter() {
        if let Ok((_, _, mut interest_positions)) = q_radial_view_points.get_mut(entity) {
            interest_positions.positions.push(pos);
        };
    }

    q_radial_view_points
        .par_iter_mut()
        .for_each(|(_, _, mut interest_positions)| {
            interest_positions.positions = interest_positions
                .positions
                .iter()
                .unique()
                .copied()
                .collect();
        });
}
