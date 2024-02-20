use crate::bevy_ryot::drawing::*;
use bevy::ecs::system::Command;

/// The main command for manipulating the content of a tile. It is used to update the content of a
/// tile, by adding, removing or updating the entities that are drawn on the tile. It is also used
/// to undo and redo the changes made to the tile content.
///
/// The command holds two vectors of DrawingInfo, one for the new content and another for the old
/// content. The new content is the content that will be applied to the tile, while the old content
/// is the content that will be reverted to, when/if the command is undone.
#[derive(Debug, Clone)]
pub struct UpdateTileContent(pub Vec<DrawingInfo>, Vec<DrawingInfo>);

impl UpdateTileContent {
    /// Constructor that guarantees that the new and old content have the same length.
    pub fn new(new: Vec<DrawingInfo>, old: Vec<DrawingInfo>) -> Self {
        if new.len() != old.len() {
            panic!("The new and old content must have the same length");
        }

        Self(new, old)
    }

    /// Constructor that inits the command from a clean state, when only the new content is known.
    pub fn for_new_bundle(bundles: Vec<DrawingBundle>) -> Self {
        Self::new(
            bundles
                .iter()
                .copied()
                .map(|bundle| bundle.into())
                .collect::<Vec<DrawingInfo>>(),
            bundles
                .into_iter()
                .map(|bundle| (bundle.tile_pos, bundle.layer, bundle.visibility, None))
                .collect(),
        )
    }

    /// Reverts the command, by swapping the new and old content. Useful for undoing the command.
    pub fn revert(&self) -> Self {
        Self::new(self.1.clone(), self.0.clone())
    }
}

impl From<UpdateTileContent> for CommandType {
    fn from(command: UpdateTileContent) -> Self {
        Box::new(command)
    }
}

impl Command for UpdateTileContent {
    fn apply(self, world: &mut World) {
        let (new, old) = (self.0, self.1);

        for (index, info) in new.iter().enumerate() {
            if info.3.is_none() && old[index].3.is_none() {
                continue;
            }

            update(world, *info, old[index], CommandState::default());
        }
    }
}

impl ReversibleCommand for UpdateTileContent {
    fn undo(&self, commands: &mut Commands) {
        commands.add(self.revert());
    }

    fn redo(&self, commands: &mut Commands) {
        commands.add(self.clone());
    }
}
