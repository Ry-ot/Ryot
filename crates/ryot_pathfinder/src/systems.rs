use crate::components::{Path, PathFindingQuery, PathFindingTask};
use crate::Pathable;
use bevy_ecs::prelude::*;
use bevy_tasks::*;
use ryot_utils::prelude::*;
use ryot_utils::Flag;

/// Defines system sets for managing perspective calculation systems.
/// This enum categorizes systems related to perspective calculations, facilitating the organization
/// and prioritization of systems that calculate and update entity perspectives based on game state.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PathFindingSystems {
    TriggerTask,
    ExecuteTask,
}

/// System that performs a path finding request query and starts a path finding
/// async task based on the request params.
pub(super) fn trigger_path_finding_tasks<P: Pathable + Component, F: Flag>(
    mut commands: Commands,
    tile_flags_cache: Res<Cache<P, F>>,
    q_path_finding_query: Query<(Entity, &P, &PathFindingQuery<P>), Changed<PathFindingQuery<P>>>,
) {
    for (entity, from, query) in &q_path_finding_query {
        let (from, query) = (*from, *query);
        let flags_cache = tile_flags_cache.clone();
        let thread_pool = AsyncComputeTaskPool::get();

        commands
            .entity(entity)
            .insert(PathFindingTask(thread_pool.spawn(async move {
                from.path_to(
                    query.to,
                    |p| {
                        flags_cache
                            .get(p)
                            .copied()
                            .unwrap_or_default()
                            .is_walkable()
                    },
                    query.timeout,
                )
            })));
    }
}

/// System that consolidates the results of the async path finding tasks and inserts the
/// resulting path into the entity that requested the path finding.
pub(super) fn handle_path_finding_tasks<P: Pathable>(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut PathFindingTask<P>)>,
) {
    for (entity, mut task) in &mut transform_tasks {
        let Some(result) = block_on(poll_once(&mut task.0)) else {
            continue;
        };

        if let Some((path, _)) = result {
            commands.entity(entity).insert(Path(path));
        };

        commands.entity(entity).remove::<PathFindingTask<P>>();
    }
}
