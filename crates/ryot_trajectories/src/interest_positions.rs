use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use std::collections::VecDeque;
use std::marker::PhantomData;

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
pub struct InterestPositions<T, P> {
    #[reflect(ignore)]
    pub positions: VecDeque<P>,
    _phantom: PhantomData<T>,
}

impl<T, P> Default for InterestPositions<T, P> {
    fn default() -> Self {
        Self {
            positions: VecDeque::default(),
            _phantom: PhantomData::<T>,
        }
    }
}

impl<T, P> InterestPositions<T, P> {
    pub fn new(positions: VecDeque<P>) -> Self {
        Self {
            positions,
            _phantom: PhantomData::<T>,
        }
    }
}
