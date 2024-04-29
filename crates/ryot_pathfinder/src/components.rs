//! This module contains the components used by the pathfinding system, and represents the main
//! API for interacting with pathfinding operations in the Bevy ECS. It includes the interfaces
//! to request pathfinding operations and to store the resulting paths.
//!
//! PathFindingQuery<P> is the entry point for pathfinding requests and Path<P> stores the path
//! that represents the output of a pathfinding operation. Everything that happens between the
//! entry point and the output is managed by the pathfinding systems, which is responsible for
//! handling the asynchronous execution of pathfinding tasks, via the PathFindingTask<P> internal
//! component.
use crate::prelude::*;
use bevy_ecs::prelude::*;
use derive_more::*;
use std::time::Duration;

/// Representing the entry point for pathfinding systems, this component is a request query
/// that contains the necessary information to calculate a path between two points. It includes
/// the destination, cost values, and acceptable proximity to the goal.
///
/// One can trigger a pathfinding task by adding this component to an entity, which will then
/// be automatically processed by the pathfinding systems.
///
/// Example:
/// ```rust
/// use std::time::Duration;
/// use bevy_ecs::prelude::*;
/// use ryot_pathfinder::prelude::*;
///
/// fn trigger_pathfinding<P: Pathable + Default>(
///     mut commands: Commands,
/// ) {
///     // basic pathfinding query
///     commands.spawn(PathFindingQuery::new(P::generate(0, 0, 0)));
/// }
///
/// fn trigger_complex_pathfinding<P: Pathable + Default>(
///     mut commands: Commands,
/// ) {
///     // pathfinding query with custom parameters
///     commands.spawn(
///         PathFindingQuery::new(P::generate(0, 0, 0))
///             .with_cardinal_cost(2) // moving in cardinal directions is cheaper
///             .with_diagonal_cost(500) // diagonal movements are more expensive, so the path will prefer cardinal directions
///             .with_success_distance(0.) // will only stop when it reaches the exact position
///             .with_timeout(Duration::from_secs(5)), // will stop the async task after 5 seconds
///     );
/// }
#[derive(Component, Copy, Clone)]
pub struct PathFindingQuery<P: Pathable> {
    pub to: P,
    pub cardinal_cost: u32,
    pub diagonal_cost: u32,
    pub success_distance: f32,
    pub timeout: Option<Duration>,
}

/// Represents the output of a pathfinding operation, this component stores the calculated path
/// resulting from the async task that processes the pathfinding query. It contains a series of
/// steps or nodes that an entity can follow to reach its destination. It's attached to the same
/// entity that requested the pathfinding operation.
///
/// Example:
/// ```rust
/// use bevy_ecs::prelude::*;
/// use ryot_pathfinder::prelude::*;
///
/// fn read_path<P: Pathable>(
///     mut query: Query<&mut Path<P>>,
/// ) {
///     for mut path in query.iter_mut() {
///         if path.is_empty() {
///             continue;
///         }
///
///         let Some(next_pos) = path.first().copied() else {
///             continue;
///         };
///
///         path.remove(0);
///         // do something with next_pos
///     }
/// }
#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct Path<P: Pathable>(pub(crate) Vec<P>);

/// Manages the asynchronous execution of pathfinding tasks, holding a future
/// that resolves to the computed path and associated costs.
#[derive(Component)]
pub(crate) struct PathFindingTask<P: Pathable>(pub(crate) bevy_tasks::Task<Option<(Vec<P>, u32)>>);

impl<P: Pathable + Default> Default for PathFindingQuery<P> {
    fn default() -> Self {
        PathFindingQuery {
            to: P::default(),
            timeout: None,
            cardinal_cost: 1,
            diagonal_cost: 500,
            success_distance: 1.,
        }
    }
}

impl<P: Pathable + Default> PathFindingQuery<P> {
    pub fn new(to: P) -> Self {
        PathFindingQuery {
            to,
            ..Self::default()
        }
    }
}

impl<P: Pathable> PathFindingQuery<P> {
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self {
            timeout: Some(timeout),
            ..self
        }
    }

    pub fn with_cardinal_cost(self, cardinal_cost: u32) -> Self {
        Self {
            cardinal_cost,
            ..self
        }
    }

    pub fn with_diagonal_cost(self, diagonal_cost: u32) -> Self {
        Self {
            diagonal_cost,
            ..self
        }
    }

    pub fn with_success_distance(self, success_distance: f32) -> Self {
        Self {
            success_distance,
            ..self
        }
    }
}
