use bevy_ecs::prelude::*;
use ryot_core::game::Point;
use std::collections::VecDeque;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection<P, T> {
    pub position: P,
    pub distance: f32,
    pub previous_position: P,
    pub pierced: bool,
    _marker: PhantomData<T>,
}

impl<P, T> Intersection<P, T> {
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
pub struct Intersections<T, P> {
    pub hits: VecDeque<Intersection<P, T>>,
    pub area_of_interest: VecDeque<P>,
}

impl<T, P> Default for Intersections<T, P> {
    fn default() -> Self {
        Self {
            hits: VecDeque::new(),
            area_of_interest: VecDeque::new(),
        }
    }
}

impl<T, P> Intersections<T, P> {
    pub fn new(hits: VecDeque<Intersection<P, T>>, area_of_interest: VecDeque<P>) -> Self {
        Self {
            hits,
            area_of_interest,
        }
    }
}

impl<T, P: Point> Intersections<T, P> {
    pub fn get_hits_last_positions(&self) -> VecDeque<P> {
        let mut last_positions = VecDeque::new();

        for hit in self.hits.iter().rev() {
            if hit.pierced {
                continue;
            }

            if !self
                .hits
                .iter()
                .any(|hit| hit.position == hit.previous_position)
            {
                last_positions.push_back(hit.previous_position);
            }
        }

        last_positions
    }
}
