use crate::components::{Path, PathFindingQuery, PathFindingTask};
use crate::prelude::{AmendPathCommand, Pathable};
use bevy_ecs::prelude::*;
use bevy_tasks::*;
use ryot_core::prelude::{Flags, Navigable};
use ryot_utils::prelude::*;
use std::sync::Arc;
use std::time::Duration;

/// Defines system sets for managing perspective calculation systems.
/// This enum categorizes systems related to perspective calculations, facilitating the organization
/// and prioritization of systems that calculate and update entity perspectives based on game state.
///
/// TriggerTask: Systems related to initiating pathfinding tasks based on changes to pathfinding queries
/// will run under this category.
///
/// ExecuteTask: Systems responsible for processing the results of pathfinding tasks, updating entities,
/// storing results, and cleaning up resources will run under this category.
///
/// You can also use those categories to schedule your systems accordingly. E.g. if you have a system
/// that needs to run before the async tasks are scheduled or after the results are processed.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PathFindingSystems {
    TriggerTask,
    ExecuteTask,
}

/// Initiates pathfinding async tasks based on changes to pathfinding queries, leveraging
/// a provided cache for environmental data.
pub(super) fn trigger_path_finding_tasks<P: Pathable + Component, N: Navigable + Copy + Default>(
    mut commands: Commands,
    flags_cache: Res<Cache<P, N>>,
    q_path_finding_query: Query<(Entity, &P, &PathFindingQuery<P>), Changed<PathFindingQuery<P>>>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for (entity, from, query) in &q_path_finding_query {
        let (from, query) = (*from, *query);
        let flags_cache_arc = Arc::clone(&flags_cache);

        commands
            .entity(entity)
            .insert(PathFindingTask(thread_pool.spawn(async move {
                from.path_to(&query, |p| p.can_be_navigated(flags_cache_arc.clone()))
            })));
    }
}

/// Processes the results of pathfinding tasks, updating entities with new paths and cleaning up
/// resources once calculations are complete.
pub(super) fn handle_path_finding_tasks<P: Pathable>(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut PathFindingTask<P>)>,
) {
    for (entity, mut task) in &mut transform_tasks {
        let Some(result) = block_on(poll_once(&mut task.0)) else {
            continue;
        };

        if let Some((path, _)) = result {
            commands.entity(entity).insert(Path::new(path));
        };

        commands.entity(entity).remove::<PathFindingQuery<P>>();
        commands.entity(entity).remove::<PathFindingTask<P>>();
    }
}

pub(super) fn amend_path<P: Pathable + Component>(
    mut amend_command_reader: EventReader<AmendPathCommand<P>>,
    mut q_actor: Query<(&mut Path<P>, &P)>,
    flags_cache: Res<Cache<P, Flags>>,
) {
    let flags_cache_arc = Arc::clone(&flags_cache);

    for command in amend_command_reader.read() {
        let Ok((mut path, actor_position)) = q_actor.get_mut(command.entity) else {
            continue;
        };

        let mut path_finding_query = command.path_finding_query;

        if path_finding_query.timeout.is_none() {
            path_finding_query = path_finding_query.with_timeout(Duration::from_millis(50));
        }

        let change = actor_position.path_to(&path_finding_query, |p| {
            p.can_be_navigated(flags_cache_arc.clone())
        });

        let Some((mut new_tiles, _)) = change else {
            continue;
        };

        if new_tiles.is_empty() {
            continue;
        }

        new_tiles.extend(
            path.iter()
                .enumerate()
                .skip(command.path_amend_index)
                .map(|(_, &p)| p),
        );

        *path = Path::new(new_tiles);
    }
}
