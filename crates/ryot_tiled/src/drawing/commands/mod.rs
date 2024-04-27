//! This module contains the commands that manipulate the visual representation of the map.
//! The commands are used to load, update and delete the tiles, and are also set in a way that
//! they can be undone and redone.
//!
//! Commands are an efficient way to perform a large number of operations on the ECS in a single
//! frame. They are used to perform operations on the ECS in a way that is more efficient than
//! using the ECS API directly.
//!
//! However, commands are not a good fit for all, they have full access to the World and can
//! be quite convoluted to use if we abuse them.
//!
//! To avoid having god-like commands responsible for everything, we use the command pattern to
//! trigger the drawing flow, and then we use the ECS API to perform the operations.
use crate::prelude::*;
use bevy_reflect::Reflect;
use bevy_render::prelude::*;
use bevy_utils::*;
use ryot_core::prelude::*;

mod update;
pub use update::*;

/// A struct that holds the state of a command, used to keep track of the commands that were
/// applied and persisted. This is used by the ECS systems to manipulate the entities that
/// were triggered by a command in a more distributed and efficient way.
///
/// Applied means that the command was already applied to the ECS, meaning that the entities
/// were created, updated or deleted and the necessary states and resources were updated.
///
/// Persisted means that that the effects of the command were translated to the persistence
/// layer, meaning that the changes were saved to the storage, like a database or similar.
#[derive(Eq, PartialEq, Default, Clone, Debug, Copy, Reflect)]
pub struct CommandState {
    pub applied: bool,
    pub persisted: bool,
}

impl CommandState {
    pub fn apply(mut self) -> Self {
        self.applied = true;
        self
    }

    pub fn persist(mut self) -> Self {
        self.persisted = true;
        self
    }
}

/// An alternative version of DrawingBundle that holds the drawing information in a more
/// flexible way, so that it can be used in the commands and systems that manipulate the
/// drawing entities. Differently from DrawingBundle, this type is not a bundle, so it doesn't
/// get added to the ECS directly.
///
/// It has an optional FrameGroup, so that it can be used to represent the old state
/// of the entity, when reverting a command. An empty (objectId, group) means that the entity
/// is freshly created or should be deleted, depending on the context used.
pub type DrawingInfo = (
    TilePosition,
    Layer,
    Visibility,
    Option<(ContentId, FrameGroup)>,
);

impl From<DrawingBundle> for DrawingInfo {
    fn from(bundle: DrawingBundle) -> Self {
        (
            bundle.tile_pos,
            bundle.layer,
            bundle.visibility,
            Some((bundle.object_id, bundle.frame_group)),
        )
    }
}

impl From<TiledContentBundle> for DrawingInfo {
    fn from(bundle: TiledContentBundle) -> Self {
        (
            bundle.position,
            bundle.layer,
            Visibility::Visible,
            Some((bundle.object_id, default())),
        )
    }
}
