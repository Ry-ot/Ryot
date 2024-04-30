//! Pathable is the bare-bone concept of this pathfinding library, representing a position in the world
//! that can be used to calculate paths between two points. It extends the Point trait, a representation
//! of a spatial point within Ryot ecosystem, and encapsulates the calculation of the path between two points.
//!
//! There is also a well-defined way to integrate the pathfinding capabilities provided by this
//! library into a Bevy app, by adding the necessary systems and resources to the app. This is done
//! by using the `add_pathable` method, which initializes the required resources and systems for a
//! pair of Pathable and Navigable types.
use crate::prelude::*;
use crate::systems::{handle_path_finding_tasks, trigger_path_finding_tasks};
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use ryot_core::prelude::*;
use ryot_utils::cache::Cache;

/// Trait for elements that can engage in pathfinding, providing a method to determine the path
/// between two Points. This trait depends on Point, which is a trait that represents a position in
/// the world. Contains a default 2d focused implementation for the `path_to` method, which can be
/// overridden to provide pathfinding to other scenarios.
pub trait Pathable: Point + Sync + Send + 'static {
    fn path_to(
        &self,
        query: &PathFindingQuery<Self>,
        validator: impl Fn(&Self) -> bool,
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
