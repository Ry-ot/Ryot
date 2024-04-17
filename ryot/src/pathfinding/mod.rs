use crate::pathfinding::systems::*;
use bevy::prelude::*;
use std::hash::Hash;
use std::time::Duration;

pub mod components;
pub mod systems;

/// Represents an App that can add one or more `Pathable` to its systems.
/// Requires the `Cache<P, TileFlags>` resource to be initialized.
pub trait PathableApp {
    fn add_pathable<P: Pathable + Component>(&mut self) -> &mut Self;
}

/// Represents an element that can be used in path finding calculations.
pub trait Pathable: Eq + Hash + Copy + Clone + Sync + Send + 'static {
    /// Returns the path from the current element to the target element, taking into
    /// account the custom walkable function (if it's possible to walk through that element or not)
    /// and a timeout (maximum time to calculate the path).
    fn path_to<F: Fn(&Self) -> bool>(
        &self,
        to: Self,
        is_walkable: F,
        timeout: Option<Duration>,
    ) -> Option<(Vec<Self>, u32)>;

    /// Returns the positions of the tiles that are directly adjacent to the current element
    /// and its weight (cost) to reach them.
    fn get_weighted_neighbors<F: Fn(&Self) -> bool + ?Sized>(
        &self,
        is_walkable: &F,
    ) -> Vec<(Self, u32)>;
}

impl PathableApp for App {
    fn add_pathable<P: Pathable + Component>(&mut self) -> &mut Self {
        self.add_systems(
            Update,
            (
                trigger_path_finding_tasks::<P>.in_set(PathFindingSystems::TriggerTask),
                handle_path_finding_tasks::<P>
                    .in_set(PathFindingSystems::ExecuteTask)
                    .after(PathFindingSystems::TriggerTask),
            ),
        )
    }
}
