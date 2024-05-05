use bevy_ecs::prelude::*;
use ryot_core::game::Point;
use std::collections::VecDeque;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Collision<T, P> {
    pub position: P,
    pub distance: f32,
    pub previous_position: P,
    pub pierced: bool,
    _marker: PhantomData<T>,
}

impl<T, P> Collision<T, P> {
    pub fn new(position: P, distance: f32, previous_position: P) -> Self {
        Self {
            position,
            distance,
            previous_position,
            pierced: false,
            _marker: PhantomData,
        }
    }

    pub fn pierced(mut self) -> Self {
        self.pierced = true;
        self
    }
}

/// Represents a collection of tile positions of interest for an entity, based on a trajectory T.
///
/// This component is used to track and share tile positions that an entity, through its specific
/// trajectory (defined by the `V` trait), deems significant. These positions could represent areas
/// the entity can see, move towards, or interact with in some capacity.
///
/// The `shared_with` field allows these positions to be shared with other entities, enabling
/// collaborative or team-based mechanics where multiple entities can benefit from shared traversals
/// or strategic information.
///
/// This struct facilitates diverse gameplay mechanics by allowing entities to dynamically respond
/// to and share critical spatial information within the game world.
#[derive(Clone, Component, Debug, PartialEq)]
pub struct TrajectoryResult<T, P> {
    pub collisions: VecDeque<Collision<T, P>>,
    pub area_of_interest: VecDeque<P>,
}

impl<T, P> Default for TrajectoryResult<T, P> {
    fn default() -> Self {
        Self {
            collisions: VecDeque::new(),
            area_of_interest: VecDeque::new(),
        }
    }
}

impl<T, P> TrajectoryResult<T, P> {
    pub fn new(collisions: VecDeque<Collision<T, P>>, area_of_interest: VecDeque<P>) -> Self {
        Self {
            collisions,
            area_of_interest,
        }
    }
}

impl<T, P: Point> TrajectoryResult<T, P> {
    pub fn get_collisions_last_positions(&self) -> VecDeque<P> {
        let mut last_positions = VecDeque::new();

        for intersection in self.collisions.iter().rev() {
            if intersection.pierced {
                continue;
            }

            if !self
                .collisions
                .iter()
                .any(|intersection| intersection.position == intersection.previous_position)
            {
                last_positions.push_back(intersection.previous_position);
            }
        }

        last_positions
    }
}
