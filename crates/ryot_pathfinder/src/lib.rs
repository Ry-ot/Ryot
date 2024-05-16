#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod commands;
pub mod components;
pub mod pathable;
pub mod systems;
mod two_d;

#[cfg(feature = "stubs")]
pub mod stubs;

pub mod prelude {
    pub use crate::{
        commands::AmendPathCommand,
        components::{Path, PathFindingQuery},
        pathable::{Pathable, PathableApp},
        systems::PathFindingSystems,
        two_d::{find_path_2d, weighted_neighbors_2d_generator},
    };
}
