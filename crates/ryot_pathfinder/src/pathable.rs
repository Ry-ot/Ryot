use crate::prelude::*;
use crate::systems::{handle_path_finding_tasks, trigger_path_finding_tasks};
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use ryot_core::prelude::Navigable;
use ryot_utils::cache::Cache;
use std::hash::Hash;

/// Enables Bevy apps to integrate pathfinding by adding required systems and resources.
/// Requires the `Cache<P, F>` resource to be initialized.
pub trait PathableApp {
    fn add_pathable<P: Pathable + Component, N: Navigable + Copy + Default>(&mut self)
        -> &mut Self;
}

/// Defines capabilities for elements that can engage in pathfinding, including
/// generating coordinates and calculating paths based on specified criteria.
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
