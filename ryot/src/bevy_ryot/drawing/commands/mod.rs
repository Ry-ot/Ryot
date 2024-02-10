use crate::bevy_ryot::drawing::Layer;
use crate::position::TilePosition;
use bevy::prelude::*;

mod update_tile_content;
pub use update_tile_content::*;

/// A trait that represents a reversible command, that can be undone and redone.
/// Due to limitations from the bevy commands, the undo and redo methods needs to be
/// implemented in each command. We cannot make a generic implementation and simplify
/// this interface to only return the reversed version of the command.
///
/// Every command added to the command system in bevy needs to have known size and be a
/// concrete type, so we cannot simply use a trait object to dispatch the command.
///
/// That's why we pass Commands and Entity here, so that the command being reversed can
/// be applied to the commands system again.
///
/// This also means that the CommandHistory needs to be controlled by the application, otherwise
/// we would need to implement it for every command, which would be a lot of boilerplate.
pub trait ReversibleCommand: Send + Sync + 'static {
    fn undo(&self, commands: &mut Commands, entity: Option<Entity>);
    fn redo(&self, commands: &mut Commands, entity: Option<Entity>);
}

/// A resource that holds the history of commands applied, used to perform undo/redo actions.
/// It holds the performed and the reversed commands, so that the application can keep track of the
/// changes made and revert them if necessary.
///
/// We store the command in a Vec<ReverseCommandRecord>, which holds the reference to the layer and
/// position where the command was applied, and the command itself.
#[derive(Default, Resource)]
pub struct CommandHistory {
    pub performed_commands: Vec<CommandType>,
    pub reversed_commands: Vec<CommandType>,
}

#[derive(Debug, Copy, Clone, Reflect, Component)]
pub struct Deleted;
#[derive(Debug, Copy, Clone, Reflect, Component)]
pub struct InTheScreen;

pub enum CommandType {
    Batch(CommandBatchSize),
    TileCommand(TileCommandRecord),
}

impl From<TileCommandRecord> for CommandType {
    fn from(record: TileCommandRecord) -> Self {
        Self::TileCommand(record)
    }
}

impl From<CommandBatchSize> for CommandType {
    fn from(size: CommandBatchSize) -> Self {
        Self::Batch(size)
    }
}

/// A record that holds the layer, position and command that was applied to a tile.
pub struct TileCommandRecord {
    pub layer: Layer,
    pub tile_pos: TilePosition,
    pub command: Box<dyn ReversibleCommand>,
}

impl TileCommandRecord {
    pub fn new(layer: Layer, tile_pos: TilePosition, command: Box<dyn ReversibleCommand>) -> Self {
        Self {
            layer,
            tile_pos,
            command,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Deref, DerefMut)]
pub struct CommandBatchSize(pub usize);
