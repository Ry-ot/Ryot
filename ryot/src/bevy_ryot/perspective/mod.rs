//! This module provides functionality for managing and processing perspectives and visibility
//! of entities in a game environment. Perspectives are defined by sets of view points that
//! determine what an entity can see, based on tile positions and other spatial considerations.
use std::marker::PhantomData;

use bevy::math::bounding::Aabb3d;
use bevy::utils::HashSet;

use crate::bevy_ryot::Cache;
use crate::position::TilePosition;

mod conditional_view_point;
pub use conditional_view_point::*;

mod sight;
pub use sight::*;

mod systems;
pub use systems::*;

mod view_point;
pub use view_point::*;

/// Represents a perspective, which consists of multiple view points. A view point is essentially
/// a perspective from a single point, determining what can be seen from that point. A perspective
/// combines multiple view points to form a comprehensive view of what an entity can see.
#[derive(Debug, Clone, Default)]
pub struct Perspective {
    pub view_points: Vec<ViewPoint>,
}

impl Perspective {
    pub fn new(view_points: Vec<ViewPoint>) -> Self {
        Self { view_points }
    }

    /// Gets the intersections filtered by the provided condition across all view points in the
    /// perspective. This represents calculating what's visible from the entire perspective.
    pub fn get_filtered_intersections(self) -> Vec<Vec<TilePosition>> {
        self.view_points
            .into_iter()
            .map(|view_point| view_point.get_filtered_intersections())
            .collect()
    }

    /// Similar to `get_filtered_intersections`, but allows specifying a custom AABB transformer
    /// function. This can be used to apply custom filters or transformations to the intersections.
    pub fn get_filtered_intersections_with(
        self,
        aabb_transformer: impl Fn(&TilePosition) -> Aabb3d + Copy,
    ) -> Vec<Vec<TilePosition>> {
        self.view_points
            .into_iter()
            .map(|view_point| view_point.get_filtered_intersections_with(aabb_transformer))
            .collect()
    }
}

impl<T: Copy + Into<RadialViewPoint>> From<&T> for RadialViewPoint {
    fn from(element: &T) -> RadialViewPoint {
        (*element).into()
    }
}

#[cfg(test)]
mod tests;
