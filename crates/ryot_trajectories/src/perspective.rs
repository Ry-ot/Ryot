//! This crate provides functionality for managing and processing perspectives and visibility
//! of entities in a game environment. Perspectives are defined by sets of view points that
//! determine what an entity can see, based on a spatial point and other considerations.
use crate::prelude::*;
use bevy_math::bounding::{Aabb3d, RayCast3d};
use derive_more::{Deref, DerefMut};

/// A group of multiple traversals representing all the possible trajectories from a single point,
/// determining what can be reached from that point. Reachable is an abstract concept that depends
/// on the context of the game and the specific traversal logic (e.g. vision, path, etc).
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Perspective<P>(Vec<(RayCast3d, Vec<P>)>);

impl<P> Default for Perspective<P> {
    fn default() -> Self {
        Perspective(vec![])
    }
}

impl<P: RayCastingPoint> Perspective<P> {
    pub fn new(traversals: Vec<(RayCast3d, Vec<P>)>) -> Self {
        Self(traversals)
    }

    /// Gets the intersections filtered by the provided condition across all view points in the
    /// perspective. This represents calculating what's visible from the entire perspective.
    pub fn get_intersections(self) -> Vec<Vec<P>> {
        self.get_intersections_with(|p| (*p).into())
    }

    /// Similar to `get_intersections`, but allows specifying a custom AABB transformer
    /// function. This can be used to apply custom filters or transformations to the intersections.
    pub fn get_intersections_with(
        self,
        aabb_transformer: impl Fn(&P) -> Aabb3d + Copy,
    ) -> Vec<Vec<P>> {
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

impl<P, T: Copy + Into<RadialArea<P>>> From<&T> for RadialArea<P> {
    fn from(element: &T) -> RadialArea<P> {
        (*element).into()
    }
}
