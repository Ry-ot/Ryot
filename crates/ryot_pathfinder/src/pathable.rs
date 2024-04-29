//! Pathable is the bare-bone concept of this pathfinding library, representing a position in the world
//! that can be used to calculate paths between two points. It's represented by a trait that knows
//! how to generate a pathable position based on (x, y, z) coordinates, which coordinates represent
//! this pathable position, and how to calculate the path to another pathable position.
//!
//! There is also a well-defined way to integrate the pathfinding capabilities provided by this
//! library into a Bevy app, by adding the necessary systems and resources to the app. This is done
//! by using the `add_pathable` method, which initializes the required resources and systems for a
//! pair of Pathable and Navigable types.
use crate::prelude::*;
use crate::systems::{handle_path_finding_tasks, trigger_path_finding_tasks};
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use ryot_core::prelude::Navigable;
use ryot_utils::cache::Cache;
use std::hash::Hash;

/// Trait for elements that can engage in pathfinding, providing the necessary methods to calculate
/// paths between two points. You can implement this trait for your own types, or if you need a
/// solution tailored for 2D tiled games, you can use the `TilePosition` struct provided by the
/// `ryot_tiled` module. Defines capabilities for elements that can engage in pathfinding. Contains
/// a default 2d focused implementation for the `path_to` method, which can be overridden based on
/// the user's needs.
pub trait Pathable: Eq + Hash + Copy + Clone + Sync + Send + 'static {
    fn generate(x: i32, y: i32, z: i32) -> Self;
    fn coordinates(&self) -> (i32, i32, i32);
    fn path_to<F: Fn(&Self) -> bool>(
        &self,
        query: &PathFindingQuery<Self>,
        validator: F,
    ) -> Option<(Vec<Self>, u32)> {
        // This crate is mostly focused on 2D path finding, so we'll provide a default implementation
        // for 2D, which can be overridden by the user if they need 3D path finding or other scenarios.
        find_path_2d(self, query, &validator, &weighted_neighbors_2d_generator)
    }
}

/// A trait that extends the Bevy `App` with the ability to add pathable elements to the app,
/// facilitating the integration of pathfinding capabilities into a Bevy app. This trait provides
/// a method to add pathable elements to the app, initializing the necessary resources and systems
/// for a pair of Pathable and Navigable types.
pub trait PathableApp {
    fn add_pathable<P: Pathable + Component, N: Navigable + Copy + Default>(&mut self)
        -> &mut Self;
}

impl PathableApp for App {
    fn add_pathable<P: Pathable + Component, N: Navigable + Copy + Default>(
        &mut self,
    ) -> &mut Self {
        self.init_resource::<Cache<P, N>>().add_systems(
            Update,
            (
                trigger_path_finding_tasks::<P, N>.in_set(PathFindingSystems::TriggerTask),
                handle_path_finding_tasks::<P>
                    .in_set(PathFindingSystems::ExecuteTask)
                    .after(PathFindingSystems::TriggerTask),
            ),
        )
    }
}
