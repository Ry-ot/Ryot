use crate::prelude::*;
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use ryot_core::prelude::Navigable;
use ryot_utils::prelude::*;

/// Represents an App that can add one or more `Trajectory` to its systems.
/// Requires the `SimpleCache<RadialArea, Vec<Vec<P>>>` resource to be initialized.
pub trait TrajectoryApp {
    fn add_trajectory<
        Marker: Copy + Send + Sync + 'static,
        P: TrajectoryPoint,
        N: Navigable + Copy + Default,
    >(
        &mut self,
    ) -> &mut Self;
}

impl TrajectoryApp for App {
    fn add_trajectory<
        Marker: Copy + Send + Sync + 'static,
        P: TrajectoryPoint,
        N: Navigable + Copy + Default,
    >(
        &mut self,
    ) -> &mut Self {
        self.init_resource_once::<Cache<P, N>>()
            .init_resource::<SimpleCache<RadialArea<P>, Vec<Vec<P>>>>()
            .add_systems(
                Update,
                (
                    update_intersection_cache::<Marker, P>.in_set(CacheSystems::UpdateCache),
                    process_trajectories::<Marker, P, N>
                        .in_set(TrajectorySystems::ProcessTrajectories)
                        .after(CacheSystems::UpdateCache),
                    share_trajectories::<Marker, P>.in_set(TrajectorySystems::ProcessTrajectories),
                )
                    .chain(),
            )
    }
}
