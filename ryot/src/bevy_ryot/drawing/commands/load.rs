use crate::bevy_ryot::drawing::{CommandState, DrawingInfo};
use crate::prelude::drawing::{update, DrawingBundle};
use bevy::ecs::system::Command;
use bevy::prelude::World;

/// This is a command that is used to load the content of a tile from a persisted state. It is used
/// to load the content of a tile from a database, for example, and apply it to the tile. The command
/// holds a vector of DrawingInfo, which is used to create the entities that are drawn on the tile.
///
/// This action is not reversible by design and should be used with caution, since it still apply the
/// same principles of update, if we don't ignore the persistence systems for those entities we can
/// create some loops of persistence (where a loaded entity is persisted again).
#[derive(Debug, Clone)]
pub struct LoadTileContent(pub Vec<DrawingInfo>);

impl LoadTileContent {
    pub fn from_bundles(new: Vec<DrawingBundle>) -> Self {
        Self(new.into_iter().map(Into::into).collect())
    }
}

impl From<LoadTileContent> for CommandState {
    fn from(_: LoadTileContent) -> Self {
        CommandState::default().persist()
    }
}

impl Command for LoadTileContent {
    fn apply(self, world: &mut World) {
        for info in self.0.iter() {
            update(
                world,
                *info,
                (info.0, info.1, info.2, None),
                self.clone().into(),
            );
        }
    }
}
