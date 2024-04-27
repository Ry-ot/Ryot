//! `ryot_pathfinder`
//!
//! Specializes in providing pathfinding functionalities for Bevy 2D, enabling complex
//! navigation and movement logic essential for dynamic game environments.
pub mod components;
pub mod pathable;
pub mod systems;

#[cfg(feature = "ryot_tiled")]
pub mod tiled;

pub mod prelude {
    pub use crate::{
        components::{Path, PathFindingQuery},
        pathable::{Pathable, PathableApp},
        systems::PathFindingSystems,
    };
}

#[cfg(test)]
#[cfg(feature = "ryot_tiled")]
mod bench;
