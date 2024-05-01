//! This module focuses on updating and processing the perspectives and visibility
//! for entities based on their positions and visibility conditions.
//! It leverages RadialAreas to calculate potential intersections and updates
//! entities' visible positions accordingly.
use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::bounding::Aabb3d;
use ryot_core::prelude::{Navigable, Point};
use ryot_utils::prelude::*;
use std::sync::mpsc;

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
pub fn update_intersection_cache<
    P: Point + Into<Aabb3d> + Send + Sync + 'static,
    T: Trajectory<Position = P>,
>(
    mut intersection_cache: ResMut<SimpleCache<RadialArea<P>, Vec<Vec<P>>>>,
    q_radial_areas: Query<&T, Changed<T>>,
) {
    q_radial_areas.iter().for_each(|trajectory| {
        let radial_area: RadialArea<P> = trajectory.get_area();

        intersection_cache
            .entry(radial_area)
            .or_insert_with(|| Perspective::<P>::from(radial_area).get_intersections());
    });
}

/// Processes each entity's perspective to update its visible positions.
/// This system filters the previously calculated intersections based on actual visibility
/// conditions (e.g., obstructions, visibility flags) and updates each entity's InterestPositions.
///
/// Run as part of [`PerspectiveSystems::CalculatePerspectives`].
pub fn process_trajectories<
    P: Point + Send + Sync + 'static,
    T: Trajectory<Position = P>,
    N: Navigable + Copy + Default,
>(
    mut commands: Commands,
    flags_cache: Res<Cache<P, N>>,
    intersection_cache: Res<SimpleCache<RadialArea<P>, Vec<Vec<P>>>>,
    mut q_radial_areas: Query<(Entity, &T)>,
) {
    let Ok(read_guard) = flags_cache.read() else {
        return;
    };

    let (tx, rx) = mpsc::channel::<(Entity, InterestPositions<T>)>();

    q_radial_areas
        .par_iter_mut()
        .for_each(|(entity, trajectory)| {
            let radial_area: RadialArea<P> = trajectory.get_area();

            let Some(intersections_per_trajectory) = intersection_cache.get(&radial_area) else {
                return;
            };

            let mut valid_positions = Vec::new();

            for intersections in intersections_per_trajectory {
                for pos in intersections {
                    let flags = read_guard.get(pos).copied().unwrap_or_default();

                    if trajectory.meets_condition(&flags, pos) {
                        valid_positions.push(*pos);
                    } else {
                        break;
                    }
                }
            }

            tx.send((entity, InterestPositions::new(valid_positions)))
                .ok();
        });

    for (entity, positions) in rx.try_iter() {
        commands.entity(entity).insert(positions);
    }
}

pub fn share_trajectories<T: Trajectory<Position: Clone + Send + Sync + 'static> + Clone>(
    mut q_interest_positions: Query<(Entity, &mut InterestPositions<T>)>,
    q_shared_with: Query<&ShareTrajectoryWith<T>>,
) {
    let (tx, rx) = mpsc::channel::<(Entity, InterestPositions<T>)>();

    q_interest_positions
        .iter()
        .for_each(|(entity, interest_positions)| {
            if let Ok(share_with) = q_shared_with.get(entity) {
                for shared_entity in &share_with.shared_with {
                    tx.send((*shared_entity, interest_positions.clone())).ok();
                }
            }
        });

    for (entity, positions) in rx.try_iter() {
        if let Ok((_, mut interest_positions)) = q_interest_positions.get_mut(entity) {
            interest_positions.positions.extend(positions.positions);
        }
    }
}
