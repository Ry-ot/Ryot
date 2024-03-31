//! This module provides functionality for managing and processing perspectives and visibility
//! of entities in a game environment. Perspectives are defined by sets of view points that
//! determine what an entity can see, based on tile positions and other spatial considerations.
use std::marker::PhantomData;

use bevy::math::bounding::Aabb3d;
use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::bevy_ryot::{AppearanceAssets, Cache};
use crate::position::TilePosition;
use crate::prelude::{tile_flags::*, CacheSystems, OptionalPlugin};

mod view_point;
pub use view_point::*;

mod systems;
pub use systems::*;

/// `PerspectivePlugin` is responsible for setting up the system infrastructure required to manage
/// perspectives and their associated data within the game. It initializes necessary resources and
/// configures system execution order to ensure that perspectives are calculated based on the latest
/// game state.
pub struct PerspectivePlugin<V: ConditionalViewPoint, C: AppearanceAssets>(
    PhantomData<V>,
    PhantomData<C>,
);

impl<V: ConditionalViewPoint, C: AppearanceAssets> Default for PerspectivePlugin<V, C> {
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}
impl<V: ConditionalViewPoint, C: AppearanceAssets> Plugin for PerspectivePlugin<V, C> {
    fn build(&self, app: &mut App) {
        app.add_optional_plugin(TileFlagPlugin::<C>::default());

        app.init_resource::<Cache<RadialViewPoint, Vec<Vec<TilePosition>>>>()
            .add_systems(
                Update,
                (
                    update_intersection_cache::<V>.in_set(CacheSystems::UpdateCache),
                    process_perspectives::<V>
                        .in_set(PerspectiveSystems::CalculatePerspectives)
                        .after(CacheSystems::UpdateCache),
                )
                    .chain(),
            );
    }
}

/// Represents a perspective, which consists of multiple view points. A view point is essentially
/// a perspective from a single point, determining what can be seen from that point. A perspective
/// combines multiple view points to form a comprehensive view of what an entity can see.
#[derive(Debug, Clone, Default)]
pub struct Perspective {
    pub view_points: Vec<ViewPoint>,
}

impl Perspective {
    pub fn new(view_points: Vec<ViewPoint>) -> Self {
        Self { view_points }
    }

    /// Gets the intersections filtered by the provided condition across all view points in the
    /// perspective. This represents calculating what's visible from the entire perspective.
    pub fn get_filtered_intersections(self) -> Vec<Vec<TilePosition>> {
        self.view_points
            .into_iter()
            .map(|view_point| view_point.get_filtered_intersections())
            .collect()
    }

    /// Similar to `get_filtered_intersections`, but allows specifying a custom AABB transformer
    /// function. This can be used to apply custom filters or transformations to the intersections.
    pub fn get_filtered_intersections_with(
        self,
        aabb_transformer: impl Fn(&TilePosition) -> Aabb3d + Copy,
    ) -> Vec<Vec<TilePosition>> {
        self.view_points
            .into_iter()
            .map(|view_point| view_point.get_filtered_intersections_with(aabb_transformer))
            .collect()
    }
}

/// Represents a collection of tile positions of interest for an entity, based on a specific viewpoint
/// scope V.
///
/// This component is used to track and share tile positions that an entity, through its specific
/// viewpoint (defined by the `V` trait), deems significant. These positions could represent areas
/// the entity can see, move towards, or interact with in some capacity.
///
/// The `shared_with` field allows these positions to be shared with other entities, enabling
/// collaborative or team-based mechanics where multiple entities can benefit from shared viewpoints
/// or strategic information.
///
/// This struct facilitates diverse gameplay mechanics by allowing entities to dynamically respond
/// to and share critical spatial information within the game world.
#[derive(Clone, Component, Debug, Reflect)]
#[reflect]
pub struct InterestPositions<V: ConditionalViewPoint> {
    #[reflect(ignore)]
    pub shared_with: HashSet<Entity>,
    #[reflect(ignore)]
    pub positions: Vec<TilePosition>,
    _phantom: PhantomData<V>,
}

impl<V: ConditionalViewPoint> Default for InterestPositions<V> {
    fn default() -> Self {
        Self {
            shared_with: HashSet::default(),
            positions: Vec::default(),
            _phantom: PhantomData::<V>,
        }
    }
}

impl<V: ConditionalViewPoint> InterestPositions<V> {
    /// Allows sharing visibility with additional entities. This can be used in team-based or
    /// cooperative scenarios, where visibility information should be shared among allies.
    pub fn share_with(mut self, entities: Vec<Entity>) -> Self {
        self.shared_with.extend(entities);
        self
    }
}

/// Represents entities that can provide a `RadialViewPoint` for perspective calculation.
///
/// This trait facilitates the generation of a viewpoint based on an entity's current state or
/// position. It is used to abstract the way different entities determine their perspective in the
/// world. The `meets_condition` method allows for additional checks on environmental or
/// entity-specific conditions that may affect whether a viewpoint is considered valid for certain
/// operations, like visibility checks or interactions.
pub trait ConditionalViewPoint: Component + Send + Sync + 'static {
    /// Generates a `RadialViewPoint` based on the entity's current state or position.
    ///
    /// Implementations should provide the logic to construct a viewpoint that accurately reflects
    /// the entity's perspective in the game world, considering factors like position and orientation.
    fn get_view_point(&self) -> RadialViewPoint;

    /// Evaluates if specific conditions are met based on the provided tile flags and position.
    ///
    /// This method should be used to check conditions related to the entity's interaction with the
    /// environment, such as obstructions, visibility, or other criteria defined by `TileFlags`.
    fn meets_condition(&self, flags: &TileFlags, _: &TilePosition) -> bool {
        flags.visible
    }
}

impl<T: Copy + Into<RadialViewPoint>> From<&T> for RadialViewPoint {
    fn from(element: &T) -> RadialViewPoint {
        (*element).into()
    }
}

#[cfg(test)]
mod tests;
