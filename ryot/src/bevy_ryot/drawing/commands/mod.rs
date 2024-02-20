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
use crate::bevy_ryot::drawing::DrawingBundle;
use crate::bevy_ryot::AppearanceDescriptor;
use crate::position::TilePosition;
use crate::Layer;
use bevy::prelude::*;

mod load;
pub use load::*;

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
    /// When loading from the persistence layer, we don't want to save the changes again,
    /// so we set the persisted flag to true, so that persistence is ignored during loading.
    pub fn from_load() -> Self {
        Self {
            applied: false,
            persisted: true,
        }
    }
}

/// A trait that represents a reversible command, that can be undone and redone.
/// Due to limitations from the bevy commands, the undo and redo methods needs to be
/// implemented in each command. We cannot make a generic implementation and simplify
/// this interface to only return the reversed version of the command.
///
/// Every command added to the command system in bevy needs to have known size and be a
/// concrete type, so we cannot simply use a trait object to dispatch the command.
///
/// That's why we pass Commands here, so that the command being reversed can be applied to the
/// commands system again.
///
/// This also means that the CommandHistory needs to be controlled by the application, otherwise
/// we would need to implement it for every command, which would be a lot of boilerplate.
pub trait ReversibleCommand: Send + Sync + 'static {
    fn undo(&self, commands: &mut Commands);
    fn redo(&self, commands: &mut Commands);
}

/// A simpler type for the Boxed version of the ReversibleCommand trait.
pub type CommandType = Box<dyn ReversibleCommand>;

/// A resource that holds the history of commands applied, used to perform undo/redo actions.
/// It holds the performed and the reversed commands, so that the application can keep track of the
/// changes made and revert them if necessary.
#[derive(Default, Resource)]
pub struct CommandHistory {
    pub performed_commands: Vec<CommandType>,
    pub reversed_commands: Vec<CommandType>,
}

/// An alternative version of DrawingBundle that holds the drawing information in a more
/// flexible way, so that it can be used in the commands and systems that manipulate the
/// drawing entities. Differently from DrawingBundle, this type is not a bundle, so it doesn't
/// get added to the ECS directly.
///
/// It has an optional AppearanceDescriptor, so that it can be used to represent the old state
/// of the entity, when reverting a command. An empty appearance means that the entity is freshly
/// created or should be deleted, depending on the context used.
pub type DrawingInfo = (
    TilePosition,
    Layer,
    Visibility,
    Option<AppearanceDescriptor>,
);

impl From<DrawingBundle> for DrawingInfo {
    fn from(bundle: DrawingBundle) -> Self {
        (
            bundle.tile_pos,
            bundle.layer,
            bundle.visibility,
            Some(bundle.appearance),
        )
    }
}
