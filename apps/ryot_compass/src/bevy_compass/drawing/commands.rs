use bevy::ecs::system::Command;
use bevy::prelude::Resource;
use ryot::prelude::UpdateTileContent;

#[derive(Clone, Debug)]
pub enum DrawingCommand {
    Update(UpdateTileContent),
}

impl DrawingCommand {
    pub fn command(&self) -> impl Command {
        match self {
            Self::Update(command) => command.clone(),
        }
    }

    pub fn reverted(&self) -> impl Command {
        match self {
            Self::Update(command) => command.revert(),
        }
    }
}

impl From<UpdateTileContent> for DrawingCommand {
    fn from(command: UpdateTileContent) -> Self {
        Self::Update(command)
    }
}

/// A resource that holds the history of commands applied, used to perform undo/redo actions.
/// It holds the performed and the reversed commands, so that the application can keep track of the
/// changes made and revert them if necessary.
#[derive(Default, Resource)]
pub struct CommandHistory {
    pub performed_commands: Vec<DrawingCommand>,
    pub reversed_commands: Vec<DrawingCommand>,
}
