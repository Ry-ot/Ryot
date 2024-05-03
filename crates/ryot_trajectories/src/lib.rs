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
mod interest_positions;

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
        interest_positions::InterestPositions,
        perspective::Perspective,
        radial_area::RadialArea,
        systems::{
            process_trajectories, share_trajectories, update_intersection_cache, TrajectorySystems,
        },
        trajectory::{visible_trajectory, walkable_trajectory, ShareTrajectoryWith, Trajectory},
        TrajectoryPoint,
    };
}

/*

TO IMPROVE

- Configurable:
    - shareable or not
    - different type of perspective search (e.g. in_area, get N hits, get closets, etc.)
    - triggered search like path finding instead of always performing

*/
