use bevy::app::{App, Update};
use bevy::prelude::*;

use crate::bevy_ryot::perspective::*;
use crate::bevy_ryot::tile_flags::TileFlags;
use crate::bevy_ryot::CacheSystems;
use crate::position::TilePosition;

/// Represents an App that can add one or more `ConditionalViewPoint` to its systems.
/// Requires the `Cache<RadialViewPoint, Vec<Vec<TilePosition>>>` resource to be initialized.
pub trait ConditionalViewPointApp {
    fn add_view_point<V: ConditionalViewPoint>(&mut self) -> &mut Self;
}

impl ConditionalViewPointApp for App {
    fn add_view_point<V: ConditionalViewPoint>(&mut self) -> &mut Self {
        self.init_resource::<Cache<RadialViewPoint, Vec<Vec<TilePosition>>>>()
            .add_systems(
                Update,
                (
                    update_intersection_cache::<V>.in_set(CacheSystems::UpdateCache),
                    process_perspectives::<V>
                        .in_set(PerspectiveSystems::CalculatePerspectives)
                        .after(CacheSystems::UpdateCache),
                )
                    .chain(),
            )
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
        flags.walkable
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
