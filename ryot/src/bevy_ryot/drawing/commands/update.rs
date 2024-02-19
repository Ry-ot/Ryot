use crate::bevy_ryot::drawing::{CommandType, Deletion, DrawingBundle, ReversibleCommand};
use crate::bevy_ryot::map::MapTiles;
use crate::bevy_ryot::AppearanceDescriptor;
use crate::prelude::drawing::TileComponent;
use crate::prelude::TilePosition;
use crate::Layer;
use bevy::ecs::system::Command;
use bevy::prelude::{Commands, Visibility, World};

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

#[derive(Debug, Clone)]
pub struct UpdateTileContent(pub Vec<DrawingInfo>, Vec<DrawingInfo>);

impl UpdateTileContent {
    pub fn new(new: Vec<DrawingInfo>, old: Vec<DrawingInfo>) -> Self {
        if new.len() != old.len() {
            panic!("The new and old content must have the same length");
        }

        Self(new, old)
    }

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
            let (pos, layer, visibility, appearance) = info;

            let mut map_tiles = world.resource_mut::<MapTiles>();
            let entity = map_tiles.entry(*pos).or_default().get(layer).copied();

            let id = match entity {
                Some(entity) => entity,
                None => world.spawn_empty().id(),
            };

            if entity.is_none() {
                world
                    .resource_mut::<MapTiles>()
                    .entry(*pos)
                    .or_default()
                    .insert(*layer, id);
            }

            match (appearance, old[index].2) {
                (None, _) => {
                    world.entity_mut(id).insert(Deletion::default());
                    continue;
                }
                (Some(bundle), _) => {
                    world.entity_mut(id).insert((
                        *pos,
                        *layer,
                        *bundle,
                        *visibility,
                        TileComponent,
                    ));
                    world.entity_mut(id).remove::<Deletion>();
                }
            }
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
