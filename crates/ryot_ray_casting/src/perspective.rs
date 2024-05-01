//! This crate provides functionality for managing and processing perspectives and visibility
//! of entities in a game environment. Perspectives are defined by sets of view points that
//! determine what an entity can see, based on tile positions and other spatial considerations.
use crate::prelude::*;
use bevy_math::bounding::{Aabb3d, RayCast3d};
use derive_more::{Deref, DerefMut};
use ryot_tiled::prelude::*;

/// A group of multiple traversals representing all the possible trajectories from a single point,
/// determining what can be reached from that point. Reachable is an abstract concept that depends
/// on the context of the game and the specific traversal logic (e.g. vision, path, etc).
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct Perspective(Vec<(RayCast3d, Vec<TilePosition>)>);

impl Perspective {
    pub fn new(traversals: Vec<(RayCast3d, Vec<TilePosition>)>) -> Self {
        Self(traversals)
    }

    /// Gets the intersections filtered by the provided condition across all view points in the
    /// perspective. This represents calculating what's visible from the entire perspective.
    pub fn get_intersections(self) -> Vec<Vec<TilePosition>> {
        self.get_intersections_with(|p| (*p).into())
    }

    /// Similar to `get_intersections`, but allows specifying a custom AABB transformer
    /// function. This can be used to apply custom filters or transformations to the intersections.
    pub fn get_intersections_with(
        self,
        aabb_transformer: impl Fn(&TilePosition) -> Aabb3d + Copy,
    ) -> Vec<Vec<TilePosition>> {
        self.0
            .into_iter()
            .map(|(ray_cast, target_area)| {
                target_area
                    .into_iter()
                    .filter_map(|pos| {
                        ray_cast.aabb_intersection_at(&aabb_transformer(&pos))?;
                        Some(pos)
                    })
                    .collect()
            })
            .collect()
    }
}

impl<T: Copy + Into<RadialArea>> From<&T> for RadialArea {
    fn from(element: &T) -> RadialArea {
        (*element).into()
    }
}
