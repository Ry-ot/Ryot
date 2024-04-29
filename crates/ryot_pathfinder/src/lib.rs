//! `ryot_pathfinder` is a high-performance, asynchronous pathfinding library
//! tailored for 2D environments within the Bevy game engine. It offers flexible,
//! efficient pathfinding capabilities designed to be easily integrated with Bevy's ECS,
//! supporting both synchronous and asynchronous execution. The crate includes a variety
//! of tools to define and handle pathfinding tasks for game entities, leveraging custom
//! components and systems for seamless operations in potentially dense 2D grids.
pub mod components;
pub mod pathable;
pub mod systems;
mod two_d;

#[cfg(feature = "ryot_tiled")]
pub mod tiled;

pub mod prelude {
    pub use crate::{
        components::{Path, PathFindingQuery},
        pathable::{Pathable, PathableApp},
        systems::PathFindingSystems,
        two_d::{find_path_2d, weighted_neighbors_2d_generator},
    };
}
