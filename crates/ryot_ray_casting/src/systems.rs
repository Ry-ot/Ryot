//! This module focuses on updating and processing the perspectives and visibility
//! for entities based on their positions and visibility conditions.
//! It leverages RadialAreas to calculate potential intersections and updates
//! entities' visible positions accordingly.
use crate::prelude::*;
use crate::request::{SharedWith, StaleRequest};
use bevy_ecs::prelude::*;
use bevy_mod_index::prelude::Index;
use ryot_core::prelude::Navigable;
use ryot_utils::prelude::*;
use std::sync::mpsc;

/// Defines system sets for managing perspective calculation systems.
/// This enum categorizes systems related to perspective calculations, facilitating the organization
/// and prioritization of systems that calculate and update entity perspectives based on game state.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum RayCastingSystems {
    Process,
    CleanUp,
}

/// Updates the cache of possible intersections over a given RadialArea, reducing the need to
/// recalculate intersections for a same Perspective multiple times. This is crucial for
/// maintaining an updated representation of the ray casting requests and their intersections.
///
/// Run as part of [`CacheSystems::UpdateCache`].
pub fn update_intersection_cache<T: Copy + ThreadSafe, P: RayCastingPoint>(
    mut intersection_cache: ResMut<SimpleCache<RadialArea<P>, Vec<Vec<P>>>>,
    q_radial_areas: Query<&RayCasting<T, P>, Changed<RayCasting<T, P>>>,
) {
    q_radial_areas.iter().for_each(|ray_casting| {
        let radial_area: RadialArea<P> = ray_casting.area;

        intersection_cache
            .entry(radial_area)
            .or_insert_with(|| Perspective::<P>::from(radial_area).get_intersections());
    });
}

/// Processes the ray casting requests for each entity with a RayCasting component.
/// This system only processes the ray casting requests if the intersections for the given
/// RadialArea are already cached, otherwise it skips the request. This is crucial for performance,
/// avoiding unnecessary calculations and ensuring that the cache is always up-to-date.
///
/// This system executes the ray casting requests, generating the RayPropagation results for each
/// entity with a RayCasting component and adding them to the entity.
///
/// Run as part of [`RayCastingSystems::Process`].
pub fn process_ray_casting<
    T: Copy + ThreadSafe,
    P: RayCastingPoint + Component,
    N: Navigable + Copy + Default,
>(
    mut commands: Commands,
    flags_cache: Res<Cache<P, N>>,
    intersection_cache: Res<SimpleCache<RadialArea<P>, Vec<Vec<P>>>>,
    mut q_radial_areas: Query<(Entity, &P, &mut RayCasting<T, P>)>,
) {
    let Ok(read_guard) = flags_cache.read() else {
        return;
    };

    let (tx, rx) = mpsc::channel::<(Entity, RayPropagation<T, P>)>();

    q_radial_areas
        .par_iter_mut()
        .for_each(|(entity, from, mut ray_casting)| {
            let Some(intersections_per_ray) = intersection_cache.get(&ray_casting.area) else {
                return;
            };

            let result = ray_casting.execute(from, intersections_per_ray, |pos| {
                read_guard.get(pos).copied().unwrap_or_default()
            });

            let Some(result) = result else {
                return;
            };

            tx.send((entity, result)).ok();
        });

    for (entity, positions) in rx.try_iter() {
        commands.entity(entity).try_insert(positions);
    }
}

/// Shares the results of ray casting requests with all the entities pointed to by the `shared_with`
/// field in the RayCasting component. This system is crucial for sharing the results of ray casting
/// requests across multiple entities.
pub fn share_results<T: Copy + ThreadSafe, P: RayCastingPoint>(
    mut commands: Commands,
    mut index: Index<SharedWith<T, P>>,
    mut q_propagation: Query<(&RayCasting<T, P>, &mut RayPropagation<T, P>)>,
    mut q_results: Query<&mut RayPropagation<T, P>, Without<RayCasting<T, P>>>,
) {
    let (tx, rx) = mpsc::channel::<(Entity, RayPropagation<T, P>)>();

    index.lookup(&true).for_each(|entity| {
        if let Ok((ray_casting, propagation)) = q_propagation.get_mut(entity) {
            for shared_entity in &ray_casting.shared_with {
                tx.send((*shared_entity, propagation.clone())).ok();
            }
        }
    });

    for (entity, shared) in rx.try_iter() {
        if let Ok((_, mut result)) = q_propagation.get_mut(entity) {
            result.area_of_interest.extend(shared.area_of_interest);
            result.collisions.extend(shared.collisions);
        } else if let Ok(mut result) = q_results.get_mut(entity) {
            result.area_of_interest.extend(shared.area_of_interest);
            result.collisions.extend(shared.collisions);
        } else {
            commands.entity(entity).try_insert(shared);
        }
    }
}

/// This system removes the RayPropagation component from entities that no longer have a RayCasting.
pub fn remove_stale_results<T: Copy + ThreadSafe, P: RayCastingPoint>(
    mut commands: Commands,
    q_orphan_results: Query<Entity, (With<RayPropagation<T, P>>, Without<RayCasting<T, P>>)>,
) {
    q_orphan_results.iter().for_each(|entity| {
        commands.entity(entity).remove::<RayPropagation<T, P>>();
    });
}

/// This system removes the RayCasting component that are no longer valid, based on their execution
/// type and last execution time.
pub fn remove_stale_requests<T: Copy + ThreadSafe, P: RayCastingPoint>(
    mut commands: Commands,
    mut index: Index<StaleRequest<T, P>>,
) {
    index.lookup(&true).for_each(|entity| {
        commands.entity(entity).remove::<RayCasting<T, P>>();
    });
}
