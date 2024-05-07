#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(trait_alias)]
use bevy_math::bounding::Aabb3d;
use ryot_core::prelude::Point;
use ryot_utils::prelude::*;

mod app;
mod propagation;
mod request;

pub mod perspective;
pub mod radial_area;
pub mod systems;

#[cfg(test)]
mod tests;

#[cfg(feature = "stubs")]
pub mod stubs;

pub trait RayCastingPoint = Point + Into<Aabb3d> + ThreadSafe;

pub mod prelude {
    pub use crate::{
        app::RayCastingApp,
        perspective::Perspective,
        propagation::{Collision, RayPropagation},
        radial_area::RadialArea,
        request::{visible_ray_casting, walkable_ray_casting, ExecutionType, RayCasting},
        systems::{
            process_ray_casting, remove_stale_requests, remove_stale_results, share_results,
            update_intersection_cache, RayCastingSystems,
        },
        RayCastingPoint,
    };
}
