#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(trait_alias)]
use bevy_math::bounding::Aabb3d;
use ryot_core::prelude::Point;

mod app;
mod collision;
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
        collision::{Collision, TrajectoryResult},
        perspective::Perspective,
        radial_area::RadialArea,
        request::ExecutionType,
        systems::{
            process_trajectories, remove_stale_results, remove_stale_trajectories, share_results,
            update_intersection_cache, TrajectorySystems,
        },
        trajectory::{visible_trajectory, walkable_trajectory, TrajectoryRequest},
        TrajectoryPoint,
    };
}
