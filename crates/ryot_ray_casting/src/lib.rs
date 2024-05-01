//! `ryot_ray_casting`
//!
//! Provides ray casting capabilities specifically for Bevy 2D, essential for implementing
//! line-of-sight features and other interactive game mechanics.
//!
//! It includes functionalities for managing and processing perspectives and visibility
//! of entities in a game environment. Perspectives are defined by sets of view points that
//! determine what an entity can see, based on it spatial position and other considerations.
pub mod trajectory;

pub mod traversal;

pub mod systems;

pub mod perspective;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::{
        perspective::Perspective,
        systems::{
            process_trajectories, share_trajectories, update_intersection_cache, PerspectiveSystems,
        },
        trajectory::{
            InterestPositions, ShareTrajectoryWith, Trajectory, TrajectoryApp, VisibleTrajectory,
            WalkableTrajectory,
        },
        traversal::RadialArea,
    };
}
