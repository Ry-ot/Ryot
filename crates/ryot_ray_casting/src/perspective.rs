//! This crate provides functionality for managing and processing perspectives and visibility
//! of entities in a game environment. Perspectives are defined by sets of view points that
//! determine what an entity can see, based on a spatial point and other considerations.
use crate::prelude::*;
use bevy_math::bounding::{Aabb3d, RayCast3d};
use derive_more::{Deref, DerefMut};

/// A perspective slices a given observable area into rays and target areas through which the
/// rays are cast. This allows for the calculation of intersections between the ray and the
/// target area, which can be used to determine the propagation of the ray given the conditions
/// of the ray casting request.
///
/// The intersection calculation is a costly operation. To mitigate this, we use the radial
/// description of the perspective, [RadialArea], to cache the intersections for each perspective,
/// so it is done only when necessary and cached based on the representation of that perspective.
///
/// Only aabb intersection is currently supported as the calculation method.
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

impl<T: Copy + Into<RadialArea<P>>, P> From<&T> for RadialArea<P> {
    fn from(element: &T) -> RadialArea<P> {
        (*element).into()
    }
}
