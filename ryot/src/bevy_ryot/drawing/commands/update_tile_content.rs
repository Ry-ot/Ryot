use crate::bevy_ryot::drawing::{commands::*, *};
use bevy::ecs::system::EntityCommand;

#[derive(Debug, Copy, Clone)]
pub struct UpdateTileContent(pub Option<DrawingBundle>, pub Option<DrawingBundle>);
impl EntityCommand for UpdateTileContent {
    fn apply(self, id: Entity, world: &mut World) {
        let UpdateTileContent(bundle, old_bundle) = &self;

        let Some(new_bundle) = bundle else {
            if let Some(mut visibility) = world.get_mut::<Visibility>(id) {
                *visibility = Visibility::Hidden
            }

            return;
        };

        if old_bundle.is_some() {
            let Some(mut descriptor) = world.get_mut::<AppearanceDescriptor>(id) else {
                return;
            };

            if *descriptor != new_bundle.appearance {
                *descriptor = new_bundle.appearance;
            }

            if let Some(mut visibility) = world.get_mut::<Visibility>(id) {
                *visibility = new_bundle.visibility
            }

            return;
        };

        world.entity_mut(id).insert(*new_bundle);
        let mut map_tiles = world.resource_mut::<MapTiles>();
        let content = map_tiles.entry(new_bundle.tile_pos).or_default();
        content.insert(new_bundle.layer, id);
    }
}

impl ReversibleCommand for UpdateTileContent {
    fn undo(&self, commands: &mut Commands, entity: Option<Entity>) {
        if let Some(entity) = entity {
            commands.add(UpdateTileContent(self.1, self.0).with_entity(entity));
        }
    }
    fn redo(&self, commands: &mut Commands, entity: Option<Entity>) {
        if let Some(entity) = entity {
            commands.add(self.with_entity(entity));
        }
    }
}
