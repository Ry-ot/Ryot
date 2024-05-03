use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_utils::HashSet;
use ryot_core::prelude::Navigable;
use std::marker::PhantomData;
use std::time::Duration;

pub enum TrajectoryType {
    OneTime,
    Time(Duration),
}

/// Represents entities that can provide a `RadialArea` for perspective calculation.
///
/// This trait facilitates the generation of a radial area based on an entity's current state or
/// position. It is used to abstract the way different entities determine their perspective in the
/// world. The `meets_condition` method allows for additional checks on environmental or
/// entity-specific conditions that may affect whether a position is considered valid for certain
/// operations within the trajectory area, like visibility checks or interactions.
#[derive(Debug, Clone, Eq, PartialEq, Component)]
pub struct Trajectory<T, P> {
    pub area: RadialArea<P>,
    pub shared_with: HashSet<Entity>,
    condition: fn(&Self, &dyn Navigable, &P) -> bool,
    marker: PhantomData<T>,
}

impl<T, P> Trajectory<T, P> {
    pub fn new(area: RadialArea<P>, condition: fn(&Self, &dyn Navigable, &P) -> bool) -> Self {
        Self {
            area,
            condition,
            shared_with: HashSet::default(),
            marker: PhantomData,
        }
    }
}

impl<T, P: Copy> Trajectory<T, P> {
    pub fn share_with(mut self, entities: Vec<Entity>) -> Self {
        self.shared_with.extend(entities);
        self
    }

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
