//! This crate provides functionality for managing and processing perspectives and visibility
//! of entities in a game environment. Perspectives are defined by sets of view points that
//! determine what an entity can see, based on tile positions and other spatial considerations.
pub mod trajectory;

pub mod traversal;

pub mod systems;

pub mod perspective;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::{
        perspective::Perspective,
        systems::{process_perspectives, update_intersection_cache, PerspectiveSystems},
        trajectory::{
            InterestPositions, Trajectory, TrajectoryApp, VisibleTrajectory, WalkableTrajectory,
        },
        traversal::{RadialArea, Traversal},
    };
}
