use bevy_ecs::prelude::*;
use ryot_core::prelude::Point;
use std::collections::VecDeque;
use std::marker::PhantomData;

/// Represents the resultant propagation of the rays derived from a given ray casting context T.
///
/// This component is used to track and share how the rays propagate through the designated plane
/// of the game world. It contains a list of collisions and a list of positions that were in the
/// ray's trajectory and deemed significant. These positions could represent areas the spectator
/// can see, move towards, or interact with in some capacity.
///
/// The `RayPropagation` component is used to track the propagation of rays through the game world
/// and is part of the RyotRayCasting public API, being attached to the entity that is the source
/// of the ray casting request.
#[derive(Clone, Component, Debug, PartialEq)]
pub struct RayPropagation<T, P> {
    pub collisions: VecDeque<Collision<T, P>>,
    pub area_of_interest: VecDeque<P>,
}

/// Represents a collision between a ray and a point in the game world.
///
/// This struct is used to track and share the results of ray casting collisions in the game
/// and contains information about the position where the collision occurred, the distance from
/// the ray's origin to the collision point, and the previous position of the ray before the
/// collision occurred.
///
/// Additionally, the `pierced` field indicates whether the ray pierced through the collision point
/// or stopped at it. This information is crucial for determining the visibility of the collision
/// and the ray's propagation, since a ray casting request allows up to N collisions before stopping.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Collision<T, P> {
    pub position: P,
    pub distance: f32,
    pub previous_position: P,
    pub pierced: bool,
    _marker: PhantomData<T>,
}

impl<T, P> Default for RayPropagation<T, P> {
    fn default() -> Self {
        Self {
            collisions: VecDeque::new(),
            area_of_interest: VecDeque::new(),
        }
    }
}

impl<T, P> RayPropagation<T, P> {
    pub fn new(collisions: VecDeque<Collision<T, P>>, area_of_interest: VecDeque<P>) -> Self {
        Self {
            collisions,
            area_of_interest,
        }
    }
}

impl<T, P: Point> RayPropagation<T, P> {
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
