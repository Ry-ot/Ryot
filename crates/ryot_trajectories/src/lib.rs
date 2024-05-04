//! `ryot_trajectories`
//!
//! Provides trajectory capabilities specifically for Bevy 2D, essential for implementing
//! line-of-sight features and other interactive game mechanics.
//!
//! It includes functionalities for managing and processing perspectives and visibility
//! of entities in a game environment. Perspectives are defined by sets of view points that
//! determine what an entity can see, based on it spatial position and other considerations.
#![feature(trait_alias)]
use bevy_math::bounding::Aabb3d;
use ryot_core::game::Point;

mod app;
mod intersection;
mod request;

pub mod perspective;
pub mod radial_area;
pub mod systems;
pub mod trajectory;

#[cfg(test)]
mod tests;

#[cfg(feature = "stubs")]
pub mod stubs;

pub trait TrajectoryPoint = Point + Into<Aabb3d> + Send + Sync + 'static;

pub mod prelude {
    pub use crate::{
        app::TrajectoryApp,
        intersection::{Intersection, Intersections},
        perspective::Perspective,
        radial_area::RadialArea,
        request::ExecutionType,
        systems::{
            process_trajectories, remove_orphan_intersections, remove_stale_trajectories,
            share_results, update_intersection_cache, TrajectorySystems,
        },
        trajectory::{visible_trajectory, walkable_trajectory, Trajectory},
        TrajectoryPoint,
    };
}
