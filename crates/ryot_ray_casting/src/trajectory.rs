use crate::prelude::*;
use crate::systems::share_trajectories;
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use bevy_math::bounding::Aabb3d;
use bevy_reflect::Reflect;
use bevy_utils::HashSet;
use ryot_core::prelude::{Navigable, Point};
use ryot_tiled::prelude::TilePosition;
use ryot_utils::prelude::*;
use std::marker::PhantomData;

/// Represents an App that can add one or more `Trajectory` to its systems.
/// Requires the `SimpleCache<RadialArea, Vec<Vec<TilePosition>>>` resource to be initialized.
pub trait TrajectoryApp {
    fn add_trajectory<
        P: Point + Into<Aabb3d> + Send + Sync + 'static,
        T: Trajectory<Position = P> + Clone,
        N: Navigable + Copy + Default,
    >(
        &mut self,
    ) -> &mut Self;
}

impl TrajectoryApp for App {
    fn add_trajectory<
        P: Point + Into<Aabb3d> + Send + Sync + 'static,
        T: Trajectory<Position = P> + Clone,
        N: Navigable + Copy + Default,
    >(
        &mut self,
    ) -> &mut Self {
        self.init_resource_once::<Cache<P, N>>()
            .init_resource::<SimpleCache<RadialArea<P>, Vec<Vec<P>>>>()
            .add_systems(
                Update,
                (
                    update_intersection_cache::<P, T>.in_set(CacheSystems::UpdateCache),
                    process_trajectories::<P, T, N>
                        .in_set(PerspectiveSystems::CalculatePerspectives)
                        .after(CacheSystems::UpdateCache),
                    share_trajectories::<T>.in_set(PerspectiveSystems::CalculatePerspectives),
                )
                    .chain(),
            )
    }
}

/// Represents entities that can provide a `RadialArea` for perspective calculation.
///
/// This trait facilitates the generation of a radial area based on an entity's current state or
/// position. It is used to abstract the way different entities determine their perspective in the
/// world. The `meets_condition` method allows for additional checks on environmental or
/// entity-specific conditions that may affect whether a position is considered valid for certain
/// operations within the trajectory area, like visibility checks or interactions.
pub trait Trajectory: Component + Send + Sync + 'static {
    type Position;

    /// Generates a `RadialArea` based on the entity's current state or position.
    ///
    /// Implementations should provide the logic to construct an area that accurately reflects
    /// the entity's perspective in the game world, considering factors like position and orientation.
    fn get_area(&self) -> RadialArea<Self::Position>;

    /// Evaluates if specific conditions are met based on the provided tile flags and position.
    ///
    /// This method should be used to check conditions related to the entity's interaction with the
    /// environment, such as obstructions, visibility, or other criteria defined by `Flag`.
    fn meets_condition(&self, flags: &impl Navigable, _: &Self::Position) -> bool {
        flags.is_walkable()
    }
}

/// Represents a collection of tile positions of interest for an entity, based on a trajectory T.
///
/// This component is used to track and share tile positions that an entity, through its specific
/// trajectory (defined by the `V` trait), deems significant. These positions could represent areas
/// the entity can see, move towards, or interact with in some capacity.
///
/// The `shared_with` field allows these positions to be shared with other entities, enabling
/// collaborative or team-based mechanics where multiple entities can benefit from shared traversals
/// or strategic information.
///
/// This struct facilitates diverse gameplay mechanics by allowing entities to dynamically respond
/// to and share critical spatial information within the game world.
#[derive(Clone, Component, Debug, Reflect)]
pub struct InterestPositions<T: Trajectory> {
    #[reflect(ignore)]
    pub positions: Vec<T::Position>,
    _phantom: PhantomData<T>,
}

impl<T: Trajectory> Default for InterestPositions<T> {
    fn default() -> Self {
        Self {
            positions: Vec::default(),
            _phantom: PhantomData::<T>,
        }
    }
}

impl<T: Trajectory> InterestPositions<T> {
    pub fn new(positions: Vec<T::Position>) -> Self {
        Self {
            positions,
            _phantom: PhantomData::<T>,
        }
    }
}

#[derive(Clone, Component, Debug, Reflect)]
pub struct ShareTrajectoryWith<T: Trajectory> {
    #[reflect(ignore)]
    pub shared_with: HashSet<Entity>,
    _phantom: PhantomData<T>,
}

impl<T: Trajectory> Default for ShareTrajectoryWith<T> {
    fn default() -> Self {
        Self {
            shared_with: HashSet::default(),
            _phantom: PhantomData,
        }
    }
}

impl<T: Trajectory> ShareTrajectoryWith<T> {
    /// Allows sharing visibility with additional entities. This can be used in team-based or
    /// cooperative scenarios, where visibility information should be shared among allies.
    pub fn share_with(mut self, entities: Vec<Entity>) -> Self {
        self.shared_with.extend(entities);
        self
    }
}

/// An implementation of trajectory used to define a what is visible for different contexts.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub struct VisibleTrajectory<T>(pub RadialArea<TilePosition>, PhantomData<T>);

impl<T> VisibleTrajectory<T> {
    pub fn new(area: RadialArea<TilePosition>) -> Self {
        Self(area, PhantomData::<T>)
    }
}

impl<T: Copy + Send + Sync + 'static> Trajectory for VisibleTrajectory<T> {
    type Position = TilePosition;

    fn get_area(&self) -> RadialArea<Self::Position> {
        (*self).into()
    }

    fn meets_condition(&self, flags: &impl Navigable, _: &Self::Position) -> bool {
        !flags.blocks_sight()
    }
}

impl<T> From<VisibleTrajectory<T>> for RadialArea<TilePosition> {
    fn from(visible: VisibleTrajectory<T>) -> Self {
        visible.0
    }
}

/// An implementation of trajectory used to define what is walkable for different contexts.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub struct WalkableTrajectory<T>(pub RadialArea<TilePosition>, PhantomData<T>);

impl<T> WalkableTrajectory<T> {
    pub fn new(area: RadialArea<TilePosition>) -> Self {
        Self(area, PhantomData::<T>)
    }
}

impl<T: Copy + Send + Sync + 'static> Trajectory for WalkableTrajectory<T> {
    type Position = TilePosition;

    fn get_area(&self) -> RadialArea<Self::Position> {
        (*self).into()
    }
}

impl<T> From<WalkableTrajectory<T>> for RadialArea<TilePosition> {
    fn from(path: WalkableTrajectory<T>) -> Self {
        path.0
    }
}
