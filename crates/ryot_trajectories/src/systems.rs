//! This module focuses on updating and processing the perspectives and visibility
//! for entities based on their positions and visibility conditions.
//! It leverages RadialAreas to calculate potential intersections and updates
//! entities' visible positions accordingly.
use crate::prelude::*;
use bevy_ecs::prelude::*;
use ryot_core::prelude::Navigable;
use ryot_utils::prelude::*;
use std::sync::mpsc;

/// Defines system sets for managing perspective calculation systems.
/// This enum categorizes systems related to perspective calculations, facilitating the organization
/// and prioritization of systems that calculate and update entity perspectives based on game state.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum TrajectorySystems {
    ProcessTrajectories,
    CleanUp,
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
/// conditions (e.g., obstructions, visibility flags) and updates each entity's Intersections.
///
/// Run as part of [`TrajectorySystems::ProcessTrajectories`].
pub fn process_trajectories<
    T: Copy + Send + Sync + 'static,
    P: TrajectoryPoint + Component,
    N: Navigable + Copy + Default,
>(
    mut commands: Commands,
    flags_cache: Res<Cache<P, N>>,
    intersection_cache: Res<SimpleCache<RadialArea<P>, Vec<Vec<P>>>>,
    mut q_radial_areas: Query<(Entity, &P, &mut Trajectory<T, P>)>,
) {
    let Ok(read_guard) = flags_cache.read() else {
        return;
    };

    let (tx, rx) = mpsc::channel::<(Entity, Intersections<T, P>)>();

    q_radial_areas
        .par_iter_mut()
        .for_each(|(entity, from, mut trajectory)| {
            let Some(intersections_per_trajectory) = intersection_cache.get(&trajectory.area)
            else {
                return;
            };

            let result = trajectory.execute(from, intersections_per_trajectory, |pos| {
                read_guard.get(pos).copied().unwrap_or_default()
            });

            let Some(intersections) = result else {
                return;
            };

            tx.send((entity, intersections)).ok();
        });

    for (entity, positions) in rx.try_iter() {
        commands.entity(entity).insert(positions);
    }
}

pub fn share_results<T: Copy + Send + Sync + 'static, P: TrajectoryPoint>(
    mut q_trajectory_intersections: Query<(&Trajectory<T, P>, &mut Intersections<T, P>)>,
    mut q_intersections: Query<&mut Intersections<T, P>, Without<Trajectory<T, P>>>,
) {
    let (tx, rx) = mpsc::channel::<(Entity, Intersections<T, P>)>();

    q_trajectory_intersections
        .iter()
        .for_each(|(trajectory, intersections)| {
            for shared_entity in &trajectory.shared_with {
                tx.send((*shared_entity, intersections.clone())).ok();
            }
        });

    for (entity, shared) in rx.try_iter() {
        if let Ok((_, mut intersections)) = q_trajectory_intersections.get_mut(entity) {
            intersections
                .area_of_interest
                .extend(shared.area_of_interest);
            intersections.hits.extend(shared.hits);
        } else if let Ok(mut intersections) = q_intersections.get_mut(entity) {
            intersections
                .area_of_interest
                .extend(shared.area_of_interest);
            intersections.hits.extend(shared.hits);
        }
    }
}

pub fn remove_orphan_intersections<T: Copy + Send + Sync + 'static, P: TrajectoryPoint>(
    mut commands: Commands,
    q_orphan_intersections: Query<Entity, (With<Intersections<T, P>>, Without<Trajectory<T, P>>)>,
) {
    q_orphan_intersections.iter().for_each(|entity| {
        commands.entity(entity).remove::<Intersections<T, P>>();
    });
}

pub fn remove_stale_trajectories<T: Copy + Send + Sync + 'static, P: TrajectoryPoint>(
    mut commands: Commands,
    q_trajectories: Query<(Entity, &Trajectory<T, P>)>,
) {
    q_trajectories.iter().for_each(|(entity, trajectory)| {
        if trajectory.last_execution().is_none() {
            return;
        }

        match trajectory.execution_type() {
            ExecutionType::Once => {
                commands.entity(entity).remove::<Trajectory<T, P>>();
            }
            ExecutionType::TimeBased(_) => (),
        }
    });
}
