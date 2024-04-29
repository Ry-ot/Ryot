use crate::prelude::*;
use bevy_ecs::prelude::*;
use derive_more::*;
use std::time::Duration;

/// Represents a request for pathfinding, detailing the destination and parameters
/// for the calculation like cost metrics and acceptable proximity to the goal.
/// Attach this to entities that require pathfinding operations.
#[derive(Component, Copy, Clone)]
pub struct PathFindingQuery<P: Pathable> {
    pub to: P,
    pub cardinal_cost: u32,
    pub diagonal_cost: u32,
    pub success_distance: f32,
    pub timeout: Option<Duration>,
}

/// Stores the calculated path as a series of steps or nodes that an entity can follow.
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
