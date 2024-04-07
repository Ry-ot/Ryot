//! This module focuses on updating and processing the perspectives and visibility
//! for entities based on their positions and visibility conditions.
//! It leverages RadialAreas to calculate potential intersections and updates
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
/// based on their RadialArea and the current state of the game world.
///
/// Run as part of [`CacheSystems::UpdateCache`].
pub fn update_intersection_cache<T: Trajectory>(
    mut intersection_cache: ResMut<Cache<RadialArea, Vec<Vec<TilePosition>>>>,
    q_radial_areas: Query<&T, Changed<T>>,
) {
    q_radial_areas.iter().for_each(|trajectory| {
        let radial_area: RadialArea = trajectory.get_area();

        intersection_cache
            .entry(radial_area)
            .or_insert_with(|| Perspective::from(radial_area).get_intersections());
    });
}

/// Processes each entity's perspective to update its visible positions.
/// This system filters the previously calculated intersections based on actual visibility
/// conditions (e.g., obstructions, visibility flags) and updates each entity's InterestPositions.
///
/// Run as part of [`PerspectiveSystems::CalculatePerspectives`].
pub fn process_perspectives<T: Trajectory>(
    tile_flags_cache: Res<Cache<TilePosition, TileFlags>>,
    intersection_cache: Res<Cache<RadialArea, Vec<Vec<TilePosition>>>>,
    mut q_radial_areas: Query<(Entity, &T, &mut InterestPositions<T>)>,
) {
    let (tx, rx) = mpsc::channel::<(Entity, TilePosition)>();

    q_radial_areas
        .par_iter_mut()
        .for_each(|(entity, trajectory, mut interest_positions)| {
            interest_positions.positions.clear();

            let radial_area: RadialArea = trajectory.get_area();

            let Some(intersections_per_trajectory) = intersection_cache.get(&radial_area) else {
                return;
            };

            for intersections in intersections_per_trajectory {
                for pos in intersections {
                    let flags = tile_flags_cache.get(pos).copied().unwrap_or_default();

                    if trajectory.meets_condition(&flags, pos) {
                        tx.send((entity, *pos)).ok();
                    } else {
                        break;
                    }
                }
            }
        });

    for (entity, pos) in rx.try_iter() {
        let mut shared_with_vec = Vec::new();
        if let Ok((_, _, mut interest_positions)) = q_radial_areas.get_mut(entity) {
            interest_positions.positions.push(pos);

            for shared_with in &interest_positions.shared_with {
                shared_with_vec.push(*shared_with);
            }
        };

        for shared_with in shared_with_vec {
            if let Ok((_, _, mut interest_positions)) = q_radial_areas.get_mut(shared_with) {
                interest_positions.positions.push(pos);
            };
        }
    }

    q_radial_areas
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
