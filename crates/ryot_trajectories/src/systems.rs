//! This module focuses on updating and processing the perspectives and visibility
//! for entities based on their positions and visibility conditions.
//! It leverages RadialAreas to calculate potential intersections and updates
//! entities' visible positions accordingly.
use crate::prelude::*;
use bevy_ecs::prelude::*;
use ryot_core::prelude::Navigable;
use ryot_utils::prelude::*;
use std::collections::VecDeque;
use std::sync::mpsc;

/// Defines system sets for managing perspective calculation systems.
/// This enum categorizes systems related to perspective calculations, facilitating the organization
/// and prioritization of systems that calculate and update entity perspectives based on game state.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum TrajectorySystems {
    ProcessTrajectories,
}

/// Updates the cache of intersections for radial view points that have changed.
/// This is crucial for maintaining an updated view of what each entity can potentially see,
/// based on their RadialArea and the current state of the game world.
///
/// Run as part of [`CacheSystems::UpdateCache`].
pub fn update_intersection_cache<T: Copy + Send + Sync + 'static, P: TrajectoryPoint>(
    mut intersection_cache: ResMut<SimpleCache<RadialArea<P>, Vec<Vec<P>>>>,
    q_radial_areas: Query<&Trajectory<T, P>, Changed<Trajectory<T, P>>>,
) {
    q_radial_areas.iter().for_each(|trajectory| {
        let radial_area: RadialArea<P> = trajectory.area;

        intersection_cache
            .entry(radial_area)
            .or_insert_with(|| Perspective::<P>::from(radial_area).get_intersections());
    });
}

/// Processes each entity's perspective to update its visible positions.
/// This system filters the previously calculated intersections based on actual visibility
/// conditions (e.g., obstructions, visibility flags) and updates each entity's InterestPositions.
///
/// Run as part of [`TrajectorySystems::ProcessTrajectories`].
pub fn process_trajectories<
    T: Copy + Send + Sync + 'static,
    P: TrajectoryPoint,
    N: Navigable + Copy + Default,
>(
    mut commands: Commands,
    flags_cache: Res<Cache<P, N>>,
    intersection_cache: Res<SimpleCache<RadialArea<P>, Vec<Vec<P>>>>,
    mut q_radial_areas: Query<(Entity, &Trajectory<T, P>)>,
) {
    let Ok(read_guard) = flags_cache.read() else {
        return;
    };

    let (tx, rx) = mpsc::channel::<(Entity, InterestPositions<T, P>)>();

    q_radial_areas
        .par_iter_mut()
        .for_each(|(entity, trajectory)| {
            let radial_area: RadialArea<P> = trajectory.area;

            let Some(intersections_per_trajectory) = intersection_cache.get(&radial_area) else {
                return;
            };

            let mut valid_positions = VecDeque::new();

            for intersections in intersections_per_trajectory {
                for pos in intersections {
                    let flags = read_guard.get(pos).copied().unwrap_or_default();

                    if trajectory.meets_condition(&flags, pos) {
                        valid_positions.push_back(*pos);
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

pub fn share_trajectories<T: Copy + Send + Sync + 'static, P: TrajectoryPoint>(
    mut q_interest_positions: Query<(&ShareTrajectoryWith<T, P>, &mut InterestPositions<T, P>)>,
    mut q_interest_positions_only: Query<
        &mut InterestPositions<T, P>,
        Without<ShareTrajectoryWith<T, P>>,
    >,
) {
    let (tx, rx) = mpsc::channel::<(Entity, InterestPositions<T, P>)>();

    q_interest_positions
        .iter()
        .for_each(|(share_with, interest_positions)| {
            for shared_entity in &share_with.shared_with {
                tx.send((*shared_entity, interest_positions.clone())).ok();
            }
        });

    for (entity, positions) in rx.try_iter() {
        if let Ok((_, mut interest_positions)) = q_interest_positions.get_mut(entity) {
            interest_positions.positions.extend(positions.positions);
        } else if let Ok(mut interest_positions) = q_interest_positions_only.get_mut(entity) {
            interest_positions.positions.extend(positions.positions);
        }
    }
}
