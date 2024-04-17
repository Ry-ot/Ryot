//! This module provides functionality for managing and processing perspectives and visibility
//! of entities in a game environment. Perspectives are defined by sets of view points that
//! determine what an entity can see, based on tile positions and other spatial considerations.
use std::marker::PhantomData;

use bevy::math::bounding::Aabb3d;

use crate::prelude::*;

mod trajectory;
pub use trajectory::*;

mod traversal;
pub use traversal::*;

mod systems;
pub use systems::*;

/// A group of multiple traversals representing all the possible trajectories from a single point,
/// determining what can be reached from that point. Reachable is an abstract concept that depends
/// on the context of the game and the specific traversal logic (e.g. vision, path, etc).
#[derive(Debug, Clone, Default)]
pub struct Perspective {
    pub traversals: Vec<Traversal>,
}

impl Perspective {
    pub fn new(traversals: Vec<Traversal>) -> Self {
        Self { traversals }
    }

    /// Gets the intersections filtered by the provided condition across all view points in the
    /// perspective. This represents calculating what's visible from the entire perspective.
    pub fn get_intersections(self) -> Vec<Vec<TilePosition>> {
        self.traversals
            .into_iter()
            .map(|traversal| traversal.get_intersections())
            .collect()
    }

    /// Similar to `get_intersections`, but allows specifying a custom AABB transformer
    /// function. This can be used to apply custom filters or transformations to the intersections.
    pub fn get_intersections_with(
        self,
        aabb_transformer: impl Fn(&TilePosition) -> Aabb3d + Copy,
    ) -> Vec<Vec<TilePosition>> {
        self.traversals
            .into_iter()
            .map(|traversal| traversal.get_intersections_with(aabb_transformer))
            .collect()
    }
}

impl<T: Copy + Into<RadialArea>> From<&T> for RadialArea {
    fn from(element: &T) -> RadialArea {
        (*element).into()
    }
}

#[cfg(test)]
mod tests;
