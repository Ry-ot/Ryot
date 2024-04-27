use crate::prelude::*;
use bevy_ecs::prelude::*;
use derive_more::*;
use std::hash::Hash;
use std::time::Duration;

/// Component that represents a new path finding query request.
/// This is the component that should be added to your entity when you want to initiate
/// a path finding calculation.
#[derive(Component, Copy, Clone)]
pub struct PathFindingQuery<N: Eq + Hash + Copy + Clone> {
    pub to: N,
    pub timeout: Option<Duration>,
}

/// Component that holds the result of a path finding calculation.
/// This is the component you should use when you want to process path finding results
/// for your entity.
#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct Path<P: Pathable>(pub(crate) Vec<P>);

/// Component that holds a task running path finding with a potential path find result.
#[derive(Component)]
pub(crate) struct PathFindingTask<P: Pathable>(pub(crate) bevy_tasks::Task<Option<(Vec<P>, u32)>>);

impl<P: Pathable + Default> Default for PathFindingQuery<P> {
    fn default() -> Self {
        PathFindingQuery {
            to: P::default(),
            timeout: None,
        }
    }
}

impl<P: Pathable> PathFindingQuery<P> {
    pub fn new(to: P) -> Self {
        PathFindingQuery { to, timeout: None }
    }

    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self {
            timeout: Some(timeout),
            ..self
        }
    }
}
