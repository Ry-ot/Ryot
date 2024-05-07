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
use ryot_utils::prelude::*;

/// Trait for elements that can engage in pathfinding, providing a method to determine the path
/// between two Points and if the current Pathable can be navigated based on a given Navigable.
/// This trait depends on Point, which is a trait that represents a position in the world.
pub trait Pathable: Point + ThreadSafe {
    /// Calculates the path between two points, based on the provided query and a validator function
    /// that determines if a point is pathable. The path is returned as a vector of points and the
    /// total cost of the path.
    /// The default implementation is focused on 2D pathfinding, which can be overridden for other
    /// scenarios, like 3D pathfinding.
    fn path_to(
        &self,
        query: &PathFindingQuery<Self>,
        validator: impl Fn(&Self) -> bool,
    ) -> Option<(Vec<Self>, u32)> {
        find_path_2d(self, query, &validator, &weighted_neighbors_2d_generator)
    }

    /// Determines if a Pathable can be navigated, based on the provided Navigable element.
    /// This method is used to check if one is allowed to navigate through the pathable in the
    /// context of the game environment.
    /// The default implementation returns true if the Navigable element is walkable, or true if
    /// no Navigable element is provided.
    fn can_be_navigated<N: Navigable>(&self, nav: Option<&N>) -> bool {
        nav.map_or(true, |nav| nav.is_walkable())
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
        self.init_resource_once::<Cache<P, N>>().add_systems(
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
