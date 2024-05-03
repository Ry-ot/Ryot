use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::Reflect;
use bevy_utils::HashSet;
use ryot_core::prelude::Navigable;
use std::marker::PhantomData;

/// Represents entities that can provide a `RadialArea` for perspective calculation.
///
/// This trait facilitates the generation of a radial area based on an entity's current state or
/// position. It is used to abstract the way different entities determine their perspective in the
/// world. The `meets_condition` method allows for additional checks on environmental or
/// entity-specific conditions that may affect whether a position is considered valid for certain
/// operations within the trajectory area, like visibility checks or interactions.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub struct Trajectory<T, P> {
    pub area: RadialArea<P>,
    condition: fn(&Self, &dyn Navigable, &P) -> bool,
    marker: PhantomData<T>,
}

impl<T, P> Trajectory<T, P> {
    pub fn new(area: RadialArea<P>, condition: fn(&Self, &dyn Navigable, &P) -> bool) -> Self {
        Self {
            area,
            condition,
            marker: PhantomData,
        }
    }
}

impl<T, P: Copy> Trajectory<T, P> {
    pub fn meets_condition<N: Navigable>(&self, flags: &N, position: &P) -> bool {
        (self.condition)(self, flags, position)
    }
}

pub fn visible_trajectory<T, P>(area: RadialArea<P>) -> Trajectory<T, P> {
    Trajectory::<T, P>::new(area, |_, flags, _pos| !flags.blocks_sight())
}

pub fn walkable_trajectory<T, P>(area: RadialArea<P>) -> Trajectory<T, P> {
    Trajectory::<T, P>::new(area, |_, flags, _pos| flags.is_walkable())
}

#[derive(Clone, Component, Debug, Reflect)]
pub struct ShareTrajectoryWith<T, P> {
    #[reflect(ignore)]
    pub shared_with: HashSet<Entity>,
    _phantom: PhantomData<Trajectory<T, P>>,
}

impl<T, P> Default for ShareTrajectoryWith<T, P> {
    fn default() -> Self {
        Self {
            shared_with: HashSet::default(),
            _phantom: PhantomData,
        }
    }
}

impl<T, P> ShareTrajectoryWith<T, P> {
    /// Allows sharing visibility with additional entities. This can be used in team-based or
    /// cooperative scenarios, where visibility information should be shared among allies.
    pub fn share_with(mut self, entities: Vec<Entity>) -> Self {
        self.shared_with.extend(entities);
        self
    }
}
