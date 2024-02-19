use crate::bevy_ryot::drawing::{CommandState, DrawingInfo, UpdateComponent};
use crate::prelude::drawing::{get_or_create_entity_for_info, DrawingBundle};
use bevy::ecs::system::Command;
use bevy::prelude::World;

#[derive(Debug, Clone)]
pub struct LoadTileContent(pub Vec<DrawingInfo>);

impl LoadTileContent {
    pub fn from_bundles(new: Vec<DrawingBundle>) -> Self {
        Self(new.into_iter().map(Into::into).collect())
    }
}

impl Command for LoadTileContent {
    fn apply(self, world: &mut World) {
        for info in self.0.iter() {
            let id = get_or_create_entity_for_info(world, info);

            world.entity_mut(id).insert(UpdateComponent {
                new: *info,
                old: (info.0, info.1, info.2, None),
                state: CommandState::from_load(),
            });
        }
    }
}
