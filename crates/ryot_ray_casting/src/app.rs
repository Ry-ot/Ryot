use crate::prelude::*;
use crate::systems::{remove_stale_requests, remove_stale_results};
use bevy_app::{App, PostUpdate, Update};
use bevy_ecs::prelude::*;
use ryot_core::prelude::Navigable;
use ryot_utils::prelude::*;

/// Represents an App that can add one or more `RayCasting<T, P>` to its systems.
/// Requires the `SimpleCache<RadialArea, Vec<Vec<P>>>` resource to be initialized.
pub trait RayCastingApp {
    fn add_ray_casting<
        Marker: Copy + Send + Sync + 'static,
        P: RayCastingPoint + Component,
        N: Navigable + Copy + Default,
    >(
        &mut self,
    ) -> &mut Self;
}

impl RayCastingApp for App {
    fn add_ray_casting<
        Marker: Copy + Send + Sync + 'static,
        P: RayCastingPoint + Component,
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
                    process_ray_casting::<Marker, P, N>
                        .in_set(RayCastingSystems::Process)
                        .after(CacheSystems::UpdateCache),
                    share_results::<Marker, P>.in_set(RayCastingSystems::Process),
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                (
                    remove_stale_results::<Marker, P>,
                    remove_stale_requests::<Marker, P>,
                )
                    .in_set(RayCastingSystems::CleanUp)
                    .after(RayCastingSystems::Process)
                    .chain(),
            )
    }
}
